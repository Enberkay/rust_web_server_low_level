use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Received request:\n{}", request);

            // เราสามารถ parse method/path ได้อย่างง่าย (ยังไม่ robust)
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from Rust!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server is running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
