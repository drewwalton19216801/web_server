use std::collections::HashMap;
use std::sync::RwLock;
use tera::Tera;
use crate::models::Comment;

pub struct AppState {
    pub templates: Tera,
    pub comments: RwLock<HashMap<String, Vec<Comment>>>, // post_id -> comments
} 