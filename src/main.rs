use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read from client");

    // this line converts the data in the buffer into a utf8 enccoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received requet: {}", request);

    let responce = "Hello, Client".as_bytes();

    stream.write(responce).expect("Failed to write responce");
}

// Entry Point
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // standard error stream
                eprintln!("Failed to establish connection : {}", e);
            }
        }
    }
}
