use std::net::TcpListener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the add-on...");
    let username = std::env::var("USERNAME");

    if let Err(error) = username {
        return Err(Box::new(error));
    }
    let username = username.unwrap();

    println!("Hello, {username}!");

    let tcp_listener = TcpListener::bind("127.0.0.1:9128")?;
    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        println!("Estabilished connection: {}", stream.local_addr().unwrap());
    }

    Ok(())
}
