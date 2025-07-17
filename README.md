# Rust Low-Level Web Server

A minimal, low-level HTTP server written in Rust, designed to demonstrate manual handling of TCP streams, HTTP parsing, and extensible low-level features.

## Changelog

### v0.6.0
- Added support for HTTP/1.1 chunked transfer encoding in request body parsing.
  - If Transfer-Encoding: chunked is present, the server parses the request body as chunked.
  - Used for POST/PUT requests with chunked bodies.
  - Follows HTTP/1.1 chunked encoding rules (hex size, chunk data, CRLF, ends with 0 chunk).

### v0.5.0
- Improved HTTP header parsing:
  - Supports multi-line (folded) headers according to HTTP/1.x spec.
  - Stores headers as HashMap<String, Vec<String>> for case-insensitive keys and duplicate headers.
  - Enables robust handling of complex and repeated headers.

### v0.4.0
- Added static file serving for GET requests to /static/*
  - Files are served from the public/ directory.
  - Content-Type is determined by file extension (e.g., .html, .css, .js, .png, etc.).
  - Directory traversal (../) is blocked for security.
  - Returns 404 if the file does not exist.

### v0.3.0
- Added HTTP/1.1 keep-alive (persistent connection) support.
  - Parses the Connection header from incoming requests.
  - If Connection: keep-alive is present, the server keeps the TCP connection open and handles multiple requests per connection.
  - If Connection: close or no keep-alive, the server closes the connection after one request.
  - Response includes the appropriate Connection header.
  - This improves efficiency for clients making multiple requests.

### v0.2.0
- Added detailed low-level logging for each HTTP request and response.
  - Logs client IP address, HTTP method, path, headers, body, response code, and response time to stdout.
  - Useful for debugging, performance analysis, and understanding raw HTTP traffic.

### v0.1.0
- Initial release: basic multi-threaded HTTP server using `std::net::TcpListener` and `TcpStream`.
- Supports basic routing for GET, POST, PUT, DELETE methods.
- Parses HTTP requests manually (no external libraries).
- Handles simple request body extraction and static response generation.
- Spawns a new thread per connection for concurrency.

### v0.7.0
- Added graceful shutdown: server handles Ctrl+C (SIGINT), stops accepting new connections, and waits for all active connections to finish before exiting.
- Refactored code into modules: HTTP utilities are now in src/http.rs, main server logic in src/main.rs.
- Uses only the `ctrlc` crate for Ctrl+C handling; all other logic is pure std.

### v0.8.0
- Added basic in-memory rate limiting per IP address.
  - Allows up to 5 requests per 3 seconds per IP.
  - Returns HTTP 429 Too Many Requests if the limit is exceeded.
  - Implemented using only the Rust standard library (no external crates).

### v0.9.0
- Added HTTP Basic Authentication for /api and /protected routes.
  - Requires username `admin` and password `password123`.
  - Returns 401 Unauthorized with WWW-Authenticate header if missing or invalid.
  - Manual base64 decoding, no external crate used.

## Usage

```sh
cargo run
```