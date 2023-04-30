use std::net::TcpListener;
use actix_web::{dev::Server, HttpServer, web, App};
use sqlx::PgPool;
use crate::routes::{health_check, subscription}; 

pub fn run(listener :TcpListener, db_pool:PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscription))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
        .run();
    Ok(server)
}
