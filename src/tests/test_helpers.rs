use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
    routing::{get, post},
    Extension,
};
use std::sync::Arc;
use tower::util::ServiceExt;
use tera::Tera;
use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;
use bytes::Bytes;
use futures_util::StreamExt;
use hyper_util::rt::TokioIo;
use http_body_util::BodyExt;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{
    state::AppState,
    handlers::*,
    models::{BlogPost, Comment, CommentRequest, SearchQuery, PaginationParams},
    tests::test_config::{
        TEST_POST_IDS,
        TEST_TAGS,
        TEST_COMMENT_AUTHORS,
        TEST_COMMENT_CONTENTS,
    },
};

/// Creates a test router with the given state
pub fn create_test_router(state: Arc<AppState>) -> Router {
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

/// Makes a test request to the given router
pub async fn make_test_request(
    app: Router,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> (StatusCode, String) {
    let body_str = body.unwrap_or("").to_string();
    let request = Request::builder()
        .method(method)
        .uri(path)
        .body(Body::from(body_str))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(bytes.to_vec()).unwrap();

    (status, body_str)
}

/// Verifies that a response is successful
pub fn verify_successful_response(status: StatusCode, body: &str) -> bool {
    status.is_success() && !body.is_empty()
}

/// Verifies that a response is a 404 error
pub fn verify_not_found_response(status: StatusCode, body: &str) -> bool {
    status == StatusCode::NOT_FOUND && body.contains("Page Not Found")
}

/// Verifies that a response contains the expected content
pub fn verify_response_content(body: &str, expected_content: &str) -> bool {
    body.contains(expected_content)
}

/// Verifies that a response is a valid JSON
pub fn verify_json_response(body: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(body).is_ok()
}

/// Verifies that a response contains the expected number of items
pub fn verify_response_item_count(body: &str, expected_count: usize) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(array) = json.as_array() {
            return array.len() == expected_count;
        }
    }
    false
}

/// Verifies that a response contains the expected pagination information
pub fn verify_pagination_info(
    body: &str,
    expected_page: usize,
    expected_per_page: usize,
    expected_total: usize,
) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = json.as_object() {
            return obj.get("page").and_then(|v| v.as_u64()).map(|v| v as usize) == Some(expected_page)
                && obj.get("per_page").and_then(|v| v.as_u64()).map(|v| v as usize) == Some(expected_per_page)
                && obj.get("total").and_then(|v| v.as_u64()).map(|v| v as usize) == Some(expected_total);
        }
    }
    false
}

/// Verifies that a response contains the expected search results
pub fn verify_search_results(body: &str, expected_query: &str) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(array) = json.as_array() {
            return array.iter().all(|item| {
                if let Some(obj) = item.as_object() {
                    let title = obj.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let content = obj.get("content").and_then(|v| v.as_str()).unwrap_or("");
                    title.to_lowercase().contains(&expected_query.to_lowercase())
                        || content.to_lowercase().contains(&expected_query.to_lowercase())
                } else {
                    false
                }
            });
        }
    }
    false
}

/// Verifies that a response contains the expected comment
pub fn verify_comment_response(
    body: &str,
    expected_author: &str,
    expected_content: &str,
) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = json.as_object() {
            return obj.get("author").and_then(|v| v.as_str()) == Some(expected_author)
                && obj.get("content").and_then(|v| v.as_str()) == Some(expected_content);
        }
    }
    false
}

/// Verifies that a response contains the expected number of comments
pub fn verify_comment_count(body: &str, expected_count: usize) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(array) = json.as_array() {
            return array.len() == expected_count;
        }
    }
    false
}

/// Verifies that a response contains the expected blog post
pub fn verify_blog_post_response(
    body: &str,
    expected_title: &str,
    expected_author: &str,
) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = json.as_object() {
            return obj.get("title").and_then(|v| v.as_str()) == Some(expected_title)
                && obj.get("author").and_then(|v| v.as_str()) == Some(expected_author);
        }
    }
    false
}

/// Verifies that a response contains the expected number of blog posts
pub fn verify_blog_post_count(body: &str, expected_count: usize) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(array) = json.as_array() {
            return array.len() == expected_count;
        }
    }
    false
}

/// Verifies that a response contains the expected tags
pub fn verify_tags_response(body: &str, expected_tags: &[&str]) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = json.as_object() {
            if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
                return tags.iter().all(|tag| {
                    if let Some(tag_str) = tag.as_str() {
                        expected_tags.contains(&tag_str)
                    } else {
                        false
                    }
                });
            }
        }
    }
    false
}

/// Verifies that a response contains the expected number of tags
pub fn verify_tag_count(body: &str, expected_count: usize) -> bool {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(obj) = json.as_object() {
            if let Some(tags) = obj.get("tags").and_then(|v| v.as_array()) {
                return tags.len() == expected_count;
            }
        }
    }
    false
} 