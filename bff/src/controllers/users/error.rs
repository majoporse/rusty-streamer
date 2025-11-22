use actix_web::HttpResponse;
use log::error;

/// Maps the generated client error into an HttpResponse and logs it
pub fn handle_client_error<T>(err: users_client::apis::Error<T>, context: &str) -> HttpResponse
where
    T: std::fmt::Debug,
{
    match err {
        users_client::apis::Error::ResponseError(resp) if resp.status.as_u16() == 404 => {
            error!("{} - Not Found: {:?}", context, resp);
            HttpResponse::NotFound().finish()
        }
        _ => {
            error!("{} - Internal Error: {:?}", context, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn handle_client_error_actix<T>(
    err: users_client::apis::Error<T>,
    context: &str,
) -> actix_web::Error
where
    T: std::fmt::Debug,
{
    match err {
        users_client::apis::Error::ResponseError(resp) if resp.status.as_u16() == 404 => {
            error!("{} - Not Found: {:?}", context, resp);
            actix_web::error::ErrorNotFound("Resource not found")
        }
        _ => {
            error!("{} - Internal Error: {:?}", context, err);
            actix_web::error::ErrorInternalServerError("Internal server error")
        }
    }
}
