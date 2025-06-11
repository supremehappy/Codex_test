use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct ContentRequest {
    pub content: String,
}

#[derive(Serialize)]
pub struct ContentResponse {
    pub received: String,
}
