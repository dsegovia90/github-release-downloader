use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ResponseError {
    pub status: String,
    pub message: String,
}
