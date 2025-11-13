use actix_web::HttpResponse;
use std::any::Any;
use std::fmt::Debug;

/// Inspect and log database errors. Returns an appropriate `HttpResponse`:
/// - `404 NotFound` when the underlying error is `diesel::result::Error::NotFound`,
/// - `500 InternalServerError` otherwise (and logs the error with context).
pub fn handle_db_error<E>(err: &E, context: &str) -> HttpResponse
where
    E: std::error::Error + Debug + 'static,
{
    // If the underlying error is a diesel::result::Error, inspect it
    if let Some(db_err) = (err as &dyn Any).downcast_ref::<diesel::result::Error>() {
        use diesel::result::Error as DieselError;
        if matches!(db_err, DieselError::NotFound) {
            // NotFound is not an internal error — return 404 without logging
            return HttpResponse::NotFound().finish();
        }
    }

    // Fallback: log the error with context and return 500
    log::error!("{}: {:#?}", context, err);
    HttpResponse::InternalServerError().finish()
}
