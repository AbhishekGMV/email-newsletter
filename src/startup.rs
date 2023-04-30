use std::net::TcpListener;
use actix_web::{dev::Server, HttpServer, web, App};
use crate::routes::{health_check, subscription}; 

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
