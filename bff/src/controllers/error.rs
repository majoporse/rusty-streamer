
use actix_web::HttpResponse;
use log::error;
static TAG: &str = "Actors";

/// Maps the generated client error into an HttpResponse and logs it
pub fn handle_client_error<T>(err: movies_client::apis::Error<T>, context: &str) -> HttpResponse
where
    T: std::fmt::Debug,
{
    match err {
        movies_client::apis::Error::ResponseError(resp) if resp.status.as_u16() == 404 => {
            error!("{} - Not Found: {:?}", context, resp);
            HttpResponse::NotFound().finish()
        }
        _ => {
            error!("{} - Internal Error: {:?}", context, err);
            HttpResponse::InternalServerError().finish()
        }
    }
}