use actix_web::HttpRequest;
use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: String,
    role: String,
    exp: usize,
}

const BEARER: &str = "Bearer ";

pub fn create_jwt(user_id: String, role: String) -> String {
    let key = b"secret";

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(5 * 60 * 60)) // 5 hours
        .expect("Fail to add 5 hours")
        .timestamp();

    let claims = Claims {
        user_id,
        role,
        exp: expiration as usize,
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(),
    }
}

pub fn parse_jwt(req: &HttpRequest) -> Result<(String, String), Error> {
    let header = match req.headers().get("Authorization") {
        Some(v) => v,
        None => return Err(ErrorKind::MissingRequiredClaim("".to_string()).into()),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_e) => return Err(ErrorKind::MissingRequiredClaim("".to_string()).into()),
    };

    if !auth_header.starts_with(BEARER) {
        return Err(ErrorKind::InvalidKeyFormat.into());
    }

    let key = b"secret";

    let validation = Validation::new(Algorithm::HS256);

    let token = auth_header.trim_start_matches(BEARER).to_owned();

    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => return Err(ErrorKind::InvalidToken.into()), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    Ok((token_data.claims.user_id, token_data.claims.role))
}
