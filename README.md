# Rust Web Server

A modern web server built with Rust using Axum framework, featuring dynamic routing, template rendering, and real-time updates.

## Features

- 🚀 Built with Rust and Axum framework
- 📝 Dynamic page rendering using Tera templates
- 🎨 Modern, responsive UI with CSS
- ⚡ Real-time updates using JavaScript
- 🔄 API endpoint for current time
- 📱 Mobile-friendly design

## Project Structure

```
web_server/
├── src/
│   └── main.rs          # Main application code
├── templates/
│   ├── base.html        # Base template with common layout
│   ├── home.html        # Home page template
│   ├── about.html       # About page template
│   └── 404.html         # Error page template
├── static/
│   └── styles.css       # Global styles
└── Cargo.toml           # Project dependencies
```

## Architecture

The application uses a layered architecture:

1. **Router Layer**: Handles HTTP routing using Axum
2. **Template Layer**: Renders dynamic content using Tera templates
3. **State Management**: Shared application state using Arc for thread safety
4. **Static File Serving**: Serves static assets using tower-http

## Dependencies

- `axum`: Web framework
- `tokio`: Async runtime
- `tera`: Template engine
- `tower-http`: Static file serving
- `serde`: Serialization/deserialization
- `chrono`: Time handling

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Running Locally

1. Clone the repository:
   ```bash
   git clone git@github.com:drewwalton19216801/web_server.git
   cd web_server
   ```

2. Build and run the server:
   ```bash
   cargo run
   ```

3. Access the application:
   - Main page: http://127.0.0.1:3000
   - About page: http://127.0.0.1:3000/about
   - Time API: http://127.0.0.1:3000/api/time

## Development

The server runs on port 3000 by default. The application uses:
- Hot reloading for templates
- Real-time updates for the current time
- Responsive design for all screen sizes

## API Endpoints

- `GET /`: Home page
- `GET /about`: About page
- `GET /api/time`: Returns current time in JSON format
- `GET /*`: 404 page for undefined routes

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the MIT License. 