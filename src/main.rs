use std::net::TcpListener;
use email_newsletter::startup::run;
use email_newsletter::configuration::get_config;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read config file");
    let connection_pool = PgPool::connect_lazy(&config.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres.");
    let url = format!("{}:{}",config.application.host, config.application.port); 
    let listener = TcpListener::bind(url).expect("Failed to bind the address"); 
    let addr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);
    run(listener, connection_pool)?.await
}
