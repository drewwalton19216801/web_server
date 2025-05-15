use chrono::Utc;
use crate::models::{
    BlogPost,
    Comment,
    CommentRequest,
    SearchQuery,
    PaginationParams,
    PaginatedResponse,
};

#[test]
fn test_blog_post_serialization() {
    let post = BlogPost {
        id: "test-post".to_string(),
        title: "Test Post".to_string(),
        date: "2024-03-20".to_string(),
        author: "Test Author".to_string(),
        excerpt: "Test excerpt".to_string(),
        content: "Test content".to_string(),
        tags: vec!["test".to_string(), "rust".to_string()],
    };

    let serialized = serde_json::to_string(&post).unwrap();
    let deserialized: BlogPost = serde_json::from_str(&serialized).unwrap();

    assert_eq!(post.id, deserialized.id);
    assert_eq!(post.title, deserialized.title);
    assert_eq!(post.date, deserialized.date);
    assert_eq!(post.author, deserialized.author);
    assert_eq!(post.excerpt, deserialized.excerpt);
    assert_eq!(post.content, deserialized.content);
    assert_eq!(post.tags, deserialized.tags);
}

#[test]
fn test_comment_serialization() {
    let comment = Comment {
        id: "test-comment".to_string(),
        post_id: "test-post".to_string(),
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
        created_at: Utc::now(),
    };

    let serialized = serde_json::to_string(&comment).unwrap();
    let deserialized: Comment = serde_json::from_str(&serialized).unwrap();

    assert_eq!(comment.id, deserialized.id);
    assert_eq!(comment.post_id, deserialized.post_id);
    assert_eq!(comment.author, deserialized.author);
    assert_eq!(comment.content, deserialized.content);
}

#[test]
fn test_comment_request_serialization() {
    let request = CommentRequest {
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CommentRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(request.author, deserialized.author);
    assert_eq!(request.content, deserialized.content);
}

#[test]
fn test_search_query_serialization() {
    let query = SearchQuery {
        q: "test query".to_string(),
        search_in: "title,content".to_string(),
    };

    let serialized = serde_json::to_string(&query).unwrap();
    let deserialized: SearchQuery = serde_json::from_str(&serialized).unwrap();

    assert_eq!(query.q, deserialized.q);
    assert_eq!(query.search_in, deserialized.search_in);
}

#[test]
fn test_pagination_params_defaults() {
    let params = PaginationParams {
        page: 0,
        per_page: 0,
    };

    let serialized = serde_json::to_string(&params).unwrap();
    let deserialized: PaginationParams = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.page, 0); // Default is not applied if field is present
    assert_eq!(deserialized.per_page, 0); // Default is not applied if field is present
}

#[test]
fn test_paginated_response() {
    let items = vec![
        BlogPost {
            id: "post-1".to_string(),
            title: "Post 1".to_string(),
            date: "2024-03-20".to_string(),
            author: "Author 1".to_string(),
            excerpt: "Excerpt 1".to_string(),
            content: "Content 1".to_string(),
            tags: vec!["test".to_string()],
        },
        BlogPost {
            id: "post-2".to_string(),
            title: "Post 2".to_string(),
            date: "2024-03-21".to_string(),
            author: "Author 2".to_string(),
            excerpt: "Excerpt 2".to_string(),
            content: "Content 2".to_string(),
            tags: vec!["test".to_string()],
        },
    ];

    let response = PaginatedResponse {
        items: items.clone(),
        total: 2,
        page: 1,
        per_page: 5,
        total_pages: 1,
    };

    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: PaginatedResponse<BlogPost> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.items.len(), items.len());
    assert_eq!(deserialized.total, 2);
    assert_eq!(deserialized.page, 1);
    assert_eq!(deserialized.per_page, 5);
    assert_eq!(deserialized.total_pages, 1);
}

#[test]
fn test_blog_post_clone() {
    let post = BlogPost {
        id: "test-post".to_string(),
        title: "Test Post".to_string(),
        date: "2024-03-20".to_string(),
        author: "Test Author".to_string(),
        excerpt: "Test excerpt".to_string(),
        content: "Test content".to_string(),
        tags: vec!["test".to_string(), "rust".to_string()],
    };

    let cloned = post.clone();
    assert_eq!(post.id, cloned.id);
    assert_eq!(post.title, cloned.title);
    assert_eq!(post.date, cloned.date);
    assert_eq!(post.author, cloned.author);
    assert_eq!(post.excerpt, cloned.excerpt);
    assert_eq!(post.content, cloned.content);
    assert_eq!(post.tags, cloned.tags);
}

#[test]
fn test_comment_clone() {
    let comment = Comment {
        id: "test-comment".to_string(),
        post_id: "test-post".to_string(),
        author: "Test User".to_string(),
        content: "Test comment".to_string(),
        created_at: Utc::now(),
    };

    let cloned = comment.clone();
    assert_eq!(comment.id, cloned.id);
    assert_eq!(comment.post_id, cloned.post_id);
    assert_eq!(comment.author, cloned.author);
    assert_eq!(comment.content, cloned.content);
    assert_eq!(comment.created_at, cloned.created_at);
} 