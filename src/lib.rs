use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, dev::Server};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// #[get("/hello/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            // .service(greet)
        .route("/healthz", web::get().to(health_check))
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        })
        .listen(listener)?
        .run();

    Ok(server)
}
