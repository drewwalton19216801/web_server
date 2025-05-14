use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Represents a blog post with its metadata and content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// Represents a comment on a blog post
#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// Query parameters for search functionality
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default, rename = "in")]
    pub search_in: String,
}

/// Request body for adding a new comment
#[derive(Deserialize)]
pub struct CommentRequest {
    pub author: String,
    pub content: String,
}

/// Query parameters for pagination
#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

/// Response wrapper for paginated data
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

/// Default page number for pagination
fn default_page() -> usize {
    1
}

/// Default number of items per page for pagination
fn default_per_page() -> usize {
    5
} 