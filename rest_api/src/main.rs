use axum::{routing::{get, post}, Router, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn get_hello() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello".to_string(),
    })
}

#[derive(Deserialize)]
struct ContentRequest {
    content: String,
}

#[derive(Serialize)]
struct ContentResponse {
    received: String,
}

async fn post_content(Json(payload): Json<ContentRequest>) -> Result<Json<ContentResponse>, StatusCode> {
    if payload.content.len() > 256 {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(ContentResponse {
        received: payload.content,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sample/getHello", get(get_hello))
        .route("/sample/postContent", post(post_content));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid address");

    println!("Listening on {}", addr);
    if let Err(e) = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_hello() {
        let app = Router::new().route("/sample/getHello", get(get_hello));

        let response = app
            .oneshot(Request::builder().uri("/sample/getHello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post_content() {
        let app = Router::new().route("/sample/postContent", post(post_content));

        let payload = serde_json::json!({"content": "data"});
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/sample/postContent")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
