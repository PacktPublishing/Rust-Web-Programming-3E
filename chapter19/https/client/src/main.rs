use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Set up the TLS connector
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // This bypasses certificate validation
        .build()
        .expect("Failed to create TLS connector");

    // Connect to the Actix server via TCP (HTTPS)
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

    // Wrap the TCP stream in a TLS connection
    let mut stream = connector
        .connect("127.0.0.1", stream)
        .expect("Failed to establish TLS connection");

    // Craft an HTTPS POST request with a JSON body
    let json_body = r#"{"message": "Hello, Actix!"}"#;
    let request = format!(
        "POST /api/data HTTP/1.1\r\n\
        Host: 127.0.0.1\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        json_body.len(),
        json_body
    );

    // Send the request over the TLS connection
    stream.write_all(request.as_bytes()).expect("Failed to write to stream");

    // Read the response from the server
    let mut buffer = [0; 512];
    let mut response = String::new();

    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        response.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
    }

    // Print the response
    println!("Response: {}", response);
}
