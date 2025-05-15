use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    Extension,
};
use std::sync::Arc;
use tower::ServiceExt;
use tera::Tera;
use std::collections::HashMap;
use std::sync::RwLock;
use axum::routing::{get, post};

use crate::{
    state::AppState,
    handlers::*,
    models::CommentRequest,
};

// Helper function to create test state
fn create_test_state() -> Arc<AppState> {
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    Arc::new(AppState {
        templates,
        comments: RwLock::new(HashMap::new()),
    })
}

// Helper function to create test router
fn create_test_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(blog_handler))
        .route("/about", get(about_handler))
        .route("/blog/{id}", get(post_handler))
        .route("/blog/tag/{tag}", get(tag_handler))
        .route("/api/posts", get(posts_handler))
        .route("/api/search", get(search_handler))
        .route("/api/time", get(time_handler))
        .route("/api/posts/{id}/comments", get(get_comments_handler))
        .route("/api/posts/{id}/comments", post(add_comment_handler))
        .fallback(handle_404)
        .layer(Extension(state))
}

#[tokio::test]
async fn test_blog_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_about_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/about").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_handler_existing() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/blog/getting-started-with-rust").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_post_handler_nonexistent() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/blog/nonexistent-post").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_tag_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/blog/tag/rust").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_posts_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/api/posts?page=1&per_page=5").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_search_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/api/search?q=rust&in=title,content").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_time_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/api/time").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_comments_handlers() {
    let state = create_test_state();
    let app = create_test_router(state);

    // Test getting comments for a post
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/api/posts/getting-started-with-rust/comments").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test adding a comment
    let comment = CommentRequest {
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/posts/getting-started-with-rust/comments")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&comment).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_404_handler() {
    let state = create_test_state();
    let app = create_test_router(state);

    let response = app
        .oneshot(Request::builder().uri("/nonexistent-route").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
} 