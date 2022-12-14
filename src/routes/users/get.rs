use crate::{
    authorization::parse_jwt,
    domain::{UserEmail, UserString},
    response::ServiceResponse,
    types::user::User,
};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;

pub async fn get_all_users(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, GetAllUsersError> {
    let mut res = ServiceResponse::new(Vec::<User>::new());

    let (_user_id, role) = parse_jwt(&req)?;

    if role != "Admin" {
        return Err(GetAllUsersError::AuthorizationError(
            jsonwebtoken::errors::ErrorKind::InvalidToken.into(),
        ));
    }

    let users = sqlx::query!(r#"SELECT * FROM users"#)
        .fetch_all(pool.get_ref())
        .await
        .map_err(GetAllUsersError::SqlxError)?;

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

    res.data = vec_user;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum GetAllUsersError {
    SqlxError(sqlx::Error),
    AuthorizationError(jsonwebtoken::errors::Error),
}

impl ResponseError for GetAllUsersError {}

impl std::fmt::Display for GetAllUsersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get users.")
    }
}

impl From<jsonwebtoken::errors::Error> for GetAllUsersError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::AuthorizationError(e)
    }
}

pub async fn get_user_email(
    pool: &web::Data<PgPool>,
    user_id: &str,
) -> Result<String, sqlx::Error> {
    let user_email = sqlx::query!(r#"SELECT email FROM users WHERE user_id = $1"#, user_id)
        .fetch_one(pool.get_ref())
        .await?
        .email;

    Ok(user_email)
}

pub async fn get_user_by_email(
    pool: &web::Data<PgPool>,
    email: String,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query!(r#"SELECT * FROM users WHERE email = $1"#, email)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(User {
        user_id: user.user_id,
        email: UserEmail::from(user.email),
        password: UserString::from(user.password),
        first_name: UserString::from(user.first_name),
        last_name: UserString::from(user.last_name),
        city: UserString::from(user.city),
        country: UserString::from(user.country),
        company_name: UserString::from(user.company_name),
        role: user.role,
    })
}
