use std::error::Error;

/// Extracts the body from a raw HTTP response
pub fn extract_body(raw_response: &str) -> Result<String, Box<dyn Error>> {
    // Split headers and body
    let (headers, body) = raw_response
        .split_once("\r\n\r\n")
        .ok_or("Invalid HTTP response: no header/body separator")?;

    if is_chunked(headers) {
        decode_chunked(body)
    } else {
        Ok(body.to_string())
    }
}

/// Checks if response uses chunked transfer encoding
fn is_chunked(headers: &str) -> bool {
    headers
        .to_ascii_lowercase()
        .contains("transfer-encoding: chunked")
}

/// Chunked decoding 
fn decode_chunked(body: &str) -> Result<String, Box<dyn Error>> {
    let mut remaining = body;
    let mut decoded = String::new();

    loop {
        // Read chunk size line
        let (size_line, rest) = remaining
            .split_once("\r\n")
            .ok_or("Invalid chunked encoding: missing size line")?;

        let size_hex = size_line
            .split(';') // ignore chunk extensions
            .next()
            .ok_or("Invalid chunk size")?
            .trim();

        // Parse chunk size from hex
        let size = usize::from_str_radix(size_hex, 16)?;

        if size == 0 {
            break;
        }

        // Ensure enough data for chunk + CRLF
        if rest.len() < size + 2 {
            return Err("Chunk shorter than declared size".into());
        }

        // Append chunk data
        let chunk = &rest[..size];
        decoded.push_str(chunk);

        // Move to next chunk
        let after_chunk = &rest[size..];

        // Checks for CRLF after chunk
        if !after_chunk.starts_with("\r\n") {
            return Err("Missing CRLF after chunk".into());
        }

        remaining = &after_chunk[2..];
    }

    Ok(decoded)
}