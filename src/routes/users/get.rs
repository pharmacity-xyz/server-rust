use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    id: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
    role: String,
}

pub async fn get_all_users(pool: web::Data<PgPool>) -> Result<HttpResponse, GetAllUsersError> {
    let users = sqlx::query!(r#"SELECT * FROM users"#)
        .fetch_all(pool.get_ref())
        .await
        .map_err(GetAllUsersError)?;

    let mut vec_user = vec![];
    for user in users.into_iter() {
        let temp_user = User {
            id: user.id,
            email: user.email,
            password: user.password,
            first_name: user.first_name,
            last_name: user.last_name,
            city: user.city,
            country: user.country,
            company_name: user.company_name,
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
