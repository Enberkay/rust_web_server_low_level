use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;
use std::fs;
use std::path::Path;

fn parse_headers(request: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut headers = std::collections::HashMap::new();
    let mut last_key = None;
    for line in request.lines().skip(1) {
        if line.is_empty() { break; }
        if line.starts_with(' ') || line.starts_with('\t') {
            // Multi-line header (folded)
            if let Some(key) = &last_key {
                if let Some(values) = headers.get_mut(key) {
                    if let Some(last_val) = values.last_mut() {
                        last_val.push_str(line.trim());
                    }
                }
            }
        } else if let Some((k, v)) = line.split_once(":") {
            let key = k.trim().to_ascii_lowercase();
            let value = v.trim().to_string();
            headers.entry(key.clone()).or_insert_with(Vec::new).push(value);
            last_key = Some(key);
        }
    }
    headers
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "unknown".to_string());
    loop {
        let start_time = Instant::now();
        let mut buffer = [0; 2048];
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed by client
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                println!("--- Request from {peer_addr} ---\n{request}");
                // Advanced header parsing
                let headers = parse_headers(&request);
                let request_line = request.lines().next().unwrap_or("");
                let mut parts = request_line.split_whitespace();
                let method = parts.next().unwrap_or("");
                let path = parts.next().unwrap_or("");
                let body = if let Some(index) = request.find("\r\n\r\n") {
                    &request[index + 4..]
                } else {
                    ""
                };
                let (status_line, content_type, response_body) = if method == "GET" && path.starts_with("/static/") {
                    // Static file serving
                    // Prevent directory traversal
                    let rel_path = &path[8..];
                    if rel_path.contains("..") {
                        (
                            "HTTP/1.1 400 BAD REQUEST",
                            "text/plain",
                            "Invalid path".to_string(),
                        )
                    } else {
                        let file_path = format!("public/{}", rel_path);
                        match fs::read(&file_path) {
                            Ok(contents) => {
                                // Guess content type by extension
                                let content_type = match Path::new(&file_path).extension().and_then(|e| e.to_str()) {
                                    Some("html") => "text/html",
                                    Some("css") => "text/css",
                                    Some("js") => "application/javascript",
                                    Some("json") => "application/json",
                                    Some("png") => "image/png",
                                    Some("jpg") | Some("jpeg") => "image/jpeg",
                                    Some("gif") => "image/gif",
                                    Some("svg") => "image/svg+xml",
                                    Some("txt") => "text/plain",
                                    _ => "application/octet-stream",
                                };
                                (
                                    "HTTP/1.1 200 OK",
                                    content_type,
                                    String::from_utf8_lossy(&contents).to_string(),
                                )
                            }
                            Err(_) => (
                                "HTTP/1.1 404 NOT FOUND",
                                "text/plain",
                                "File not found".to_string(),
                            ),
                        }
                    }
                } else {
                    match (method, path) {
                        ("GET", "/") => (
                            "HTTP/1.1 200 OK",
                            "text/html",
                            "<h1>Welcome to Rust low-level server</h1>".to_string(),
                        ),
                        ("GET", "/hello") => ("HTTP/1.1 200 OK", "text/plain", "Hello!".to_string()),
                        ("GET", "/api") => (
                            "HTTP/1.1 200 OK",
                            "application/json",
                            r#"{"message": "This is JSON"}"#.to_string(),
                        ),
                        ("GET", p) if p.starts_with("/user/") => {
                            let user_id = &p["/user/".len()..];
                            if !user_id.is_empty() && user_id.chars().all(|c| c.is_ascii_digit()) {
                                (
                                    "HTTP/1.1 200 OK",
                                    "text/plain",
                                    format!("You requested user {}", user_id),
                                )
                            } else {
                                (
                                    "HTTP/1.1 400 BAD REQUEST",
                                    "text/plain",
                                    "Invalid user ID".to_string(),
                                )
                            }
                        }
                        ("POST", "/submit") => (
                            "HTTP/1.1 200 OK",
                            "text/plain",
                            format!("Received POST data: {}", body.trim()),
                        ),
                        ("PUT", "/update") => (
                            "HTTP/1.1 200 OK",
                            "text/plain",
                            format!("Updated data: {}", body.trim()),
                        ),
                        ("DELETE", "/delete") => (
                            "HTTP/1.1 200 OK",
                            "text/plain",
                            "Delete request processed.".to_string(),
                        ),
                        _ => (
                            "HTTP/1.1 404 NOT FOUND",
                            "text/plain",
                            "404 Not Found".to_string(),
                        ),
                    }
                };
                // Determine if keep-alive is requested
                let connection_header = headers.get("connection").and_then(|v| v.get(0)).map(|v| v.to_ascii_lowercase());
                let keep_alive = match connection_header.as_deref() {
                    Some("keep-alive") => true,
                    Some("close") => false,
                    _ => false,
                };
                let mut response = format!(
                    "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n",
                    response_body.len()
                );
                if keep_alive {
                    response.push_str("Connection: keep-alive\r\n");
                } else {
                    response.push_str("Connection: close\r\n");
                }
                response.push_str("\r\n");
                response.push_str(&response_body);
                let response_time = start_time.elapsed();
                println!(
                    "--- Response to {peer_addr} ---\nStatus: {status_line}\nContent-Type: {content_type}\nContent-Length: {}\nResponse Body: {}\nResponse Time: {:?}\nConnection: {}\n",
                    response_body.len(),
                    if response_body.len() < 256 { &response_body } else { "<body too large>" },
                    response_time,
                    if keep_alive { "keep-alive" } else { "close" }
                );
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Error writing to stream: {e}");
                    break;
                }
                if !keep_alive {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {e}");
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");
    println!("Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
}
