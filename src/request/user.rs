use crate::{
    domain::{UserEmail, UserString},
    routes::users::{PostUserError, UpdateUserError},
    types::user::User,
};
use actix_web::web;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostUser {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub country: String,
    pub company_name: String,
}

impl TryFrom<web::Json<PostUser>> for User {
    type Error = PostUserError;

    fn try_from(value: web::Json<PostUser>) -> Result<Self, Self::Error> {
        if value.password != value.confirm_password {
            return Err(PostUserError::ValidationError(
                "Password and Confirm Password does not match".to_string(),
            ));
        }

        let new_user = User {
            user_id: "".to_string(),
            email: UserEmail::parse_with_validation(value.email.clone())?,
            password: UserString::parse_with_validation(value.password.clone())?,
            first_name: UserString::parse_with_validation(value.first_name.clone())?,
            last_name: UserString::parse_with_validation(value.last_name.clone())?,
            city: UserString::parse_with_validation(value.city.clone())?,
            country: UserString::parse_with_validation(value.country.clone())?,
            company_name: UserString::parse_with_validation(value.company_name.clone())?,
            role: "User".to_string(),
        };

        Ok(new_user)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub country: String,
    pub company_name: String,
}

impl TryFrom<web::Json<UpdateUser>> for User {
    type Error = UpdateUserError;

    fn try_from(value: web::Json<UpdateUser>) -> Result<Self, Self::Error> {
        let new_user = User {
            user_id: value.user_id.to_string(),
            email: UserEmail::parse_with_validation(value.email.clone())?,
            password: UserString::from("".to_string()),
            first_name: UserString::parse_with_validation(value.first_name.clone())?,
            last_name: UserString::parse_with_validation(value.last_name.clone())?,
            city: UserString::parse_with_validation(value.city.clone())?,
            country: UserString::parse_with_validation(value.country.clone())?,
            company_name: UserString::parse_with_validation(value.company_name.clone())?,
            role: "User".to_string(),
        };

        Ok(new_user)
    }
}
