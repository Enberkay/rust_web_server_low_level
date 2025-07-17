use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;

/// Parse HTTP headers from a request string.
/// Supports multi-line (folded) headers and duplicate headers (Vec<String> per key).
pub fn parse_headers(request: &str) -> HashMap<String, Vec<String>> {
    let mut headers = HashMap::new();
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

/// Read HTTP/1.1 chunked transfer encoding body from a TcpStream.
pub fn read_chunked_body(request: &str, stream: &mut TcpStream) -> String {
    let header_end = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(request.len());
    let mut body = String::new();
    let mut buffer = request[header_end..].as_bytes().to_vec();
    loop {
        // Read chunk size line
        let mut size_line = Vec::new();
        while let Some(&b) = buffer.first() {
            buffer.remove(0);
            if b == b'\n' {
                break;
            } else if b != b'\r' {
                size_line.push(b);
            }
        }
        let size_str = String::from_utf8_lossy(&size_line);
        let chunk_size = usize::from_str_radix(size_str.trim(), 16).unwrap_or(0);
        if chunk_size == 0 {
            break;
        }
        // Read chunk data
        while buffer.len() < chunk_size + 2 {
            let mut temp = [0u8; 2048];
            match stream.read(&mut temp) {
                Ok(0) => break,
                Ok(n) => buffer.extend_from_slice(&temp[..n]),
                Err(_) => break,
            }
        }
        if buffer.len() < chunk_size + 2 {
            break;
        }
        let chunk = &buffer[..chunk_size];
        body.push_str(&String::from_utf8_lossy(chunk));
        buffer.drain(..chunk_size + 2); // remove chunk + \r\n
    }
    body
}

/// Decode HTTP Basic Auth header. Returns (username, password) if valid, else None.
pub fn decode_basic_auth(header: &str) -> Option<(String, String)> {
    // header is expected to be "Basic base64string"
    let b64 = header.strip_prefix("Basic ")?.trim();
    let decoded = base64_decode(b64)?;
    let decoded_str = String::from_utf8(decoded).ok()?;
    let mut parts = decoded_str.splitn(2, ':');
    let user = parts.next()?.to_string();
    let pass = parts.next()?.to_string();
    Some((user, pass))
}

/// Minimal base64 decoder (supports standard base64, no padding check)
pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut val = 0u32;
    let mut valb = -8i32;
    let mut out = Vec::new();
    for c in input.chars() {
        if c == '=' { break; }
        let idx = TABLE.iter().position(|&x| x == c as u8)? as u32;
        val = (val << 6) | idx;
        valb += 6;
        if valb >= 0 {
            out.push(((val >> valb) & 0xFF) as u8);
            valb -= 8;
        }
    }
    Some(out)
} 