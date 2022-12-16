use bigdecimal::BigDecimal;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestProduct {
    pub product_name: String,
    pub product_description: String,
    pub image_url: String,
    pub stock: i32,
    pub price: BigDecimal,
    pub category_id: uuid::Uuid,
}
