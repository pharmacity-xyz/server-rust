use actix_web::web::Json;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Cart {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

impl From<Json<Cart>> for Cart {
    fn from(value: Json<Cart>) -> Self {
        Cart {
            user_id: value.user_id,
            product_id: value.product_id,
            quantity: value.quantity,
        }
    }
}
