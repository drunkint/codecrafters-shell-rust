#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::{Path, PathBuf}, process::{exit, Command}};

struct Wd {
    wd: PathBuf,
}

impl Wd {
    fn new() -> Self {
        if let Ok(wd) = env::current_dir() {
            Self { wd }
        } else {
            panic!("Failed to get the current directory");
        }
    }

    fn pwd(&self) {
        match self.wd.to_str() {
            Some(wd_str) => println!("{}", wd_str),
            None => println!("cannot get wd"),
        }
    }
}

const BUILTIN_CMDS: [&str; 4] = ["pwd", "exit", "echo", "type"];

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

fn get_executable_file(cmd: &str) -> Option<String> {
    if let Ok(paths_str) = std::env::var("PATH") {
        if let Some(found_path_buf) = paths_str
            .split(':')
            .map(Path::new)
            .map(|path| path.join(cmd))
            .find(|file| file.exists())
        {
            return Some(found_path_buf.to_str().unwrap_or_default().to_owned());
        }
    }

    None
}

fn handle_type(cmd: &str) {
    if BUILTIN_CMDS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
        return;
    }

    if let Some(found_path) = get_executable_file(cmd) {
        println!("{} is {}", cmd, found_path);
        return;
    }

    handle_not_found(cmd);
}

fn handle_execute(cmd_str: &str, args: Vec<&str>) {
    if let Some(cmd_full_path) = get_executable_file(cmd_str) {
        let output = Command::new(cmd_full_path)
            .args(args)
            .output()
            .unwrap();

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        return;
    } 

    handle_not_found(cmd_str);
}

fn main() {

    let wd = Wd::new();

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
            ["pwd"] => wd.pwd(),
            ["echo", ..] => println!("{}", input[1..].join(" ")),
            ["exit", number] => handle_exit(number),
            ["type", cmd] => handle_type(cmd),
            [cmd, ..] => handle_execute(cmd, input[1..].to_vec()),
            _ => continue,
        }
    }

}
