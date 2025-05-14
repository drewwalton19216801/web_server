use serde::{Serialize, Deserialize};

/// Query parameters for pagination
#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

/// Response wrapper for paginated data
#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

/// Default page number for pagination
fn default_page() -> usize {
    1
}

/// Default number of items per page for pagination
fn default_per_page() -> usize {
    5
} 