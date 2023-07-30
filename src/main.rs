use std::io;
use std::io::Write;

fn main() {

    println!("--- Password Manager ---");
    print!("Master Key: ");
    io::stdout().flush().unwrap();

    let mut key: String = String::new();

    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line.");

    println!("Key Entered: {key}");

    
}
