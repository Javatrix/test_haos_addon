fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the add-on...");
    let username = std::env::var("USERNAME");

    if let Err(error) = username {
        return Err(Box::new(error));
    }
    let username = username.unwrap();

    println!("Hello, {username}!");

    Ok(())
}
