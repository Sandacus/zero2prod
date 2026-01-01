//! src/lib.rs

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

pub const HEALTH_CHECK: &str = "/health_check";

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route(HEALTH_CHECK, web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
