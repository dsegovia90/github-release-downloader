use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ResponseError {
    pub status: u16,
    pub message: String,
}
