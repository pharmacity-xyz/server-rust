use std::time::SystemTime;

use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    // let exp_time = get_current_timestamp()
    //     .checked_add(chrono::Duration::hours(5))
    //     .expect("Fail to add 5 hours");

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

pub fn parse_jwt(token: String) -> Result<(Uuid, String), Error> {
    let key = b"secret";

    let validation = Validation::new(Algorithm::HS256);

    if validation.validate_exp {
        return Err(ErrorKind::ExpiredSignature.into());
    }

    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    return Ok((token_data.claims.user_id, token_data.claims.role));
}
