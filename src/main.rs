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
        let trimmed_input = input.trim().split_whitespace().collect::<Vec<&str>>();
        if trimmed_input.len() > 0 {
            match trimmed_input[0] {
                "exit 0" => return,
                "echo" => {
                    let echo_content = if trimmed_input.len() > 0 {&trimmed_input[1..].join(" ")} else {""};
                    echo(echo_content);
                },
                _ => println!("{}: command not found", &trimmed_input[0]),
            }
        }
    }
}


fn echo(content: &str) {
    println!("{}", content);
}