use actix_web::{error, get, web, Error, HttpResponse};

use crate::{
    repository::database::DbPool,
    services::traffic_ticket::{get_traffic_tickets, traffic_ticket_by},
};

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

#[get("/{id}")]
pub async fn get_traffic_ticket(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let traffic_ticket = web::block(move || {
        let conn = &mut pool.get()?;
        traffic_ticket_by(id.into_inner(), conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(traffic_ticket?))
}
