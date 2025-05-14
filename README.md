# Rust Web Server with Axum

A modern web server built with Rust and the Axum framework, featuring dynamic content rendering, pagination, and a powerful search system.

## Features

- **Dynamic Page Rendering**: Server-side rendering using Tera templates
- **Blog System**: Complete blog functionality with posts, tags, and excerpts
- **Error Handling**:
  - Consistent error page styling
  - Custom error messages for different scenarios
  - HTTP status code display
  - User-friendly error descriptions
  - Easy navigation back to home
- **Pagination Support**:
  - Configurable results per page (3, 5, or 10 items)
  - Dynamic page navigation
  - Responsive pagination controls
  - Smooth page transitions
  - Total results and pages display
- **Comment System**:
  - In-memory comment storage
  - Real-time comment updates
  - Modern comment form with validation
  - Responsive comment layout
  - Timestamp and author display
- **Advanced Search**:
  - Real-time search as you type
  - Search across multiple fields (title, content, tags)
  - Modern mega menu interface
  - Instant results with smooth animations
  - Responsive design for all devices
- **Modern UI/UX**:
  - Clean, responsive design
  - Dark mode by default
  - Smooth animations and transitions
  - Accessible navigation
  - Mobile-friendly layout

## Project Structure

```
.
├── src/
│   ├── main.rs           # Main application code
│   ├── models.rs         # Data models
│   ├── handlers.rs       # Request handlers
│   ├── state.rs          # Application state
│   └── data.rs           # Sample data
├── static/
│   └── styles.css        # Global styles
├── templates/
│   ├── base.html         # Base template
│   ├── home.html         # Home page
│   ├── about.html        # About page
│   ├── blog.html         # Blog listing
│   ├── post.html         # Individual post
│   ├── error.html        # Generic error page
│   └── components/       # Reusable components
│       └── search.html   # Search component
└── Cargo.toml           # Project dependencies
```

## Dependencies

- **axum**: Web framework
- **tokio**: Async runtime
- **tera**: Template engine
- **tower-http**: HTTP utilities
- **chrono**: Date and time handling
- **serde**: Serialization/deserialization
- **uuid**: Unique ID generation

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/web_server.git
   cd web_server
   ```

2. Build and run the server:
   ```bash
   cargo run
   ```

3. Visit `http://localhost:3000` in your browser

## API Endpoints

- `GET /`: Home page
- `GET /about`: About page
- `GET /blog`: Blog listing
- `GET /blog/{id}`: Individual blog post
- `GET /api/time`: Current server time
- `GET /api/posts`: List of all blog posts (supports pagination)
- `GET /api/search`: Search posts (query parameters: `q` for search term, `in` for search fields)
- `GET /api/posts/{id}/comments`: Get comments for a post
- `POST /api/posts/{id}/comments`: Add a comment to a post

## Pagination Features

The blog system now includes powerful pagination capabilities:

- **Configurable Results**:
  - Choose between 3, 5, or 10 items per page
  - Dynamic page size adjustment
  - Persistent page size selection
- **Navigation Controls**:
  - Previous/Next page buttons
  - Page number indicators
  - Current page highlighting
  - Total pages display
- **User Experience**:
  - Smooth page transitions
  - Maintains scroll position
  - Responsive design
  - Clear visual feedback
- **Technical Implementation**:
  - Server-side pagination
  - Efficient data loading
  - Optimized performance
  - Clean URL structure

## Search Features

The search system provides a powerful way to find content across the blog:

- **Real-time Search**: Results update as you type
- **Field Filtering**: Search in specific fields:
  - Title
  - Content
  - Tags
- **Modern Interface**:
  - Dropdown mega menu
  - Smooth animations
  - Responsive design
  - Custom scrollbar
  - Blur effects
- **Result Display**:
  - Clean, focused results showing only title and excerpt
  - Optimized for quick scanning
  - Smooth hover effects
  - Stable positioning without flickering

## Comment System

The comment system allows visitors to interact with blog posts:

- **Features**:
  - Add comments to any blog post
  - Real-time comment updates
  - Author name and timestamp display
  - Responsive comment layout
  - Form validation
- **Technical Details**:
  - In-memory comment storage
  - RESTful API endpoints
  - Asynchronous comment loading
  - Modern UI with animations
  - Error handling

## Error Handling

The application implements a robust error handling system:

- **Generic Error Page**:
  - Consistent styling across all error types
  - Clear error code display
  - Descriptive error messages
  - Easy navigation back to home
  - Maintains site branding
- **Error Types**:
  - 404 Not Found (missing pages/posts)
  - 500 Internal Server Error
  - Custom error messages for specific scenarios
- **Implementation**:
  - Centralized error rendering
  - Type-safe error handling
  - Proper HTTP status codes
  - User-friendly messages
  - Maintainable error system

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.