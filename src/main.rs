use axum::{
    routing::get,
    Router,
    response::{Html, Json, IntoResponse},
    extract::{Extension, Path, Query},
    http::StatusCode,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;
use chrono::Local;
use serde::{Serialize, Deserialize};

// Shared state for our application
struct AppState {
    templates: Tera,
}

#[derive(Serialize)]
struct TimeResponse {
    current_time: String,
}

#[derive(Serialize, Clone)]
struct BlogPost {
    id: String,
    title: String,
    date: String,
    author: String,
    excerpt: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default, rename = "in")]
    search_in: String,
}

// Store blog posts in memory (in a real app, this would be a database)
fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "getting-started-with-rust".to_string(),
            title: "Getting Started with Rust".to_string(),
            date: "2024-03-20".to_string(),
            author: "John Doe".to_string(),
            excerpt: "Learn the basics of Rust programming language and why it's becoming increasingly popular for systems programming.".to_string(),
            content: r#"
                <h2>Introduction to Rust</h2>
                <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It's designed to be memory-safe without using garbage collection.</p>
                
                <h2>Why Choose Rust?</h2>
                <p>Rust offers several advantages over other programming languages:</p>
                <ul>
                    <li>Memory safety without garbage collection</li>
                    <li>Concurrency without data races</li>
                    <li>Abstraction without performance penalties</li>
                    <li>Stability without stagnation</li>
                </ul>

                <h2>Getting Started</h2>
                <p>To start using Rust, you'll need to:</p>
                <ol>
                    <li>Install Rust using rustup</li>
                    <li>Create your first project with Cargo</li>
                    <li>Learn the basic syntax and concepts</li>
                </ol>

                <h2>Conclusion</h2>
                <p>Rust is an excellent choice for systems programming, web development, and many other use cases. Its focus on safety and performance makes it a compelling alternative to traditional systems languages.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "beginners".to_string(), "programming".to_string()],
        },
        BlogPost {
            id: "building-web-apps-with-axum".to_string(),
            title: "Building Web Applications with Axum".to_string(),
            date: "2024-03-19".to_string(),
            author: "Jane Smith".to_string(),
            excerpt: "A comprehensive guide to building modern web applications using the Axum framework in Rust.".to_string(),
            content: r#"
                <h2>What is Axum?</h2>
                <p>Axum is a web application framework for Rust that focuses on ergonomics, modularity, and performance. It's built on top of Tokio and Tower.</p>

                <h2>Key Features</h2>
                <ul>
                    <li>Routing with extractors</li>
                    <li>Middleware support</li>
                    <li>Error handling</li>
                    <li>WebSocket support</li>
                </ul>

                <h2>Building Your First App</h2>
                <p>Let's create a simple web application with Axum:</p>
                <pre><code>
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
                </code></pre>

                <h2>Conclusion</h2>
                <p>Axum provides a powerful and ergonomic way to build web applications in Rust. Its focus on modularity and performance makes it an excellent choice for modern web development.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "axum".to_string(), "web-development".to_string()],
        },
        BlogPost {
            id: "async-programming-in-rust".to_string(),
            title: "Async Programming in Rust".to_string(),
            date: "2024-03-18".to_string(),
            author: "Mike Johnson".to_string(),
            excerpt: "Understanding async/await and how to write efficient asynchronous code in Rust.".to_string(),
            content: r#"
                <h2>Understanding Async/Await</h2>
                <p>Async/await in Rust provides a way to write asynchronous code that looks and feels like synchronous code. It's built on top of the Future trait and the Tokio runtime.</p>

                <h2>Key Concepts</h2>
                <ul>
                    <li>Futures and async functions</li>
                    <li>The .await syntax</li>
                    <li>Task scheduling</li>
                    <li>Error handling</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
async fn fetch_data() -> Result<String, Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let data = response.text().await?;
    Ok(data)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Got data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When writing async code in Rust:</p>
                <ul>
                    <li>Use async/await instead of raw Futures when possible</li>
                    <li>Be mindful of blocking operations</li>
                    <li>Handle errors appropriately</li>
                    <li>Consider performance implications</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Async programming in Rust provides powerful tools for writing efficient, concurrent applications. With proper understanding and practice, you can write high-performance async code that's both safe and maintainable.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "async".to_string(), "performance".to_string()],
        },
    ]
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
        .route("/blog/{id}", get(post_handler))
        .route("/blog/tag/{tag}", get(tag_handler))
        .route("/api/time", get(time_handler))
        .route("/api/posts", get(posts_handler))
        .route("/api/search", get(search_handler))
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

// Handler for individual blog posts
async fn post_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let posts = get_blog_posts();
    if let Some(post) = posts.into_iter().find(|p| p.id == id) {
        let mut context = tera::Context::new();
        context.insert("current_page", "blog");
        context.insert("post", &post);
        
        match state.templates.render("post.html", &context) {
            Ok(rendered) => Html(rendered).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, "Post not found").into_response()
    }
}

// Handler for the posts API
async fn posts_handler() -> Json<Vec<BlogPost>> {
    Json(get_blog_posts())
}

// Handler for the search API
async fn search_handler(Query(params): Query<SearchQuery>) -> Json<Vec<BlogPost>> {
    let query = params.q.to_lowercase();
    let search_in: Vec<&str> = params.search_in.split(',').collect();
    
    let results = get_blog_posts()
        .into_iter()
        .filter(|post| {
            if query.is_empty() {
                return false;
            }

            let mut matches = false;
            
            if search_in.contains(&"title") {
                matches |= post.title.to_lowercase().contains(&query);
            }
            
            if search_in.contains(&"content") {
                matches |= post.content.to_lowercase().contains(&query);
            }
            
            if search_in.contains(&"tags") {
                matches |= post.tags.iter().any(|tag| tag.to_lowercase().contains(&query));
            }
            
            matches
        })
        .collect();
    
    Json(results)
}

// Handler for tag-based post filtering
async fn tag_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(tag): Path<String>,
) -> impl IntoResponse {
    let posts = get_blog_posts()
        .into_iter()
        .filter(|post| post.tags.contains(&tag))
        .collect::<Vec<_>>();

    let mut context = tera::Context::new();
    context.insert("current_page", "blog");
    context.insert("tag", &tag);
    context.insert("posts", &posts);
    
    match state.templates.render("tag.html", &context) {
        Ok(rendered) => Html(rendered).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
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
