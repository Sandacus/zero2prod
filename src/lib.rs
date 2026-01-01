//! src/lib.rs

use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

pub const HEALTH_CHECK: &str = "/health_check";

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route(HEALTH_CHECK, web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
