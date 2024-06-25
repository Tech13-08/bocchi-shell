#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    

    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmedInput = input.trim();
        match trimmedInput {
            "exit 0" => Ok(());
            _ => println!("{}: command not found", trimmedInput);
        }
    }
}
