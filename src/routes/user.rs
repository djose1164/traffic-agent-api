use actix_web::{error, post, web, Error, HttpResponse};

use crate::{
    models::user::UserPayload,
    repository::database::DbPool,
    services::user::{authenticate_user, create_user},
};

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    payload: web::Json<UserPayload>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let conn = &mut pool.get()?;
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

// #[get("/")]
// pub async fn read() ->Result<HttpResponse, Error> {
//     Ok(HttpResponse::Ok().json({}))
// }
