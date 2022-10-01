use crate::helpers::spawn_app;
use std::collections::HashMap;

#[tokio::test]
async fn post_user_returns_a_200_for_valid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let mut map = HashMap::new();
    map.insert("name", "Tom");
    map.insert("address", "Tokyo");
    map.insert("phonenumber", "026122222222");
    map.insert("email", "tokyo@gmail.com");
    map.insert("password", "password");
    let response = client
        .post(&format!("{}/user", &app.address))
        .json(&map)
        .send()
        .await
        .expect("Failed to post user.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT id, name, address, phonenumber, email, password FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved users");

    assert_eq!(saved.name, "Tom");
    assert_eq!(saved.address, "Tokyo");
    assert_eq!(saved.phonenumber, "026122222222");
    assert_eq!(saved.email, "tokyo@gmail.com");
    assert_eq!(saved.password, "password");
}

#[tokio::test]
async fn post_user_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let mut missing_name_map = HashMap::new();
    missing_name_map.insert("address", "Tokyo");
    missing_name_map.insert("phonenumber", "026122222222");
    missing_name_map.insert("email", "tokyo@gmail.com");
    missing_name_map.insert("password", "password");

    let mut missing_address_map = HashMap::new();
    missing_address_map.insert("name", "Tom");
    missing_address_map.insert("phonenumber", "026122222222");
    missing_address_map.insert("email", "tokyo@gmail.com");
    missing_address_map.insert("password", "password");

    let mut missing_phonenumber_map = HashMap::new();
    missing_phonenumber_map.insert("name", "Tom");
    missing_phonenumber_map.insert("address", "Tokyo");
    missing_phonenumber_map.insert("email", "tokyo@gmail.com");
    missing_phonenumber_map.insert("password", "password");

    let mut missing_email_map = HashMap::new();
    missing_email_map.insert("name", "Tom");
    missing_email_map.insert("address", "Tokyo");
    missing_email_map.insert("phonenumber", "026122222222");
    missing_email_map.insert("password", "password");

    let mut missing_password_map = HashMap::new();
    missing_password_map.insert("name", "Tom");
    missing_password_map.insert("address", "Tokyo");
    missing_password_map.insert("phonenumber", "026122222222");
    missing_password_map.insert("email", "tokyo@gmail.com");

    let test_cases = vec![
        (missing_name_map, "missing the name"),
        (missing_address_map, "missing the address"),
        (missing_phonenumber_map, "missing the phonenumber"),
        (missing_email_map, "missing the email"),
        (missing_password_map, "missing the password"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/user", &app.address))
            .json(&invalid_body)
            .send()
            .await
            .expect("Failed to post user");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
