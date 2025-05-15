use std::time::Duration;

/// Default timeout for async operations in tests
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Default number of test posts to create
pub const DEFAULT_TEST_POSTS: usize = 5;

/// Default number of test comments per post
pub const DEFAULT_TEST_COMMENTS: usize = 3;

/// Default page size for pagination tests
pub const DEFAULT_PAGE_SIZE: usize = 10;

/// Test post IDs
pub const TEST_POST_IDS: [&str; 3] = [
    "getting-started-with-rust",
    "building-web-apps-with-axum",
    "async-programming-in-rust",
];

/// Test tags
pub const TEST_TAGS: [&str; 4] = [
    "rust",
    "axum",
    "async",
    "web-development",
];

/// Test search queries
pub const TEST_SEARCH_QUERIES: [&str; 3] = [
    "rust",
    "web",
    "async",
];

/// Test comment authors
pub const TEST_COMMENT_AUTHORS: [&str; 3] = [
    "John Doe",
    "Jane Smith",
    "Mike Johnson",
];

/// Test comment contents
pub const TEST_COMMENT_CONTENTS: [&str; 3] = [
    "Great post!",
    "Very informative.",
    "Thanks for sharing!",
];

/// Test API endpoints
pub const TEST_API_ENDPOINTS: [&str; 5] = [
    "/api/posts",
    "/api/search",
    "/api/time",
    "/api/posts/getting-started-with-rust/comments",
    "/api/posts/building-web-apps-with-axum/comments",
];

/// Test page routes
pub const TEST_PAGE_ROUTES: [&str; 4] = [
    "/",
    "/about",
    "/blog/getting-started-with-rust",
    "/blog/tag/rust",
];

/// Test error routes
pub const TEST_ERROR_ROUTES: [&str; 2] = [
    "/nonexistent-route",
    "/blog/nonexistent-post",
];

/// Test HTTP methods
pub const TEST_HTTP_METHODS: [&str; 2] = [
    "GET",
    "POST",
];

/// Test content types
pub const TEST_CONTENT_TYPES: [&str; 2] = [
    "application/json",
    "text/html",
];

/// Test HTTP status codes
pub const TEST_STATUS_CODES: [u16; 3] = [
    200, // OK
    201, // Created
    404, // Not Found
];

/// Test pagination parameters
pub const TEST_PAGINATION_PARAMS: [(usize, usize); 3] = [
    (1, 5),  // First page, 5 items
    (2, 10), // Second page, 10 items
    (3, 15), // Third page, 15 items
];

/// Test search parameters
pub const TEST_SEARCH_PARAMS: [(&str, &str); 3] = [
    ("rust", "title,content"),
    ("web", "title"),
    ("async", "content"),
];

/// Test comment request parameters
pub const TEST_COMMENT_PARAMS: [(&str, &str); 3] = [
    ("John Doe", "Great post!"),
    ("Jane Smith", "Very informative."),
    ("Mike Johnson", "Thanks for sharing!"),
]; 