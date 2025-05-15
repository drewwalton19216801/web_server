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

/// Mock template engine for testing
pub struct MockTera {
    templates: HashMap<String, String>,
}

impl MockTera {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        templates.insert(
            "blog.html".to_string(),
            r#"
                <h1>Blog Posts</h1>
                {% for post in posts %}
                    <article>
                        <h2>{{ post.title }}</h2>
                        <p>{{ post.excerpt }}</p>
                    </article>
                {% endfor %}
            "#.to_string(),
        );
        templates.insert(
            "about.html".to_string(),
            r#"
                <h1>About</h1>
                <p>This is a test blog.</p>
            "#.to_string(),
        );
        templates.insert(
            "post.html".to_string(),
            r#"
                <article>
                    <h1>{{ post.title }}</h1>
                    <p>{{ post.content }}</p>
                    <div class="comments">
                        {% for comment in comments %}
                            <div class="comment">
                                <p>{{ comment.content }}</p>
                                <small>By {{ comment.author }}</small>
                            </div>
                        {% endfor %}
                    </div>
                </article>
            "#.to_string(),
        );
        templates.insert(
            "tag.html".to_string(),
            r#"
                <h1>Posts tagged with "{{ tag }}"</h1>
                {% for post in posts %}
                    <article>
                        <h2>{{ post.title }}</h2>
                        <p>{{ post.excerpt }}</p>
                    </article>
                {% endfor %}
            "#.to_string(),
        );
        templates.insert(
            "error.html".to_string(),
            r#"
                <h1>Error {{ error_code }}</h1>
                <h2>{{ error_title }}</h2>
                <p>{{ error_message }}</p>
            "#.to_string(),
        );
        
        Self { templates }
    }
    
    pub fn render(&self, template_name: &str, _context: &tera::Context) -> Result<String, tera::Error> {
        self.templates
            .get(template_name)
            .cloned()
            .ok_or_else(|| tera::Error::msg("Template not found"))
    }
    
    pub fn get_template_names(&self) -> impl Iterator<Item = &String> {
        self.templates.keys()
    }
}

/// Mock state for testing
pub struct MockState {
    pub templates: MockTera,
    pub comments: RwLock<HashMap<String, Vec<Comment>>>,
}

impl MockState {
    pub fn new() -> Self {
        Self {
            templates: MockTera::new(),
            comments: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn with_comments(mut self, comments: Vec<Comment>) -> Self {
        let mut comments_map = HashMap::new();
        for comment in comments {
            let post_comments = comments_map
                .entry(comment.post_id.clone())
                .or_insert_with(Vec::new);
            post_comments.push(comment);
        }
        self.comments = RwLock::new(comments_map);
        self
    }
}

/// Mock blog post for testing
pub struct MockBlogPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub excerpt: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl MockBlogPost {
    pub fn new() -> Self {
        Self {
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
    
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }
    
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    
    pub fn with_date(mut self, date: String) -> Self {
        self.date = date;
        self
    }
    
    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }
    
    pub fn with_excerpt(mut self, excerpt: String) -> Self {
        self.excerpt = excerpt;
        self
    }
    
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn into_blog_post(self) -> BlogPost {
        BlogPost {
            id: self.id,
            title: self.title,
            date: self.date,
            author: self.author,
            excerpt: self.excerpt,
            content: self.content,
            tags: self.tags,
        }
    }
}

/// Mock comment for testing
pub struct MockComment {
    pub id: String,
    pub post_id: String,
    pub author: String,
    pub content: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl MockComment {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            post_id: TEST_POST_IDS[0].to_string(),
            author: TEST_COMMENT_AUTHORS[0].to_string(),
            content: TEST_COMMENT_CONTENTS[0].to_string(),
            created_at: Utc::now(),
        }
    }
    
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }
    
    pub fn with_post_id(mut self, post_id: String) -> Self {
        self.post_id = post_id;
        self
    }
    
    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }
    
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
    
    pub fn with_created_at(mut self, created_at: chrono::DateTime<Utc>) -> Self {
        self.created_at = created_at;
        self
    }
    
    pub fn into_comment(self) -> Comment {
        Comment {
            id: self.id,
            post_id: self.post_id,
            author: self.author,
            content: self.content,
            created_at: self.created_at,
        }
    }
}

/// Mock comment request for testing
pub struct MockCommentRequest {
    pub author: String,
    pub content: String,
}

impl MockCommentRequest {
    pub fn new() -> Self {
        Self {
            author: TEST_COMMENT_AUTHORS[0].to_string(),
            content: TEST_COMMENT_CONTENTS[0].to_string(),
        }
    }
    
    pub fn with_author(mut self, author: String) -> Self {
        self.author = author;
        self
    }
    
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }
    
    pub fn into_comment_request(self) -> CommentRequest {
        CommentRequest {
            author: self.author,
            content: self.content,
        }
    }
}

/// Mock search query for testing
pub struct MockSearchQuery {
    pub q: String,
    pub search_in: String,
}

impl MockSearchQuery {
    pub fn new() -> Self {
        Self {
            q: "test".to_string(),
            search_in: "title,content".to_string(),
        }
    }
    
    pub fn with_query(mut self, q: String) -> Self {
        self.q = q;
        self
    }
    
    pub fn with_search_in(mut self, search_in: String) -> Self {
        self.search_in = search_in;
        self
    }
    
    pub fn into_search_query(self) -> SearchQuery {
        SearchQuery {
            q: self.q,
            search_in: self.search_in,
        }
    }
}

/// Mock pagination parameters for testing
pub struct MockPaginationParams {
    pub page: usize,
    pub per_page: usize,
}

impl MockPaginationParams {
    pub fn new() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
    
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }
    
    pub fn with_per_page(mut self, per_page: usize) -> Self {
        self.per_page = per_page;
        self
    }
    
    pub fn into_pagination_params(self) -> PaginationParams {
        PaginationParams {
            page: self.page,
            per_page: self.per_page,
        }
    }
} 