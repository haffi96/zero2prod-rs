use reqwest;

#[tokio::test]
async fn health_check_works() {
    //Arrange
    let addr = spawn_app();
    // Bring in reqwest to perform HTTP requests
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthz", &addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    
}

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}