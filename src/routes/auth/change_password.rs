use crate::authentication::get_stored_credentials;
use actix_web::{web, HttpResponse};
use actix_web_flash_messages::FlashMessage;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    util::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
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
    let credentials = Credentials {
        email: form.email.clone(),
        password: form.current_password.clone(),
    };
    if let Err(e) = validate_credentials(credentials, &pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                FlashMessage::error("The current password is incorrect.").send();
                Err(e500(e))
            }
            AuthError::UnexpectedError(_) => Err(e500(e)),
        };
    }
    let user_id = sqlx::query!(r#"SELECT id FROM users WHERE email = $1"#, form.email)
        .fetch_optional(&**pool)
        .await;

    // let user_id = match user_id {
    //     Ok(record) => {
    //         if record.is_some() {
    //             record.expect("").id
    //         } else {
    //             return Err(e500("The id does not find"));
    //         }
    //     }
    //     Err(e) => return Err(e500(e));
    // };

    let id = user_id.expect("").expect("").id;

    crate::authentication::change_password(id, form.new_password.clone(), &pool)
        .await
        .map_err(e500)?;
    FlashMessage::error("Your password has been changed.").send();
    Ok(HttpResponse::Ok().finish())
}
