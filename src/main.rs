#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn handle_not_found(input: &str) {
    println!("{}: not found", input);
}

fn handle_exit(input: Vec<&str>) {
    if input.len() != 2 || input[0] != "exit" || input[1].parse::<i32>().is_err() {
        handle_not_found(input[0]);
        return;
    }

    exit(input[1].parse::<i32>().ok().unwrap());
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
    
        match input[0] {
            "exit" => handle_exit(input),
            _ => handle_not_found(input[0]),
        }
    }

}
