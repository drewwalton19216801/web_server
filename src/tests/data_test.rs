use crate::data::get_blog_posts;

#[test]
fn test_get_blog_posts() {
    let posts = get_blog_posts();
    
    // Test that we get some posts
    assert!(!posts.is_empty());
    
    // Test that each post has required fields
    for post in posts {
        assert!(!post.id.is_empty());
        assert!(!post.title.is_empty());
        assert!(!post.date.is_empty());
        assert!(!post.author.is_empty());
        assert!(!post.excerpt.is_empty());
        assert!(!post.content.is_empty());
        assert!(!post.tags.is_empty());
    }
}

#[test]
fn test_blog_posts_unique_ids() {
    let posts = get_blog_posts();
    let mut ids = std::collections::HashSet::new();
    
    for post in posts {
        let post_id = post.id.clone();
        assert!(ids.insert(post_id), "Duplicate post ID found: {}", post.id);
    }
}

#[test]
fn test_blog_posts_date_format() {
    let posts = get_blog_posts();
    
    for post in posts {
        // Check that the date is in YYYY-MM-DD format
        assert_eq!(post.date.len(), 10);
        assert!(post.date.chars().nth(4).unwrap() == '-');
        assert!(post.date.chars().nth(7).unwrap() == '-');
        
        // Verify that the date components are valid numbers
        let year: i32 = post.date[0..4].parse().unwrap();
        let month: u32 = post.date[5..7].parse().unwrap();
        let day: u32 = post.date[8..10].parse().unwrap();
        
        assert!(year >= 2024); // Assuming posts are from 2024 or later
        assert!(month >= 1 && month <= 12);
        assert!(day >= 1 && day <= 31);
    }
}

#[test]
fn test_blog_posts_content_format() {
    let posts = get_blog_posts();
    
    for post in posts {
        // Check that content contains HTML tags
        assert!(post.content.contains("<h2>"));
        assert!(post.content.contains("</h2>"));
        assert!(post.content.contains("<p>"));
        assert!(post.content.contains("</p>"));
    }
}

#[test]
fn test_blog_posts_tags() {
    let posts = get_blog_posts();
    
    for post in posts {
        // Check that each post has at least one tag
        assert!(!post.tags.is_empty());
        
        // Check that tags are lowercase
        for tag in &post.tags {
            assert_eq!(tag.to_lowercase(), *tag);
        }
        
        // Check that tags don't contain spaces
        for tag in &post.tags {
            assert!(!tag.contains(' '));
        }
    }
}

#[test]
fn test_blog_posts_ordering() {
    let posts = get_blog_posts();
    
    // Check that posts are ordered by date (newest first)
    for window in posts.windows(2) {
        let date1 = &window[0].date;
        let date2 = &window[1].date;
        assert!(date1 >= date2, "Posts are not ordered by date");
    }
} 