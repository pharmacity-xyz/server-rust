use crate::{
    domain::{UserEmail, UserString},
    types::user::User,
};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

pub async fn get_all_users(pool: web::Data<PgPool>) -> Result<HttpResponse, GetAllUsersError> {
    let users = sqlx::query!(r#"SELECT * FROM users"#)
        .fetch_all(pool.get_ref())
        .await
        .map_err(GetAllUsersError)?;

    let mut vec_user = vec![];
    for user in users.into_iter() {
        let temp_user = User {
            user_id: user.user_id,
            email: UserEmail::from(user.email),
            password: UserString::from(user.password),
            first_name: UserString::from(user.first_name),
            last_name: UserString::from(user.last_name),
            city: UserString::from(user.city),
            country: UserString::from(user.country),
            company_name: UserString::from(user.company_name),
            role: user.role,
        };

        vec_user.push(temp_user);
    }

    Ok(HttpResponse::Ok().json(vec_user))
}

#[derive(Debug)]
pub struct GetAllUsersError(sqlx::Error);

impl ResponseError for GetAllUsersError {}

impl std::fmt::Display for GetAllUsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get users.")
    }
}
