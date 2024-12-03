#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn handle_not_found(input: &str) {
    println!("{}: not found", input);
}

fn handle_exit(num_str: &str) {
    if let Some(num) = num_str.parse::<i32>().ok() {
        exit(num);
    }
        
    println!("cannot exit with error code {}", num_str);
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
            _ => handle_not_found(input[0]),
        }
    }

}
