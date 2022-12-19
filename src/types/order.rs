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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OrderOverview {
    pub order_id: Uuid,
    pub order_date: DateTime<Utc>,
    pub total_price: BigDecimal,
    pub product: String,
    pub product_image_url: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct OrderDetail {
    pub order_date: DateTime<Utc>,
    pub order_total_price: BigDecimal,
    pub product_id: String,
    pub product_name: String,
    pub image_url: String,
    pub quantity: i32,
    pub order_item_total_price: BigDecimal,
}
