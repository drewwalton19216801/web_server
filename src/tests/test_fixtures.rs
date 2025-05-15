use std::sync::Arc;
use tera::Tera;
use std::collections::HashMap;
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    state::AppState,
    models::{BlogPost, Comment, CommentRequest, SearchQuery, PaginationParams},
    tests::test_config::{
        TEST_POST_IDS,
        TEST_TAGS,
        TEST_COMMENT_AUTHORS,
        TEST_COMMENT_CONTENTS,
    },
};

/// Creates a test state with sample data
pub fn create_test_state_with_data() -> Arc<AppState> {
    let templates = Tera::new("templates/**/*").expect("Failed to initialize Tera");
    let state = Arc::new(AppState {
        templates,
        comments: RwLock::new(HashMap::new()),
    });
    
    // Add sample comments
    for (i, post_id) in TEST_POST_IDS.iter().enumerate() {
        let comment = Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post_id.to_string(),
            author: TEST_COMMENT_AUTHORS[i % TEST_COMMENT_AUTHORS.len()].to_string(),
            content: TEST_COMMENT_CONTENTS[i % TEST_COMMENT_CONTENTS.len()].to_string(),
            created_at: Utc::now() - chrono::Duration::minutes(i as i64),
        };
        
        let mut comments = state.comments.write().unwrap();
        let post_comments = comments.entry(post_id.to_string()).or_insert_with(Vec::new);
        post_comments.push(comment);
    }
    
    state
}

/// Creates a sample blog post
pub fn create_sample_blog_post() -> BlogPost {
    BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
            
            <h2>Why Choose Rust?</h2>
            <p>Rust offers several advantages over other programming languages:</p>
            <ul>
                <li>Memory safety without garbage collection</li>
                <li>Concurrency without data races</li>
                <li>Abstraction without performance penalties</li>
            </ul>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    }
}

/// Creates a sample comment request
pub fn create_sample_comment_request() -> CommentRequest {
    CommentRequest {
        author: TEST_COMMENT_AUTHORS[0].to_string(),
        content: TEST_COMMENT_CONTENTS[0].to_string(),
    }
}

/// Creates a sample search query
pub fn create_sample_search_query() -> SearchQuery {
    SearchQuery {
        q: "rust".to_string(),
        search_in: "title,content".to_string(),
    }
}

/// Creates a sample pagination parameters
pub fn create_sample_pagination_params() -> PaginationParams {
    PaginationParams {
        page: 1,
        per_page: 10,
    }
}

/// Creates a sample blog post with comments
pub fn create_sample_blog_post_with_comments() -> (BlogPost, Vec<Comment>) {
    let post = BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
            
            <h2>Why Choose Rust?</h2>
            <p>Rust offers several advantages over other programming languages:</p>
            <ul>
                <li>Memory safety without garbage collection</li>
                <li>Concurrency without data races</li>
                <li>Abstraction without performance penalties</li>
            </ul>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    };
    
    let comments = vec![
        Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post.id.clone(),
            author: TEST_COMMENT_AUTHORS[0].to_string(),
            content: TEST_COMMENT_CONTENTS[0].to_string(),
            created_at: Utc::now(),
        },
        Comment {
            id: Uuid::new_v4().to_string(),
            post_id: post.id.clone(),
            author: TEST_COMMENT_AUTHORS[1].to_string(),
            content: TEST_COMMENT_CONTENTS[1].to_string(),
            created_at: Utc::now(),
        },
    ];
    
    (post, comments)
}

/// Creates a sample blog post with specific tags
pub fn create_sample_blog_post_with_tags(tags: Vec<String>) -> BlogPost {
    BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
        "#.to_string(),
        tags,
    }
}

/// Creates a sample blog post with specific content
pub fn create_sample_blog_post_with_content(content: String) -> BlogPost {
    BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content,
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    }
}

/// Creates a sample blog post with specific author
pub fn create_sample_blog_post_with_author(author: String) -> BlogPost {
    BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author,
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    }
}

/// Creates a sample blog post with specific date
pub fn create_sample_blog_post_with_date(date: String) -> BlogPost {
    BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date,
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    }
}

/// Creates a sample blog post with pagination
pub fn create_sample_blog_post_with_pagination() -> (BlogPost, PaginationParams) {
    let post = BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
            
            <h2>Why Choose Rust?</h2>
            <p>Rust offers several advantages over other programming languages:</p>
            <ul>
                <li>Memory safety without garbage collection</li>
                <li>Concurrency without data races</li>
                <li>Abstraction without performance penalties</li>
            </ul>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    };
    
    let pagination = PaginationParams {
        page: 1,
        per_page: 10,
    };
    
    (post, pagination)
}

/// Creates a sample blog post with search
pub fn create_sample_blog_post_with_search() -> (BlogPost, SearchQuery) {
    let post = BlogPost {
        id: TEST_POST_IDS[0].to_string(),
        title: "Getting Started with Rust".to_string(),
        date: "2024-03-20".to_string(),
        author: "John Doe".to_string(),
        excerpt: "Learn the basics of Rust programming language.".to_string(),
        content: r#"
            <h2>Introduction to Rust</h2>
            <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.</p>
            
            <h2>Why Choose Rust?</h2>
            <p>Rust offers several advantages over other programming languages:</p>
            <ul>
                <li>Memory safety without garbage collection</li>
                <li>Concurrency without data races</li>
                <li>Abstraction without performance penalties</li>
            </ul>
        "#.to_string(),
        tags: TEST_TAGS.iter().map(|&s| s.to_string()).collect(),
    };
    
    let search = SearchQuery {
        q: "rust".to_string(),
        search_in: "title,content".to_string(),
    };
    
    (post, search)
} 