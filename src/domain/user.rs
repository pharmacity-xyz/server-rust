use std::fmt::Display;

use uuid::Uuid;
use validator::validate_email;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NewUser {
    pub user_id: Uuid,
    pub email: UserEmail,
    pub password: UserString,
    pub first_name: UserString,
    pub last_name: UserString,
    pub city: UserString,
    pub country: UserString,
    pub company_name: UserString,
    pub role: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse_with_validation(s: String) -> Result<UserEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid user email.", s))
        }
    }

    pub fn inner(&self) -> String {
        self.0.to_string()
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserEmail {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserString(String);

impl Display for UserString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserString {
    pub fn parse_with_validation(s: String) -> Result<UserString, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        if is_empty_or_whitespace {
            Err(format!("{} is not a valid user", s))
        } else {
            Ok(Self(s))
        }
    }

    pub fn inner(&self) -> String {
        self.0.to_string()
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UserString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for UserString {
    fn from(s: String) -> Self {
        Self(s)
    }
}
