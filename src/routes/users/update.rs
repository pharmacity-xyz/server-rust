use crate::{request::UpdateUser, response::ServiceResponse, types::user::User};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum UpdateUserError {
    ValidationError(String),
    DatabaseError(sqlx::Error),
}

impl ResponseError for UpdateUserError {}

impl std::fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update user.")
    }
}

impl std::error::Error for UpdateUserError {}

impl From<String> for UpdateUserError {
    fn from(e: String) -> Self {
        Self::ValidationError(e)
    }
}

#[tracing::instrument(name = "Updating user", skip(user, pool))]
pub async fn update_user(
    user: web::Json<UpdateUser>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateUserError> {
    let mut res = ServiceResponse::new(Uuid::default());
    let update_user = match User::try_from(user) {
        Ok(u) => u,
        Err(e) => {
            res.message = e.to_string();
            return Err(UpdateUserError::ValidationError(e.to_string()));
        }
    };

    match update_user_for_db(&update_user, pool).await {
        Ok(_) => {}
        Err(e) => {
            res.message = e.to_string();
            return Err(e);
        }
    }

    Ok(HttpResponse::Ok().json(res))
}

#[tracing::instrument(name = "Update user in db", skip(user))]
async fn update_user_for_db(user: &User, pool: web::Data<PgPool>) -> Result<(), UpdateUserError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET email = $1, first_name = $2, last_name = $3, city = $4, country = $5, company_name = $6
        WHERE user_id = $7
        "#,
        user.email.inner(),
        user.first_name.inner(),
        user.last_name.inner(),
        user.city.inner(),
        user.country.inner(),
        user.company_name.inner(),
        user.user_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateUserError::DatabaseError)?;

    Ok(())
}
