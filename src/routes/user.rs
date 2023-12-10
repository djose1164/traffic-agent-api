use actix_web::{error, get, post, web, Error, HttpResponse};

use crate::{
    models::user::{UserCredentials, UserPayload},
    repository::database::DbPool,
    services::user::{authenticate_user, create_user, find_by},
};

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    payload: web::Json<UserCredentials>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;
        log::info!("{payload:?}");
        authenticate_user(payload.into_inner(), conn)
    })
    .await?
    .map_err(error::ErrorNotFound);

    Ok(HttpResponse::Ok().json(user?))
}

#[post("/signup")]
pub async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<UserPayload>,
) -> Result<HttpResponse, Error> {
    let new_user = web::block(move || {
        let conn = &mut pool.get()?;
        create_user(payload.into_inner(), conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Created().json(new_user?))
}

#[get("/{user_id}")]
pub async fn read(pool: web::Data<DbPool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;
        find_by(user_id.into_inner(), conn)
    })
    .await?
    .map_err(error::ErrorNotFound);

    Ok(HttpResponse::Ok().json(user?))
}
