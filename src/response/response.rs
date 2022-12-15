#[derive(serde::Serialize, serde::Deserialize)]
pub struct ServiceResponse<T> {
    pub data: T,
    pub success: bool,
    pub message: String,
}

impl<T> ServiceResponse<T> {
    pub fn new(data: T) -> Self {
        ServiceResponse {
            data: data,
            success: false,
            message: "".to_string(),
        }
    }
}
