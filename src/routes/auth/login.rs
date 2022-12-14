use crate::cookie::set_cookie;
use crate::response::ServiceResponse;
use crate::util::error_chain_fmt;
use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    authorization::create_jwt,
};
use actix_web::error::InternalError;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use actix_web_flash_messages::FlashMessage;
use secrecy::Secret;
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
    let mut res = ServiceResponse::new(String::default());
    let credentials = Credentials {
        email: credential.email.clone(),
        password: credential.password.clone(),
    };
    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    match validate_credentials(credentials, &pool).await {
        Ok((user_id, user_role)) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));

            let token = create_jwt(user_id, user_role);

            let cookie = set_cookie(token.as_str());

            res.data = token;
            res.success = true;

            Ok(HttpResponse::Ok()
                .insert_header(("Set-Cookie", cookie.to_string()))
                .json(res))
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
