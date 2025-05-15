use std::sync::Arc;
use tera::Tera;
use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    state::AppState,
    models::Comment,
};

#[test]
fn test_app_state_creation() {
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    let state = AppState {
        templates,
        comments: RwLock::new(HashMap::new()),
    };
    
    assert!(state.templates.get_template_names().count() > 0);
    assert!(state.comments.read().unwrap().is_empty());
}

#[test]
fn test_comments_storage() {
    let state = Arc::new(AppState {
        templates: Tera::new("templates/**/*").expect("Failed to initialize Tera"),
        comments: RwLock::new(HashMap::new()),
    });
    
    let post_id = "test-post".to_string();
    let comment = Comment {
        id: Uuid::new_v4().to_string(),
        post_id: post_id.clone(),
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
        created_at: Utc::now(),
    };
    
    // Test adding a comment
    {
        let mut comments = state.comments.write().unwrap();
        let post_comments = comments.entry(post_id.clone()).or_insert_with(Vec::new);
        post_comments.push(comment.clone());
    }
    
    // Test retrieving comments
    {
        let comments = state.comments.read().unwrap();
        let post_comments = comments.get(&post_id).unwrap();
        assert_eq!(post_comments.len(), 1);
        assert_eq!(post_comments[0].id, comment.id);
        assert_eq!(post_comments[0].author, comment.author);
        assert_eq!(post_comments[0].content, comment.content);
    }
}

#[test]
fn test_comments_concurrent_access() {
    let state = Arc::new(AppState {
        templates: Tera::new("templates/**/*").expect("Failed to initialize Tera"),
        comments: RwLock::new(HashMap::new()),
    });
    
    let post_id = "test-post".to_string();
    
    // Create multiple comments
    let comments: Vec<Comment> = (0..5)
        .map(|i| Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post_id.clone(),
            author: format!("User {}", i),
            content: format!("Comment {}", i),
            created_at: Utc::now(),
        })
        .collect();
    
    // Add comments concurrently
    let handles: Vec<_> = comments
        .iter()
        .map(|comment| {
            let state = Arc::clone(&state);
            let comment = comment.clone();
            let post_id = post_id.clone();
            std::thread::spawn(move || {
                let mut comments = state.comments.write().unwrap();
                let post_comments = comments.entry(post_id).or_insert_with(Vec::new);
                post_comments.push(comment);
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all comments were added
    let stored_comments = state.comments.read().unwrap();
    let post_comments = stored_comments.get(&post_id).unwrap();
    assert_eq!(post_comments.len(), 5);
}

#[test]
fn test_comments_empty_post() {
    let state = Arc::new(AppState {
        templates: Tera::new("templates/**/*").expect("Failed to initialize Tera"),
        comments: RwLock::new(HashMap::new()),
    });
    
    let post_id = "nonexistent-post".to_string();
    
    // Test retrieving comments for a post with no comments
    let comments = state.comments.read().unwrap();
    let post_comments = comments.get(&post_id);
    assert!(post_comments.is_none());
}

#[test]
fn test_comments_ordering() {
    let state = Arc::new(AppState {
        templates: Tera::new("templates/**/*").expect("Failed to initialize Tera"),
        comments: RwLock::new(HashMap::new()),
    });
    
    let post_id = "test-post".to_string();
    
    // Create comments with different timestamps
    let mut comments = Vec::new();
    for i in 0..3 {
        let comment = Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post_id.clone(),
            author: format!("User {}", i),
            content: format!("Comment {}", i),
            created_at: Utc::now() - chrono::Duration::minutes(i as i64),
        };
        comments.push(comment);
    }
    
    // Add comments in reverse order
    {
        let mut stored_comments = state.comments.write().unwrap();
        let post_comments = stored_comments.entry(post_id.clone()).or_insert_with(Vec::new);
        for comment in comments.iter().rev() {
            post_comments.push(comment.clone());
        }
    }
    
    // Verify comments are stored in the correct order
    let stored_comments = state.comments.read().unwrap();
    let post_comments = stored_comments.get(&post_id).unwrap();
    
    for (i, comment) in post_comments.iter().enumerate() {
        assert_eq!(comment.author, format!("User {}", 2 - i));
    }
} 