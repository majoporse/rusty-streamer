use std::future::{ready, Ready};

use actix_web::cookie::{time, Cookie, SameSite};
use actix_web::error::ErrorBadRequest;
use actix_web::{dev::Payload, error::InternalError, web, HttpRequest, HttpResponse};
use actix_web::{post, FromRequest};
use chrono::{Duration, Utc};
use jwt_compact::alg::Hs256Key;
use jwt_compact::{alg::Hs256, AlgorithmExt, Token, UntrustedToken};
use jwt_compact::{Claims, Header, TimeOptions};
use serde::{Deserialize, Serialize};
use users_client::apis::users_api;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::controllers::users::client_config;
use crate::controllers::users::error::handle_client_error_actix;

static TAG: &str = "Auth";

static REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";
static ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";

pub fn get_key() -> Hs256Key {
    Hs256Key::new(b"super_secret_key_donut_steel")
}

pub fn get_access_claims(custom: CustomClaims) -> Claims<CustomClaims> {
    let time_options = TimeOptions::default();

    Claims::new(custom)
        .set_duration_and_issuance(&time_options, Duration::seconds(20))
        .set_not_before(Utc::now() - Duration::hours(1))
}

pub fn get_refresh_claims(custom: CustomClaims) -> Claims<CustomClaims> {
    let time_options = TimeOptions::default();
    Claims::new(custom)
        .set_duration_and_issuance(&time_options, Duration::days(7))
        .set_not_before(Utc::now() - Duration::hours(1))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomClaims {
    pub user_id: Uuid,
}

impl FromRequest for CustomClaims {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header_str| {
                if header_str.starts_with("Bearer ") {
                    Some(header_str[7..].to_string())
                } else {
                    None
                }
            });

        log::info!("Extracting token from header: {:?}", access_token);

        match access_token {
            Some(token_string) => {
                let time_options = TimeOptions::default();

                let key = get_key();

                let token = UntrustedToken::new(&token_string).unwrap();

                assert_eq!(token.header().key_id, Some("my-key".to_owned()));

                let token: Token<CustomClaims> = Hs256.validator(&key).validate(&token).unwrap();

                token
                    .claims()
                    .validate_expiration(&time_options)
                    .unwrap()
                    .validate_maturity(&time_options)
                    .unwrap();

                let user = token.claims().custom.clone();

                ready(Ok(user))
            }

            None => ready(Err(InternalError::from_response(
                String::from("No token provided"),
                HttpResponse::Unauthorized().finish(),
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login user and set auth cookies", body = LoginResponse),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/login")]
async fn login(body: web::Json<LoginRequest>) -> Result<HttpResponse, actix_web::Error> {
    let config = client_config();
    let user = users_api::get_user_by_email(&config, &body.email)
        .await
        .map_err(|e| handle_client_error_actix(e, &format!("Fetching user {}", body.email)))?;

    let key = get_key();
    let header = Header::empty().with_key_id("my-key");

    let custom = CustomClaims { user_id: user.id };

    let access_claims = get_access_claims(custom.clone());
    let refresh_claims = get_refresh_claims(custom);

    let token_string = Hs256.token(&header, &access_claims, &key).unwrap();

    log::debug!("token: {token_string}");

    let refresh_string = Hs256.token(&header, &refresh_claims, &key).unwrap();

    let refresh_cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_string.clone())
        .path("/refresh")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(LoginResponse {
            user_id: user.id,
            user_name: user.username,
            user_email: user.email,
            access_token: token_string,
            refresh_token: refresh_string,
        }))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RefreshResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Refresh tokens", body = RefreshResponse),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/refresh")]
pub async fn refresh(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let refresh_token = req
        .cookie(REFRESH_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    // validation
    let refresh_token = match refresh_token {
        Some(token) => token,
        None => return Err(ErrorBadRequest("No refresh token provided")),
    };
    let time_options = TimeOptions::default();
    let token = UntrustedToken::new(&refresh_token).unwrap();

    assert_eq!(token.header().key_id, Some("my-key".to_owned()));

    let key = get_key();
    let refresh_token: Token<CustomClaims> = Hs256.validator(&key).validate(&token).unwrap();

    refresh_token
        .claims()
        .validate_expiration(&time_options)
        .unwrap()
        .validate_maturity(&time_options)
        .unwrap();

    let user = refresh_token.claims().custom.clone();

    let config = client_config();

    let parsed_id = match Uuid::parse_str(&user.user_id.to_string()) {
        Ok(id) => id,
        Err(_) => return Err(ErrorBadRequest("Invalid UUID format")),
    };

    let result = users_api::get_user_by_id(&config, &*parsed_id.to_string()).await;
    if let Err(err) = result {
        return Err(handle_client_error_actix(
            err,
            &format!("Fetching user {}", user.user_id),
        ));
    }
    let user = result.unwrap();

    // generation
    let key = get_key();
    let header = Header::empty().with_key_id("my-key");

    let custom = CustomClaims { user_id: user.id };

    let refresh_claims = get_refresh_claims(custom.clone());
    let access_claims = get_access_claims(custom.clone());

    let access_string = Hs256.token(&header, &access_claims, &key).unwrap();
    let refresh_string = Hs256.token(&header, &refresh_claims, &key).unwrap();

    log::debug!("token: {refresh_string}");
    log::debug!("token: {access_string}");

    let cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_string.clone())
        .path("/refresh")
        .http_only(true)
        .same_site(SameSite::Lax)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(RefreshResponse {
        user_id: user.id,
        user_name: user.username,
        user_email: user.email,
        access_token: access_string,
        refresh_token: refresh_string,
    }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Logout user by clearing auth cookies"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/logout")]
pub async fn logout() -> Result<HttpResponse, actix_web::Error> {
    let expired_cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, "")
        .path("/refresh")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .same_site(SameSite::Lax)
        .finish();

    Ok(HttpResponse::Ok().cookie(expired_cookie).body("Logged out"))
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(login, refresh, logout))]
pub struct ApiDoc;

pub fn auth_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(refresh);
    cfg.service(login);
    cfg.service(logout);
}
