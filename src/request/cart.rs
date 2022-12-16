use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestPostCart {
    pub product_id: Uuid,
    pub quantity: i32,
}
