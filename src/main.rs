use axum::{
    routing::get,
    Router,
    response::{Html, Json, IntoResponse},
    extract::Extension,
    http::StatusCode,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;
use chrono::Local;
use serde::Serialize;

// Shared state for our application
struct AppState {
    templates: Tera,
}

#[derive(Serialize)]
struct TimeResponse {
    current_time: String,
}

#[derive(Serialize)]
struct BlogPost {
    title: String,
    date: String,
    author: String,
    excerpt: String,
}

#[tokio::main]
async fn main() {
    // Initialize Tera templates
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    
    // Create shared state
    let state = Arc::new(AppState { templates });

    // Build our application with routes
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(about_handler))
        .route("/blog", get(blog_handler))
        .route("/api/time", get(time_handler))
        .route("/api/posts", get(posts_handler))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handle_404)
        .layer(Extension(state));

    // Run the server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// Handler for the home page
async fn home_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("current_page", "home");
    
    let rendered = state.templates
        .render("home.html", &context)
        .expect("Failed to render template");
    
    Html(rendered)
}

// Handler for the about page
async fn about_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("current_page", "about");
    
    let rendered = state.templates
        .render("about.html", &context)
        .expect("Failed to render template");
    
    Html(rendered)
}

// Handler for the time API
async fn time_handler() -> Json<TimeResponse> {
    Json(TimeResponse {
        current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

// Handler for the blog page
async fn blog_handler(
    Extension(state): Extension<Arc<AppState>>,
) -> Html<String> {
    let mut context = tera::Context::new();
    context.insert("current_page", "blog");
    
    let rendered = state.templates
        .render("blog.html", &context)
        .expect("Failed to render template");
    
    Html(rendered)
}

// Handler for the posts API
async fn posts_handler() -> Json<Vec<BlogPost>> {
    let posts = vec![
        BlogPost {
            title: "Getting Started with Rust".to_string(),
            date: "2024-03-20".to_string(),
            author: "John Doe".to_string(),
            excerpt: "Learn the basics of Rust programming language and why it's becoming increasingly popular for systems programming.".to_string(),
        },
        BlogPost {
            title: "Building Web Applications with Axum".to_string(),
            date: "2024-03-19".to_string(),
            author: "Jane Smith".to_string(),
            excerpt: "A comprehensive guide to building modern web applications using the Axum framework in Rust.".to_string(),
        },
        BlogPost {
            title: "Async Programming in Rust".to_string(),
            date: "2024-03-18".to_string(),
            author: "Mike Johnson".to_string(),
            excerpt: "Understanding async/await and how to write efficient asynchronous code in Rust.".to_string(),
        },
    ];
    
    Json(posts)
}

// Handler for 404 errors
async fn handle_404(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let mut context = tera::Context::new();
    context.insert("current_page", "404");
    
    let rendered = state.templates
        .render("404.html", &context)
        .expect("Failed to render template");
    
    (StatusCode::NOT_FOUND, Html(rendered))
}
