use axum::{
    response::{Html, IntoResponse},
    extract::Extension,
    http::StatusCode,
};
use std::sync::Arc;
use crate::state::AppState;

/// Helper function to render error pages
pub fn render_error_page(
    state: &AppState,
    status_code: StatusCode,
    title: &str,
    message: &str,
) -> impl IntoResponse {
    let mut context = tera::Context::new();
    context.insert("current_page", "error");
    context.insert("error_code", &status_code.as_u16().to_string());
    context.insert("error_title", title);
    context.insert("error_message", message);
    
    match state.templates.render("error.html", &context) {
        Ok(rendered) => (status_code, Html(rendered)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
    }
}

/// Handler for 404 errors
pub async fn handle_404(
    Extension(state): Extension<Arc<AppState>>,
) -> impl IntoResponse {
    render_error_page(
        &state,
        StatusCode::NOT_FOUND,
        "Page Not Found",
        "The page you're looking for doesn't exist or has been moved."
    )
} 