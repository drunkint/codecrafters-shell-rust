#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

const BUILTIN_CMDS: [&str; 3] = ["exit", "echo", "type"];

fn handle_not_found(input: &str) {
    if input.is_empty() {
        return;
    }
    
    println!("{}: not found", input);
}

fn handle_exit(num_str: &str) {
    if let Some(num) = num_str.parse::<i32>().ok() {
        exit(num);
    }
        
    println!("cannot exit with error code {}", num_str);
}

fn handle_type(cmd: &str) {
    if BUILTIN_CMDS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    handle_not_found(cmd);
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let input: Vec<&str> = input.trim().split(' ').collect();

        if input.len() <= 0 {
            continue;
        }
    
        match input[..] {
            ["echo", ..] => println!("{}", input[1..].join(" ")),
            ["exit", number] => handle_exit(number),
            ["type", cmd] => handle_type(cmd),
            [cmd, ..] => handle_not_found(cmd),
            _ => continue,
        }
    }

}
