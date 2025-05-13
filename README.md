# Rust Web Server with Axum

A modern web server built with Rust and the Axum framework, featuring dynamic content rendering, real-time updates, and a powerful search system.

## Features

- **Dynamic Page Rendering**: Server-side rendering using Tera templates
- **Real-time Updates**: Live time display with JavaScript
- **Blog System**: Complete blog functionality with posts, tags, and excerpts
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
│   └── main.rs           # Main application code
├── static/
│   └── styles.css        # Global styles
├── templates/
│   ├── base.html         # Base template
│   ├── home.html         # Home page
│   ├── about.html        # About page
│   ├── blog.html         # Blog listing
│   ├── post.html         # Individual post
│   ├── 404.html          # Error page
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
- `GET /api/posts`: List of all blog posts
- `GET /api/search`: Search posts (query parameters: `q` for search term, `in` for search fields)

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
  - Post title with link
  - Publication date
  - Author information
  - Content excerpt
  - Related tags

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 