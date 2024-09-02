use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    if let Ok(_size) = stream.read(&mut buffer) {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nhello, world!";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the add-on...");
    let username = std::env::var("USERNAME");

    if let Err(error) = username {
        return Err(Box::new(error));
    }
    let username = username.unwrap();

    println!("Hello, {username}!");

    let tcp_listener = TcpListener::bind("0.0.0.0:9128")?;
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
