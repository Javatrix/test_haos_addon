use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting the add-on...");
    let username = std::env::var("USERNAME");

    if let Err(error) = username {
        return Err(Box::new(error));
    }
    let username = username.unwrap();

    println!("Hello, {username}!");

    let input = std::io::read_to_string(stdin());
    if let Err(error) = input {
        return Err(Box::new(error));
    }
    let input = input.unwrap();

    println!("GOT THE INPUT!\n\t: {input}");

    Ok(())
}
