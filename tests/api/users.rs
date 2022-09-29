use std::collections::HashMap;

use crate::helpers::spawn_app;

#[tokio::test]
async fn post_user_returns_a_200_for_valid() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let mut map = HashMap::new();
    map.insert("name", "Tom");
    map.insert("address", "Tokyo");
    map.insert("phonenumber", "026122222222");
    map.insert("email", "tokyo@gmail.com");
    map.insert("password", "password");
    let response = client
        .post(&format!("{}/user", &app_address))
        .json(&map)
        .send()
        .await
        .expect("Failed to post user.");

    assert_eq!(200, response.status().as_u16());
}
