use uuid::Uuid;
use validator::validate_email;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NewUser {
    pub id: Uuid,
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
    pub fn parse(s: String) -> Result<UserEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid user email.", s))
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserString(String);

impl UserString {
    pub fn parse(s: String) -> Result<UserString, String> {
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
