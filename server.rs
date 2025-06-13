use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);

            // พิมพ์ request ทั้งหมด
            println!("--- Incoming Request ---\n{request}");

            // เอาแค่บรรทัดแรก (request line)
            let request_line = request.lines().next().unwrap_or("");
            let mut parts = request_line.split_whitespace();

            let method = parts.next().unwrap_or("");
            let path = parts.next().unwrap_or("");

            // Routing แบบง่าย
            let (status_line, content_type, body) = match (method, path) {
                ("GET", "/") => (
                    "HTTP/1.1 200 OK",
                    "text/html",
                    "<h1>Welcome to the Rust low-level server</h1>",
                ),
                ("GET", "/hello") => (
                    "HTTP/1.1 200 OK",
                    "text/plain",
                    "Hello there!",
                ),
                ("GET", "/api") => (
                    "HTTP/1.1 200 OK",
                    "application/json",
                    r#"{"message": "This is JSON"}"#,
                ),
                _ => (
                    "HTTP/1.1 404 NOT FOUND",
                    "text/plain",
                    "404 Not Found",
                ),
            };

            let response = format!(
                "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Error reading stream: {e}");
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to address");

    println!("Server running at http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }
}
