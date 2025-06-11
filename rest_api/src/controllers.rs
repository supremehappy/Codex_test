use axum::{Json, http::StatusCode};

use crate::models::{HelloResponse, ContentRequest, ContentResponse};

pub async fn get_hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello".to_string(),
    })
}

pub async fn post_content(Json(payload): Json<ContentRequest>) -> Result<Json<ContentResponse>, StatusCode> {
    if payload.content.len() > 256 {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(ContentResponse {
        received: payload.content,
    }))
}
