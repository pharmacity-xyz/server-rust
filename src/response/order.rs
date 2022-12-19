use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct OrderDetailsResponse {
    pub order_date: DateTime<Utc>,
    pub total_price: BigDecimal,
    pub order_details_product: Vec<OrderDetailsProduct>,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct OrderDetailsProduct {
    pub product_id: String,
    pub product_name: String,
    pub image_url: String,
    pub quantity: i32,
    pub total_price: BigDecimal,
}
