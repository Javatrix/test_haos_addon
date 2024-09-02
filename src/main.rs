use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the add-on...");
    let username = std::env::var("USERNAME");

    if let Err(error) = username {
        return Err(Box::new(error));
    }
    let username = username.unwrap();

    println!("Hello, {username}!");

    let test_file = fs::write("test.txt", "Test message from the best Rust add-on ever :3");

    println!("{:?}", test_file);

    Ok(())
}
