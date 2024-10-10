use actix_web::{get, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod routes;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("expense-tracker")
}

#[get("/users")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("get users")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
