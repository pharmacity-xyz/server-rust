use std::time::SystemTime;

use actix_web::HttpRequest;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cookie::get_cookie_value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: Uuid,
    role: String,
    exp: usize,
}

pub fn create_jwt(user_id: Uuid, role: String) -> String {
    let key = b"secret";

    let now = SystemTime::now();

    let add_hours = now
        .checked_add(std::time::Duration::from_secs(5 * 60 * 60)) // 5 hours
        .expect("Fail to add 5 hours");

    let exp_time = add_hours
        .duration_since(std::time::UNIX_EPOCH)
        .expect("")
        .as_secs();

    let claims = Claims {
        user_id,
        role,
        exp: exp_time as usize,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(),
    };

    token
}

pub fn parse_jwt(req: &HttpRequest) -> Result<(Uuid, String), Error> {
    let mut cookie_string: String = String::default();
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        cookie_string = String::from(v.to_str().unwrap());
    }

    let token: String;

    match get_cookie_value("key", cookie_string) {
        Some(t) => token = t,
        None => return Err(ErrorKind::InvalidToken.into()),
    };
    let key = b"secret";

    let validation = Validation::new(Algorithm::HS256);

    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => return Err(ErrorKind::InvalidToken.into()), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    return Ok((token_data.claims.user_id, token_data.claims.role));
}
