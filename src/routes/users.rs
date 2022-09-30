use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
    address: String,
    phonenumber: String,
    email: String,
    password: String,
}

pub async fn post_user(user: web::Json<User>, connection: web::Data<PgConnection>) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, address, phonenumber, email, password)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        user.name,
        user.address,
        user.phonenumber,
        user.email,
        user.password
    )
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
