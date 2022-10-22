use crate::auth::{validate_credentials, AuthError, Credentials};
use crate::util::error_chain_fmt;
use actix_web::error::InternalError;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use actix_web_flash_messages::FlashMessage;
use hmac::{Hmac, Mac};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

#[tracing::instrument(skip(credential, pool), fields(email=tracing::field::Empty, user_id=tracing::field::Empty))]
pub async fn login(
    credential: web::Json<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        email: credential.email.clone(),
        password: credential.password.clone(),
    };
    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            Ok(HttpResponse::Ok().finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            FlashMessage::error(e.to_string()).send();
            let response = HttpResponse::SeeOther().finish();
            Err(InternalError::from_response(e, response))
        }
    }
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
