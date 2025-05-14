pub mod posts;
pub mod comments;
pub mod search;
pub mod pagination;

// Re-export commonly used types
pub use posts::BlogPost;
pub use comments::{Comment, CommentRequest};
pub use search::SearchQuery;
pub use pagination::{PaginationParams, PaginatedResponse}; 