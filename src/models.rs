use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct TimeResponse {
    pub current_time: String,
}

#[derive(Serialize, Clone)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default, rename = "in")]
    pub search_in: String,
}

#[derive(Deserialize)]
pub struct CommentRequest {
    pub author: String,
    pub content: String,
} 