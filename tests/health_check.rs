use email_newsletter::run;


fn spawn_app() {
    let server = run()
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);
}


#[tokio::test]
async fn health_check_will_return_200() {
    spawn_app();
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8000/health").send().await.expect("Failed to send request");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));

}
