use actix_web::http::header::HeaderMap;
use anyhow::Context;
use argon2::{Algorithm, Argon2, Params, Version, PasswordHasher};
use secrecy::{ExposeSecret, Secret};
use sha3::Digest;
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
}

pub fn basic_authentication(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not valid UTF8 string.")?;

    let base64encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not 'Basic'")?;

    let decoded_bytes = base64::decode_config(base64encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode 'Basic' credentials")?;

    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8")?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let email = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A email must be provided in 'Basic' auth."))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Credentials {
        email,
        password: Secret::new(password),
    })
}

#[tracing::instrument(name = "Validate stored credentials", skip(email, pool))]
async fn get_stored_credentials(
    email: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"SELECT id, password_hash FROM users WHERE email = $1"#,
        email,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials")?
    .map(|row| (row.id, Secret::new(row.password_hash)));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(String::new());

    let hasher = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).expect("Failed to create params"),
    );

    let password_hash = sha3::Sha3_256::digest(credentials.password.expose_secret().as_bytes());
    let password_hash = format!("{:x}", password_hash);

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.email, pool).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    // .fetch_optional(pool)
    // .await?;

    // user_id
    //     .map(|row| row.id)
    //     .ok_or_else(|| anyhow::anyhow!("Invalid error or password"))
    //     .map_err(LoginError::AuthError)
    Ok(uuid::Uuid::new_v4())
}
