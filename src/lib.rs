use std::net::TcpListener;

use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::dev::Server;
use serde;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.name)
}

async fn subscription(_form:web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener :TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscription))
    })
    .listen(listener)?
        .run();
    Ok(server)
}
