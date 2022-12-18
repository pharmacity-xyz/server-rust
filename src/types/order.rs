use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CartItemWithProduct {
    pub product_id: String,
    pub product_name: String,
    pub image_url: String,
    pub price: BigDecimal,
    pub quantity: i32,
}

pub struct Order {
    pub order_id: Uuid,
    pub user_id: String,
    pub total_price: BigDecimal,
    pub ship_address: String,
    pub order_date: DateTime<Utc>,
    pub shipped_date: DateTime<Utc>,
}
