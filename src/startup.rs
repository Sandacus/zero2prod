//! src/startup.rs

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub const HEALTH_CHECK: &str = "/health_check";
pub const SUBSCRIBE: &str = "/subscriptions";

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route(HEALTH_CHECK, web::get().to(health_check))
            .route(SUBSCRIBE, web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
