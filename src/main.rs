use std::net::TcpListener;
use email_newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the address"); 
    let addr = listener.local_addr().unwrap();
    println!("Listening on {}", addr);
    run(listener)?.await
}
