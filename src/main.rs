#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;


fn main() {
    // Uncomment this block to pass the first stage
    let path = env::var("PATH");
    let binding = path.expect("REASON");
    let paths = binding.split(":").collect::<Vec<&str>>();
    
    let builtin = vec!["exit", "echo", "type"];

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
                "exit" => {
                    if trimmed_input.len() > 1 {
                        let exit_content = trimmed_input[1];
                        match exit_content {
                            "0" => return,
                            _ => return,
                        }
                    }
                },
                "echo" => {
                    if trimmed_input.len() > 1 {
                        let echo_content = &trimmed_input[1..].join(" ");
                        println!("{}", echo_content);
                    }
                    else{
                        println!(" ");
                    }
                },
                "type" => {
                    if builtin.contains(&trimmed_input[1]) {
                        println!("{} is a shell builtin", trimmed_input[1]);
                    }
                    else{
                        let mut found = false;
                        for path in paths.iter(){
                            let file_path = format!("{}/{}", *path, trimmed_input[1]);
                            if Path::new(&file_path).exists(){
                                found = true;
                                println!("{} is {}", trimmed_input[1], file_path);
                                break;
                            }
                        }
                        if !found {println!("{}: not found", trimmed_input[1]);}
                    }
                },
                _ => println!("{}: command not found", trimmed_input[0]),
            }
        }
    }
}