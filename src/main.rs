use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind the address"); 
    run(listener)?.await
}
