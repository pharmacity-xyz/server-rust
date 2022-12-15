use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CartItemWithProduct {
    pub product_id: Uuid,
    pub product_name: String,
    pub image_url: String,
    pub price: BigDecimal,
    pub quantity: i32,
}
