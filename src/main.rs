use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod configs;
mod models;
mod routes;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("expense-tracker")
}

#[get("/users/{id}")]
async fn get_user(id: String) -> impl Responder {
    HttpResponse::Ok().body(format!("get user: {}", id))
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("get users")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_users)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
