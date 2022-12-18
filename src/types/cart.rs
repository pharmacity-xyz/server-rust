use actix_web::web::Json;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Cart {
    pub user_id: String,
    pub product_id: String,
    pub quantity: i32,
}

impl From<Json<Cart>> for Cart {
    fn from(value: Json<Cart>) -> Self {
        Cart {
            user_id: value.user_id.to_string(),
            product_id: value.product_id.to_string(),
            quantity: value.quantity,
        }
    }
}
