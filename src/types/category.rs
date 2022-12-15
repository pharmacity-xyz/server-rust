#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    pub category_id: uuid::Uuid,
    pub name: String,
}
