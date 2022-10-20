use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    pub id: Uuid,
    pub email: UserString,
    pub password: UserString,
    pub first_name: UserString,
    pub last_name: UserString,
    pub city: UserString,
    pub country: UserString,
    pub company_name: UserString,
    pub role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserString(String);

impl UserString {
    pub fn parse(s: String) -> UserString {
        let is_empty_or_whitespace = s.trim().is_empty();

        if is_empty_or_whitespace {
            panic!("{} is not a valid user", s);
        } else {
            Self(s)
        }
    }

    pub fn inner(&self) -> String {
        self.0.to_string()
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }
}
