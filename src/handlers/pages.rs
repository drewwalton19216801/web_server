use axum::{
    response::{Html, IntoResponse},
    extract::{Extension, Path},
    http::StatusCode,
};
use std::sync::Arc;
use crate::{
    state::AppState,
    data::get_blog_posts,
    handlers::errors::render_error_page,
};

/// Handler for the about page
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

/// Handler for the blog page
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

/// Handler for individual blog posts
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
            Err(_) => render_error_page(
                &state,
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
                "An error occurred while rendering the page."
            ).into_response(),
        }
    } else {
        render_error_page(
            &state,
            StatusCode::NOT_FOUND,
            "Post Not Found",
            "The blog post you're looking for doesn't exist or has been removed."
        ).into_response()
    }
}

/// Handler for tag-based post filtering
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
        Err(_) => render_error_page(
            &state,
            StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
            "An error occurred while rendering the page."
        ).into_response(),
    }
} 