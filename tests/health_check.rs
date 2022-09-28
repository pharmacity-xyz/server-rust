use pharmacity::main;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let address = "127.0.0.1:0";
    let server = pharmacity::run(address).expect("Failed to bind address"); 
    let _ = tokio::spawn(server);
}
