use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 2048]; // เพิ่มขนาด buffer รองรับ body

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("--- Request ---\n{request}");

            let request_line = request.lines().next().unwrap_or("");
            let mut parts = request_line.split_whitespace();
            let method = parts.next().unwrap_or("");
            let path = parts.next().unwrap_or("");

            // อ่าน body (ง่ายๆ โดยหาจาก \r\n\r\n แล้วเอาส่วนหลัง)
            let body = if let Some(index) = request.find("\r\n\r\n") {
                &request[index + 4..]
            } else {
                ""
            };

            let (status_line, content_type, response_body) = match (method, path) {
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
            };

            let response = format!(
                "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{response_body}",
                response_body.len()
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error reading stream: {e}");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");
    println!("Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
}
