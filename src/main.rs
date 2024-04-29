use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            println!(
                "Received request from client: {}",
                String::from_utf8_lossy(&buffer)
            );
            let endpoint_to_check = String::from_utf8_lossy(&buffer);
            let mut trimmed_path = String::new();
            if let Some(path) = endpoint_to_check.splitn(2, "GET ").nth(1) {
                if let Some(end) = path.find(" HTTP/") {
                    trimmed_path = path[..end].to_string(); // Assign value inside the block
                    println!("{}", trimmed_path);
                } else {
                    println!("String not found 'HTTP/'");
                }
            } else {
                println!("String not found 'GET '");
            }

            let response = if trimmed_path.starts_with("/") {
                if trimmed_path.len() > "/".len() {
                    // Zliczanie za pomocą metody matches()
                    let count_matches = trimmed_path.matches('/').count();
                    println!("Liczba znaków '/' (matches()): {}", count_matches);
                    if count_matches > 1 {
                        if trimmed_path.ends_with("/") {
                            // Jeśli tak, wywołaj funkcję z odpowiednimi argumentami
                            let file_path = format!("pages{}index.html", trimmed_path);
                            read_file_response(&file_path, "text/html")
                        } else {
                            let path_to_find = format!("pages{}.html", trimmed_path);
                            read_file_response(&path_to_find, "text/html")
                        }
                    }else {
                        let path_to_find = format!("pages{}.html", trimmed_path);
                        read_file_response(&path_to_find, "text/html")
                    }
                } else {
                    //let path_to_find = format!("pages{}.html", trimmed_path);
                    read_file_response("pages/index.html", "text/html")
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
    println!("try to find {}", file_path);
    let file_content = read_file(file_path).unwrap_or_else(|_| not_found_response());
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
        content_type, file_content
    )
}

fn not_found_response() -> String {
    let not_found_content =
        read_file("pages/en/404.html").unwrap_or_else(|_| String::from("404 Not Found"));
    format!("{}", not_found_content)
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
