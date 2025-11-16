
use log::error;

/// Maps the generated client error into an HttpResponse and logs it
pub fn handle_client_error<T>(err: movies_client::apis::Error<T>, context: &str) -> actix_web::Error
where
    T: std::fmt::Debug,
{
    match err {
        movies_client::apis::Error::ResponseError(resp) if resp.status.as_u16() == 404 => {
            error!("{} - Not Found: {:?}", context, resp);
            actix_web::error::ErrorNotFound("")
        }
        _ => {
            error!("{} - Internal Error: {:?}", context, err);
            actix_web::error::ErrorInternalServerError("")
        }
    }
}