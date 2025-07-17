# Rust Low-Level Web Server

A minimal, low-level HTTP server written in Rust, designed to demonstrate manual handling of TCP streams, HTTP parsing, and extensible low-level features.

## Changelog

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

## Usage

```sh
cargo run
```

The server listens on 127.0.0.1:7878 by default.

## Roadmap
- HTTP/1.1 keep-alive support
- Static file serving
- Advanced header parsing
- Chunked transfer encoding
- HTTPS (TLS) support
- Graceful shutdown
- Rate limiting and basic authentication
- Unit and integration tests