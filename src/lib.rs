use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, dev::Server};
use std::net::TcpListener;
// use serde;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        .route("/healthz", web::get().to(health_check))
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
