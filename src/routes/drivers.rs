use actix_web::{error, get, web, Error, HttpResponse};

use crate::{repository::database::DbPool, services::drivers};

#[get("/{driver_id}")]
pub async fn driver_index(
    pool: web::Data<DbPool>,
    driver_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let driver = web::block(move || {
        let conn = &mut pool.get()?;

        drivers::read(driver_id.into_inner(), conn)
    })
    .await?
    .map_err(error::ErrorNotFound);

    Ok(HttpResponse::Ok().json(driver?))
}
