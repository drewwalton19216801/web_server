mod models;
mod state;
mod data;
mod handlers;

use axum::{
    routing::{get, post},
    Router,
    extract::Extension,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::{
    state::AppState,
    handlers::*,
};

#[tokio::main]
async fn main() {
    // Initialize Tera templates
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    
    // Create shared state
    let state = Arc::new(AppState { 
        templates,
        comments: RwLock::new(HashMap::new()),
    });

    // Build our application with routes
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/blog", get(blog_handler))
        .route("/blog/tag/{tag}", get(tag_handler))
        .route("/blog/{id}", get(post_handler))
        .route("/api/time", get(time_handler))
        .route("/api/posts", get(posts_handler))
        .route("/api/search", get(search_handler))
        .route("/api/posts/{id}/comments", get(get_comments_handler))
        .route("/api/posts/{id}/comments", post(add_comment_handler))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handle_404)
        .layer(Extension(state));

    // Run the server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
} 