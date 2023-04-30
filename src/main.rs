use std::net::TcpListener;
use email_newsletter::startup::run;
use email_newsletter::configuration::get_config;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config file");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let url = format!("127.0.0.1:{}", config.app_port); 
    let listener = TcpListener::bind(url).expect("Failed to bind the address"); 
    let addr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);
    run(listener, connection_pool)?.await
}
