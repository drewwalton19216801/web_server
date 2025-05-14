use crate::models::BlogPost;

pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "getting-started-with-rust".to_string(),
            title: "Getting Started with Rust".to_string(),
            date: "2024-03-20".to_string(),
            author: "John Doe".to_string(),
            excerpt: "Learn the basics of Rust programming language and why it's becoming increasingly popular for systems programming.".to_string(),
            content: r#"
                <h2>Introduction to Rust</h2>
                <p>Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It's designed to be memory-safe without using garbage collection.</p>
                
                <h2>Why Choose Rust?</h2>
                <p>Rust offers several advantages over other programming languages:</p>
                <ul>
                    <li>Memory safety without garbage collection</li>
                    <li>Concurrency without data races</li>
                    <li>Abstraction without performance penalties</li>
                    <li>Stability without stagnation</li>
                </ul>

                <h2>Getting Started</h2>
                <p>To start using Rust, you'll need to:</p>
                <ol>
                    <li>Install Rust using rustup</li>
                    <li>Create your first project with Cargo</li>
                    <li>Learn the basic syntax and concepts</li>
                </ol>

                <h2>Conclusion</h2>
                <p>Rust is an excellent choice for systems programming, web development, and many other use cases. Its focus on safety and performance makes it a compelling alternative to traditional systems languages.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "beginners".to_string(), "programming".to_string()],
        },
        BlogPost {
            id: "building-web-apps-with-axum".to_string(),
            title: "Building Web Applications with Axum".to_string(),
            date: "2024-03-19".to_string(),
            author: "Jane Smith".to_string(),
            excerpt: "A comprehensive guide to building modern web applications using the Axum framework in Rust.".to_string(),
            content: r#"
                <h2>What is Axum?</h2>
                <p>Axum is a web application framework for Rust that focuses on ergonomics, modularity, and performance. It's built on top of Tokio and Tower.</p>

                <h2>Key Features</h2>
                <ul>
                    <li>Routing with extractors</li>
                    <li>Middleware support</li>
                    <li>Error handling</li>
                    <li>WebSocket support</li>
                </ul>

                <h2>Building Your First App</h2>
                <p>Let's create a simple web application with Axum:</p>
                <pre><code>
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}
                </code></pre>

                <h2>Conclusion</h2>
                <p>Axum provides a powerful and ergonomic way to build web applications in Rust. Its focus on modularity and performance makes it an excellent choice for modern web development.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "axum".to_string(), "web-development".to_string()],
        },
        BlogPost {
            id: "async-programming-in-rust".to_string(),
            title: "Async Programming in Rust".to_string(),
            date: "2024-03-18".to_string(),
            author: "Mike Johnson".to_string(),
            excerpt: "Understanding async/await and how to write efficient asynchronous code in Rust.".to_string(),
            content: r#"
                <h2>Understanding Async/Await</h2>
                <p>Async/await in Rust provides a way to write asynchronous code that looks and feels like synchronous code. It's built on top of the Future trait and the Tokio runtime.</p>

                <h2>Key Concepts</h2>
                <ul>
                    <li>Futures and async functions</li>
                    <li>The .await syntax</li>
                    <li>Task scheduling</li>
                    <li>Error handling</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
async fn fetch_data() -> Result<String, Error> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let data = response.text().await?;
    Ok(data)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Got data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When writing async code in Rust:</p>
                <ul>
                    <li>Use async/await instead of raw Futures when possible</li>
                    <li>Be mindful of blocking operations</li>
                    <li>Handle errors appropriately</li>
                    <li>Consider performance implications</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Async programming in Rust provides powerful tools for writing efficient, concurrent applications. With proper understanding and practice, you can write high-performance async code that's both safe and maintainable.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "async".to_string(), "performance".to_string()],
        },
        BlogPost {
            id: "error-handling-in-rust".to_string(),
            title: "Error Handling in Rust".to_string(),
            date: "2024-03-17".to_string(),
            author: "Sarah Wilson".to_string(),
            excerpt: "A deep dive into Rust's error handling mechanisms and best practices.".to_string(),
            content: r#"
                <h2>Understanding Error Handling</h2>
                <p>Rust's approach to error handling is unique and powerful, combining the Result type with pattern matching for robust error management.</p>

                <h2>Key Concepts</h2>
                <ul>
                    <li>The Result type</li>
                    <li>Error propagation with ?</li>
                    <li>Custom error types</li>
                    <li>Error conversion</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

fn process_file() -> Result<String, AppError> {
    let content = std::fs::read_to_string("data.json")?;
    let data: Data = serde_json::from_str(&content)?;
    Ok(data.to_string())
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When handling errors in Rust:</p>
                <ul>
                    <li>Use the Result type for recoverable errors</li>
                    <li>Implement custom error types for your application</li>
                    <li>Use the ? operator for error propagation</li>
                    <li>Provide meaningful error messages</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Rust's error handling system encourages writing robust and maintainable code by making error cases explicit and forcing developers to handle them appropriately.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "error-handling".to_string(), "best-practices".to_string()],
        },
        BlogPost {
            id: "testing-in-rust".to_string(),
            title: "Testing in Rust".to_string(),
            date: "2024-03-16".to_string(),
            author: "David Brown".to_string(),
            excerpt: "Learn how to write effective tests in Rust using the built-in testing framework.".to_string(),
            content: r#"
                <h2>Rust's Testing Framework</h2>
                <p>Rust comes with a powerful built-in testing framework that makes it easy to write and run tests for your code.</p>

                <h2>Types of Tests</h2>
                <ul>
                    <li>Unit tests</li>
                    <li>Integration tests</li>
                    <li>Documentation tests</li>
                    <li>Property-based tests</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_division() {
        divide(10, 0);
    }
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When writing tests in Rust:</p>
                <ul>
                    <li>Write tests alongside your code</li>
                    <li>Use descriptive test names</li>
                    <li>Test both success and failure cases</li>
                    <li>Keep tests focused and isolated</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Testing is a crucial part of software development, and Rust's testing framework makes it easy to write comprehensive tests for your code.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "testing".to_string(), "best-practices".to_string()],
        },
        BlogPost {
            id: "rust-memory-safety".to_string(),
            title: "Understanding Rust's Memory Safety".to_string(),
            date: "2024-03-15".to_string(),
            author: "Emily Chen".to_string(),
            excerpt: "A detailed exploration of how Rust ensures memory safety without garbage collection.".to_string(),
            content: r#"
                <h2>Memory Safety in Rust</h2>
                <p>Rust's unique approach to memory safety combines ownership, borrowing, and lifetimes to prevent common memory-related bugs.</p>

                <h2>Key Concepts</h2>
                <ul>
                    <li>Ownership rules</li>
                    <li>Borrowing and references</li>
                    <li>Lifetimes</li>
                    <li>Smart pointers</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1's ownership moves to s2
    // println!("{}", s1); // This would cause a compile error
    println!("{}", s2); // This works fine
}

fn process_string(s: &str) {
    println!("Processing: {}", s);
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When working with memory in Rust:</p>
                <ul>
                    <li>Understand ownership rules</li>
                    <li>Use references when appropriate</li>
                    <li>Let the compiler guide you</li>
                    <li>Use smart pointers for complex cases</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Rust's memory safety guarantees make it possible to write high-performance code without the overhead of garbage collection or the risks of manual memory management.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "memory-safety".to_string(), "systems-programming".to_string()],
        },
        BlogPost {
            id: "rust-concurrency".to_string(),
            title: "Concurrency in Rust".to_string(),
            date: "2024-03-14".to_string(),
            author: "Alex Thompson".to_string(),
            excerpt: "Exploring Rust's concurrency features and how to write safe concurrent code.".to_string(),
            content: r#"
                <h2>Concurrency in Rust</h2>
                <p>Rust provides powerful tools for writing concurrent code while maintaining safety guarantees.</p>

                <h2>Key Concepts</h2>
                <ul>
                    <li>Threads and thread safety</li>
                    <li>Message passing</li>
                    <li>Shared state concurrency</li>
                    <li>Async/await</li>
                </ul>

                <h2>Example Code</h2>
                <pre><code>
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
                </code></pre>

                <h2>Best Practices</h2>
                <p>When writing concurrent code in Rust:</p>
                <ul>
                    <li>Prefer message passing over shared state</li>
                    <li>Use appropriate synchronization primitives</li>
                    <li>Consider using async/await for I/O-bound tasks</li>
                    <li>Test concurrent code thoroughly</li>
                </ul>

                <h2>Conclusion</h2>
                <p>Rust's concurrency features make it possible to write safe and efficient concurrent code without the common pitfalls of other languages.</p>
            "#.to_string(),
            tags: vec!["rust".to_string(), "concurrency".to_string(), "performance".to_string()],
        },
    ]
} 