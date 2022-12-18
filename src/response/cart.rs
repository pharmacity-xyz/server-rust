use bigdecimal::BigDecimal;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CartItemWithProduct {
    pub product_id: String,
    pub product_name: String,
    pub image_url: String,
    pub price: BigDecimal,
    pub quantity: i32,
}
