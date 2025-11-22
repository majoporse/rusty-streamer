use std::future::{ready, Ready};

use actix_jwt_auth_middleware::{AuthError, AuthResult, FromRequest, TokenSigner};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{cookie, post, Error, FromRequest};
use actix_web::{
    dev::Payload, error::InternalError, get, http::header, web, HttpRequest, HttpResponse,
};
use chrono::{Duration, Utc};
use jwt_compact::alg::Hs256Key;
use jwt_compact::{alg::Hs256, AlgorithmExt, Token, UntrustedToken};
use jwt_compact::{Claims, Header, TimeOptions};
use serde::{Deserialize, Serialize};
use users_client::apis::users_api;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::controllers::users::error::{handle_client_error, handle_client_error_actix};
use crate::{controllers::users::client_config, models::users::User, UserAuth};

static TAG: &str = "Auth";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomClaims {
    pub user_id: Uuid
}

impl FromRequest for CustomClaims {
    type Error = InternalError<String>;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let access_token = req
            .cookie("access_token")
            .map(|cookie| cookie.value().to_string());
        log::info!("Extracting token from cookie: {:?}", access_token);

        match access_token {
            Some(token_string) => {
                let time_options = TimeOptions::default();
                let key = Hs256Key::new(b"super_secret_key_donut_steel");

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

    let time_options = TimeOptions::default();
    let key = Hs256Key::new(b"super_secret_key_donut_steel");
    let header = Header::empty().with_key_id("my-key");
    let claims = Claims::new(CustomClaims {
        user_id: user.id
    })
    .set_duration_and_issuance(&time_options, Duration::days(7))
    .set_not_before(Utc::now() - Duration::hours(1));

    let token_string = Hs256.token(&header, &claims, &key).unwrap();

    log::debug!("token: {token_string}");

    let cookie = Cookie::build("access_token", token_string.clone())
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", token_string)
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

#[derive(utoipa::OpenApi)]
#[openapi(paths(login))]
pub struct ApiDoc;

pub fn auth_config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(login);
}
