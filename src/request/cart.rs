#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestPostCart {
    pub product_id: String,
    pub quantity: i32,
}
