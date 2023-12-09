use actix_web::{web, Error, HttpResponse, get, error};

use crate::{repository::database::DbPool, services::vehicle::vehicle_by};

#[get("/{license_plate}")]
pub async fn vehicle_index(
    pool: web::Data<DbPool>,
    license_plate: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let fetched = web::block(move || {
        let conn = &mut pool.get()?;
        vehicle_by(&license_plate, conn)
    })
    .await?
    .map_err(error::ErrorNotFound);

    Ok(HttpResponse::Ok().json(fetched?))
}
