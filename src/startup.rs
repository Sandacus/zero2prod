//! src/startup.rs

use actix_web::{App, HttpServer, dev::Server, web};
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub const HEALTH_CHECK: &str = "/health_check";
pub const SUBSCRIBE: &str = "/subscriptions";

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(HEALTH_CHECK, web::get().to(health_check))
            .route(SUBSCRIBE, web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
