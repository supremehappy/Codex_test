use axum::{routing::{get, post}, Router};
use std::{env, net::SocketAddr};

mod models;
mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/sample/getHello", get(controllers::get_hello))
        .route("/sample/postContent", post(controllers::post_content));

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
    use axum::{routing::{get, post}, Router};
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;
    use serde_json;

    #[tokio::test]
    async fn test_get_hello() {
        let app = Router::new().route("/sample/getHello", get(super::controllers::get_hello));

        let response = app
            .oneshot(Request::builder().uri("/sample/getHello").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post_content() {
        let app = Router::new().route("/sample/postContent", post(super::controllers::post_content));

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
