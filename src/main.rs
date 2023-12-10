use actix_web::{get, middleware, web, App, Responder, HttpResponse, HttpServer};
use actix_files as fs;
mod repository;
mod models;
mod routes;
mod services;

use repository::database::establish_connection;
use routes::{
    user::{create, login},
    traffic_ticket::{traffic_tickets, get_traffic_ticket},
    vehicle::vehicle_index,
    drivers::driver_index
};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Welcome to TrafficAgent API!</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at: http://127.0.0.1:8080");

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                establish_connection().expect("Failed to get pool.")
            ))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(
                web::scope("/users")
                .service(create)
                .service(login)
            )
            .service(
                web::scope("/traffic_tickets")
                    .service(traffic_tickets)
                    .service(get_traffic_ticket)
            )
            .service(
                web::scope("/vehicles")
                    .service(vehicle_index)
            )
            .service(fs::Files::new("/assets", "./assets").show_files_listing())
            .service(
                web::scope("/drivers")
                    .service(driver_index)   
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
