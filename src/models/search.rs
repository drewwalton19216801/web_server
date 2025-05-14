use serde::Deserialize;

/// Query parameters for search functionality
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default, rename = "in")]
    pub search_in: String,
} 