use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Represents a comment on a blog post
#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Request body for adding a new comment
#[derive(Deserialize)]
pub struct CommentRequest {
    pub author: String,
    pub content: String,
} 