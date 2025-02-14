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

    fn pwd_no_line(&self) {
        match self.wd.to_str() {
            Some(wd_str) => print!("{}", wd_str),
            None => print!("cannot get wd"),
        }
    }

    fn pwd(&self) {
        self.pwd_no_line();
        println!();
    }

    // main function for cd.
    // returns true if successfully cd to new wd, false o/w.
    fn _cd(&mut self, new_wd_str: &str) -> bool {
        let prefix: &str;
        if new_wd_str.starts_with("./") {
            prefix = "./";
            // println!("starts with ./");
        } else if new_wd_str.starts_with("../") {
            prefix = "../";
            // println!("starts with ../");

            if self.wd.to_str() == Some("/") {
                println!("Root has no parent.");
                return false;
            }

            // go to parent
            self.wd = self.wd.parent().unwrap().to_path_buf();
            
        } else if new_wd_str.starts_with("~") {
            prefix = if new_wd_str.starts_with("~/") {"~/"} else {"~"};
            let home_result = std::env::var("HOME");
            match home_result {
                Ok(home_str) => {
                    self.wd = Path::new(&home_str).to_path_buf();
                },
                Err(_) => {
                    println!("cd: No home.");
                    return false;
                },
            }

        } else {
            let new_wd = Path::new(new_wd_str);
            match new_wd_str != ".." && new_wd_str != "." && new_wd.exists() {
                true => {
                    self.wd = new_wd.to_path_buf();
                    return true;
                },
                false => {
                    println!("cd: {}: No such file or directory", new_wd_str);
                    return false;
                },
            }
        }

        // extract the rest.
        let rest_new_wd_str = new_wd_str.strip_prefix(prefix).unwrap_or_default();
        if rest_new_wd_str.starts_with("./") || rest_new_wd_str.starts_with("../") || rest_new_wd_str.starts_with("~") {
            return self._cd(rest_new_wd_str);
        }

        // if the rest doesn't start with ../ or ./ or ~, then the rest is under the current dir.
        let new_wd = match rest_new_wd_str {
            "" => self.wd.clone(),
            _ => self.wd.join(rest_new_wd_str),
        };

        return self._cd(new_wd.to_str().unwrap());

    }

    fn cd(&mut self, new_wd_str: &str) {
        let rollback_wd = self.wd.clone();
        if !self._cd(new_wd_str) {
            self.wd = rollback_wd;
        } 
    }
}

fn parse_args(input: &str) -> Option<Vec<String>> {
    let mut dest = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '\\' if !in_single_quote && !in_double_quote => {
                // Escape: take the next character literally
                if let Some(&escaped_char) = chars.peek() {
                    current.push(escaped_char);
                    chars.next(); // Consume the escaped character
                }
            }
            '\\' if !in_single_quote && in_double_quote => {
                if let Some(&escaped_char) = chars.peek() {
                    if ['\\', '$', '"'].contains(&escaped_char) {
                        current.push(escaped_char);
                        chars.next(); // Consume the escaped character
                    } else {
                        current.push(c);
                    }
                }
            }
            '\'' if !in_double_quote => {
                // Toggle single-quote mode
                in_single_quote = !in_single_quote;
            }
            '\"' if !in_single_quote => {
                // Toggle double-quote mode
                in_double_quote = !in_double_quote;
            }
            ' ' if !in_single_quote && !in_double_quote => {
                // Space outside quotes: finalize the current argument
                if !current.is_empty() {
                    dest.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                // Any other character: add to the current argument
                // println!("pushing {}", c);
                current.push(c);
            }
        }
    }

    // Push the last argument if it's non-empty
    if !current.is_empty() {
        dest.push(current);
    }

    // Ensure all quotes are closed
    if in_single_quote && !in_double_quote {
        println!("bad format: unbalanced quotes");
        return None;
    }

    return Some(dest);
}


const BUILTIN_CMDS: [&str; 5] = ["cd", "pwd", "exit", "echo", "type"];

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

    let mut wd = Wd::new();

    loop {
        // wd.pwd_no_line(); print!(" ");
        print!("$ ");
        io::stdout().flush().unwrap();
    
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let input_opt = parse_args(input.trim());
        let input = match input_opt {
            None => continue,
            Some(parsed_input) => parsed_input,
        };
        let input = input.iter().map(String::as_str).collect::<Vec<_>>();
        
        // println!("input: {:#?}", input);
        

        if input.len() <= 0 {
            continue;
        }
    
        match input[..] {
            ["pwd"] => wd.pwd(),
            ["cd", new_wd] => wd.cd(new_wd),
            ["echo", ..] => println!("{}", input[1..].join(" ")),
            ["exit", number] => handle_exit(number),
            ["type", cmd] => handle_type(cmd),
            [cmd, ..] => handle_execute(cmd, input[1..].to_vec()),
            _ => continue,
        }
    }

}
