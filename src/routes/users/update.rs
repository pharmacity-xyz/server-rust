use crate::types::user::User;
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(Debug)]
pub enum UpdateUserError {
    DatabaseError(sqlx::Error),
    // StripeUpdateError(StripeError),
}

impl ResponseError for UpdateUserError {}

impl std::fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update user.")
    }
}

#[tracing::instrument(name = "Updating user", skip(user, pool))]
pub async fn update_user(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateUserError> {
    // let updated_customer = update_user_for_stripe(&user).await?;
    update_user_for_db(&user, pool).await?;

    Ok(HttpResponse::Ok().json(""))
}

// #[tracing::instrument(name = "Update user in stripe", skip(user))]
// async fn update_user_for_stripe(user: &web::Json<User>) -> Result<Customer, UpdateUserError> {
//     let secret_key =
//         std::env::var("STRIPE_SECRET_KEY").expect("Can not find stripe secret key in env");
//     let client = Client::new(secret_key);

//     let customers = Customer::list(
//         &client,
//         ListCustomers {
//             email: Some(user.email.as_str()),
//             ..Default::default()
//         },
//     )
//     .await
//     .unwrap();

//     let name = format!("{} {}", user.first_name, user.last_name);

//     let customer = Customer::update(
//         &client,
//         &customers.data[0].id,
//         UpdateCustomer {
//             name: Some(name.as_ref()),
//             email: Some(user.email.as_ref()),
//             description: Some(
//                 "A fake customer that is used to illustrate the examples in async-stripe",
//             ),
//             metadata: Some(
//                 [("async-stripe".to_string(), "true".to_string())]
//                     .iter()
//                     .cloned()
//                     .collect(),
//             ),

//             ..Default::default()
//         },
//     )
//     .await
//     .map_err(UpdateUserError::StripeUpdateError)?;

//     Ok(customer)
// }

#[tracing::instrument(name = "Update user in db", skip(user))]
async fn update_user_for_db(
    user: &web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<(), UpdateUserError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET email = $1, first_name = $2, last_name = $3, city = $4, country = $5, company_name = $6
        WHERE user_id = $7
        "#,
        user.email,
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateUserError::DatabaseError)?;

    Ok(())
}
