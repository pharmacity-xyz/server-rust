#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceResponse<T> {
    pub data: T,
    pub success: bool,
    pub message: String,
}
