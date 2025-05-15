use serde::Deserialize;
use serde::Serialize;

/// Query parameters for search functionality
#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default, rename = "in")]
    pub search_in: String,
} 