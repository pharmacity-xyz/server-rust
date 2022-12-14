use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub product_id: Uuid,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub stock: i32,
    pub price: BigDecimal,
    pub category_id: uuid::Uuid,
    pub featured: bool,
}
