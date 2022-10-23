use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use actix_web_flash_messages::FlashMessage;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(
    form: web::Json<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        FlashMessage::error(
            "You entered two different new password - the field values must match.",
        )
        .send();
    }
    todo!()
}
