use axum::{
    response::{Html, Json, IntoResponse},
    extract::{Extension, Path, Query, Json as JsonExtractor},
    http::StatusCode,
};
use std::sync::Arc;
use chrono::{Local, Utc};
use uuid::Uuid;

use crate::{
    models::{TimeResponse, BlogPost, SearchQuery, Comment, CommentRequest},
    state::AppState,
    data::get_blog_posts,
};

// Handler for the home page
pub async fn home_handler(
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
pub async fn about_handler(
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
pub async fn time_handler() -> Json<TimeResponse> {
    Json(TimeResponse {
        current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

// Handler for the blog page
pub async fn blog_handler(
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
pub async fn post_handler(
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
pub async fn posts_handler() -> Json<Vec<BlogPost>> {
    Json(get_blog_posts())
}

// Handler for the search API
pub async fn search_handler(Query(params): Query<SearchQuery>) -> Json<Vec<BlogPost>> {
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
pub async fn tag_handler(
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
pub async fn handle_404(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    let mut context = tera::Context::new();
    context.insert("current_page", "404");
    
    let rendered = state.templates
        .render("404.html", &context)
        .expect("Failed to render template");
    
    (StatusCode::NOT_FOUND, Html(rendered))
}

// Handler for getting comments for a post
pub async fn get_comments_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Json<Vec<Comment>> {
    let comments = state.comments.read().unwrap();
    let post_comments = comments.get(&post_id).cloned().unwrap_or_default();
    Json(post_comments)
}

// Handler for adding a comment to a post
pub async fn add_comment_handler(
    Extension(state): Extension<Arc<AppState>>,
    Path(post_id): Path<String>,
    JsonExtractor(comment_req): JsonExtractor<CommentRequest>,
) -> impl IntoResponse {
    // Verify the post exists
    let posts = get_blog_posts();
    if !posts.iter().any(|p| p.id == post_id) {
        return (StatusCode::NOT_FOUND, "Post not found").into_response();
    }

    let comment = Comment {
        id: Uuid::new_v4().to_string(),
        post_id,
        author: comment_req.author,
        content: comment_req.content,
        created_at: Utc::now(),
    };

    let mut comments = state.comments.write().unwrap();
    let post_comments = comments.entry(comment.post_id.clone()).or_insert_with(Vec::new);
    post_comments.push(comment.clone());

    (StatusCode::CREATED, Json(comment)).into_response()
} 