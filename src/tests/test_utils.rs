use std::sync::Arc;
use tera::Tera;
use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    state::AppState,
    models::{BlogPost, Comment, CommentRequest},
};

/// Creates a test state with initialized templates and empty comments
pub fn create_test_state() -> Arc<AppState> {
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    Arc::new(AppState {
        templates,
        comments: RwLock::new(HashMap::new()),
    })
}

/// Creates a test blog post
pub fn create_test_post() -> BlogPost {
    BlogPost {
        id: "test-post".to_string(),
        title: "Test Post".to_string(),
        date: "2024-03-20".to_string(),
        author: "Test Author".to_string(),
        excerpt: "Test excerpt".to_string(),
        content: "Test content".to_string(),
        tags: vec!["test".to_string(), "rust".to_string()],
    }
}

/// Creates a test comment
pub fn create_test_comment(post_id: String) -> Comment {
    Comment {
        id: Uuid::new_v4().to_string(),
        post_id,
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
        created_at: Utc::now(),
    }
}

/// Creates a test comment request
pub fn create_test_comment_request() -> CommentRequest {
    CommentRequest {
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
    }
}

/// Adds a test comment to the state
pub fn add_test_comment(state: &Arc<AppState>, post_id: String) -> Comment {
    let comment = create_test_comment(post_id.clone());
    let mut comments = state.comments.write().unwrap();
    let post_comments = comments.entry(post_id).or_insert_with(Vec::new);
    post_comments.push(comment.clone());
    comment
}

/// Verifies that a comment exists in the state
pub fn verify_comment_exists(state: &Arc<AppState>, post_id: &str, comment_id: &str) -> bool {
    let comments = state.comments.read().unwrap();
    if let Some(post_comments) = comments.get(post_id) {
        post_comments.iter().any(|c| c.id == comment_id)
    } else {
        false
    }
}

/// Creates multiple test comments for a post
pub fn create_multiple_test_comments(state: &Arc<AppState>, post_id: String, count: usize) -> Vec<Comment> {
    let mut comments = Vec::new();
    for i in 0..count {
        let comment = Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post_id.clone(),
            author: format!("Test User {}", i),
            content: format!("Test comment {}", i),
            created_at: Utc::now() - chrono::Duration::minutes(i as i64),
        };
        comments.push(comment.clone());
        
        let mut stored_comments = state.comments.write().unwrap();
        let post_comments = stored_comments.entry(post_id.clone()).or_insert_with(Vec::new);
        post_comments.push(comment);
    }
    comments
}

/// Verifies the number of comments for a post
pub fn verify_comment_count(state: &Arc<AppState>, post_id: &str, expected_count: usize) -> bool {
    let comments = state.comments.read().unwrap();
    if let Some(post_comments) = comments.get(post_id) {
        post_comments.len() == expected_count
    } else {
        expected_count == 0
    }
}

/// Clears all comments from the state
pub fn clear_comments(state: &Arc<AppState>) {
    let mut comments = state.comments.write().unwrap();
    comments.clear();
}

/// Verifies that the state has no comments
pub fn verify_no_comments(state: &Arc<AppState>) -> bool {
    let comments = state.comments.read().unwrap();
    comments.is_empty()
} 