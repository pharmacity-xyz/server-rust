use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
    address: String,
    phonenumber: String,
    email: String,
    password: String,
}

pub async fn post_user(_user: web::Json<User>) -> HttpResponse {

    HttpResponse::Ok().finish()
}
