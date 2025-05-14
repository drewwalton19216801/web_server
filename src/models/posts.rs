use serde::{Serialize, Deserialize};

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