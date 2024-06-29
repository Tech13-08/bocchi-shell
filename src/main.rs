#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};


fn main() {
    // Uncomment this block to pass the first stage
    let path = env::var("PATH");
    let binding = path.expect("REASON");
    let paths = binding.split(":").collect::<Vec<&str>>();
    
    let builtin = vec!["exit", "echo", "type", "pwd", "cd"];
    

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
                    // Handle error codes at some point
                    // if trimmed_input.len() > 1 {
                    //     let exit_content = trimmed_input[1];
                    //     match exit_content {
                    //         _ => return,
                    //     }
                    // }
                    return;
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
                    if trimmed_input.len() > 1 {
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
                    }
                    else{
                        println!("Missing 1 argument");
                    }
                },
                "pwd" => println!("{}", get_pwd()),
                "cd" => {
                    if trimmed_input.len() > 1 {
                        let mut new_path_string = get_pwd();
                        let mut parts = trimmed_input[1].split("/").collect::<Vec<&str>>();
                        parts.retain(|x| *x != "");
                        let mut relative = false;
                        for part in parts.iter(){
                            if part.to_owned() == "."{
                                relative = true;
                            }
                            else if part.to_owned() == ".."{
                                let path_vec = new_path_string.as_str().split("/").collect::<Vec<&str>>();
                                new_path_string = (path_vec[..path_vec.len()-1].join("/")).to_owned();
                                relative = true;
                            }
                            else{
                                let home = get_home();
                                if relative {new_path_string.push_str(format!("/{}",(if part.to_owned() == "~" {home.as_str()} else {part})).as_str());}
                                else {
                                    new_path_string = format!("/{}",if part.to_owned() == "~" {home.as_str()} else {part});
                                    relative = true;
                                }
                            }
                        }
                        let new_path = Path::new(new_path_string.as_str());
                        if new_path.exists(){
                            let _ = env::set_current_dir(&new_path);
                        }
                        else{
                            println!("cd: {}: No such file or directory", trimmed_input[1]);
                        }   
                    }
                    else{
                        println!("Missing 1 argument");
                    }
                },
                _ => {
                    let mut found = false;
                    for path in paths.iter(){
                        let file_path = format!("{}/{}", *path, trimmed_input[0]);
                        if Path::new(&file_path).exists(){
                            found = true;
                            let output = Command::new(trimmed_input[0])
                                // Tell the OS to record the command's output
                                .stdout(Stdio::piped())
                                .args(&trimmed_input[1..])
                                // execute the command, wait for it to complete, then capture the output
                                .output()
                                // Blow up if the OS was unable to start the program
                                .unwrap();

                            // extract the raw bytes that we captured and interpret them as a string
                            let stdout = String::from_utf8(output.stdout).unwrap();
                            println!("{}", stdout.trim());
                            break;
                        }
                    }
                    

                    if !found {println!("{}: command not found", trimmed_input[0]);}
                },
            }
        }
    }
}



fn get_pwd() -> String {
    return env::current_dir().unwrap().into_os_string().into_string().unwrap();
}

fn get_home() -> String {
    return env::var("HOME").unwrap();
}