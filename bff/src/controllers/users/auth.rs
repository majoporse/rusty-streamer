use std::future::{ready, Ready};

use actix_web::cookie::{Cookie, SameSite};
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
        .set_duration_and_issuance(&time_options, Duration::hours(1))
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
            .cookie(ACCESS_TOKEN_COOKIE_NAME)
            .map(|cookie| cookie.value().to_string());
        log::info!("Extracting token from cookie: {:?}", access_token);

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
    pub user_id: Uuid,
    pub password: String,
}

#[utoipa::path(
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login user and set auth cookies"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = TAG
)]
#[post("/login/{user_id}")]
async fn login(body: web::Json<LoginRequest>) -> Result<HttpResponse, actix_web::Error> {
    let config = client_config();
    let user = users_api::get_user_by_id(&config, &body.user_id.to_string())
        .await
        .map_err(|e| handle_client_error_actix(e, &format!("Fetching user {}", body.user_id)))?;

    let key = get_key();
    let header = Header::empty().with_key_id("my-key");

    let custom = CustomClaims {
        user_id: body.user_id,
    };

    let access_claims = get_access_claims(custom.clone());
    let refresh_claims = get_refresh_claims(custom);

    let token_string = Hs256.token(&header, &access_claims, &key).unwrap();

    log::debug!("token: {token_string}");

    let refresh_string = Hs256.token(&header, &refresh_claims, &key).unwrap();

    let cookie = Cookie::build(ACCESS_TOKEN_COOKIE_NAME, token_string)
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    let refresh_cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_string)
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .cookie(refresh_cookie)
        .json(user))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Refresh tokens"),
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

    // generation
    let key = get_key();
    let header = Header::empty().with_key_id("my-key");

    let custom = CustomClaims {
        user_id: user.user_id,
    };

    let refresh_claims = get_refresh_claims(custom.clone());
    let access_claims = get_access_claims(custom.clone());

    let access_string = Hs256.token(&header, &access_claims, &key).unwrap();
    let refresh_string = Hs256.token(&header, &refresh_claims, &key).unwrap();

    log::debug!("token: {refresh_string}");
    log::debug!("token: {access_string}");

    let access_cookie = Cookie::build(ACCESS_TOKEN_COOKIE_NAME, access_string.clone())
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    let refresh_cookie = Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_string.clone())
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .finish())
}

#[derive(utoipa::OpenApi)]
#[openapi(paths(login, refresh))]
pub struct ApiDoc;

pub fn auth_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(refresh);
    cfg.service(login);
}
