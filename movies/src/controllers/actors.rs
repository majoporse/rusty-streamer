use actix_web::{HttpResponse, Responder, get, web};
use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use crate::models::actor::Actor;
use crate::data::actors;

#[utoipa::path(
    responses(
        (status = 200, description = "List all actors", body = [Actor]),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Actors"
)]
#[get("/actors")]
pub async fn get_all_actors(
    pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder
{
    let mut db_conn = pool.get().expect("Couldn't get DB connection from pool");

    match actors::list_actors(&mut db_conn, 100, 0) {
        Ok(actors) => HttpResponse::Ok().json(actors),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn scoped_config(
    cfg: &mut utoipa_actix_web::service_config::ServiceConfig,
) {
    cfg.service(get_all_actors);
}