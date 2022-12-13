use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    user_id: Uuid,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
    role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostUser {
    email: String,
    password: String,
    confirm_password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
}

