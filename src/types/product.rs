use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub product_id: String,
    pub product_name: String,
    pub product_description: String,
    pub image_url: String,
    pub stock: i32,
    pub price: BigDecimal,
    pub category_id: uuid::Uuid,
    pub featured: bool,
}

impl Default for Product {
    fn default() -> Self {
        Self::new()
    }
}

impl Product {
    pub fn new() -> Self {
        Product {
            product_id: String::default(),
            product_name: "".to_string(),
            product_description: "".to_string(),
            image_url: "".to_string(),
            stock: i32::default(),
            price: BigDecimal::default(),
            category_id: Uuid::default(),
            featured: false,
        }
    }
}
