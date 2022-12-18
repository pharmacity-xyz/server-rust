use crate::domain::{UserEmail, UserString};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: UserEmail,
    pub password: UserString,
    pub first_name: UserString,
    pub last_name: UserString,
    pub city: UserString,
    pub country: UserString,
    pub company_name: UserString,
    pub role: String,
}
