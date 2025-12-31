use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, http::header::ContentType, web,
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/healthcheck", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
