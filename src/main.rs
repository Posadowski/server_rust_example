use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Received request from client: {}", String::from_utf8_lossy(&buffer));
            let response = if buffer.starts_with(b"GET /") {
                if buffer.starts_with(b"GET / ") || buffer.starts_with(b"GET /index ") {
                    read_file_response("pages/index.html", "text/html")
                } else if buffer.starts_with(b"GET /contact ") {
                    read_file_response("pages/contact.html", "text/html")
                } else if buffer.starts_with(b"GET /about ") {
                    read_file_response("pages/about.html", "text/html")
                } else {
                    not_found_response()
                }
            } else {
                not_found_response()
            };

            if let Err(e) = stream.write(response.as_bytes()) {
                eprintln!("Error sending response: {}", e);
            } else {
                println!("Response sent successfully.");
                println!("Response: {}", response);
            }
        }
        Err(e) => {
            eprintln!("Error reading request: {}", e);
        }
    }
}

fn read_file_response(file_path: &str, content_type: &str) -> String {
    let file_content = read_file(file_path).unwrap_or_else(|_| String::from("404 Not Found"));
    format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", content_type, file_content)
}

fn not_found_response() -> String {
    let not_found_content = read_file("pages/404.html").unwrap_or_else(|_| String::from("404 Not Found"));
    format!("HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n{}", not_found_content)
}

fn read_file(path: &str) -> std::io::Result<String> {
    let not_found_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let mut file = File::open(not_found_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?; // Bind to all interfaces
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}