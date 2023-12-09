use actix_web::{get, HttpResponse, Error, web, error};

use crate::{repository::database::DbPool, services::traffic_ticket::get_traffic_tickets};


#[get("/")]
pub async fn traffic_tickets(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let traffic_tickets = web::block(move || {
        let conn = &mut pool.get()?;
        get_traffic_tickets(conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(traffic_tickets?))
}