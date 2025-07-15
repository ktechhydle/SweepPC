use simply_colored::*;
use std::io::{self, Write};

pub fn get_yes_or_no(prompt: &str) -> bool {
    loop {
        print!("🤔 {ITALIC}{prompt} (y/n):{RESET} ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'."),
        }
    }
}
