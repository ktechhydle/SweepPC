use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to SweepPC. Let's clean your computer!");
    println!("Type 'help' for usage instructions or 'exit' to exit.");

    loop {
        let mut input: String = String::new();
        print!(">>> ");

        let _ = stdout().flush();

        stdin().read_line(&mut input).expect("Not a valid command");

        let command: &Vec<&str> = &input.trim().split(' ').collect();

        if command.len() > 0 {
            if command[0] == "exit" {
                break;
            } else if command[0] == "help" {
                println!("usage: [command] ... [arg]");
                println!("  help: see this again");
                println!("  exit: exit sweeppc");
                println!("  trim: clean up memory");
                println!("  scan [arg]: scan the specified target");
                println!("      [arg] temp: scan for temp/cache dirs");
                println!("      [arg] old: scan for old and large files");
                println!("      [arg] danger: scan for malicious process");
                println!("  clean [arg]: clean up the specified target");
                println!("      [arg] temp: clean up temp/cache dirs");
                println!("      [arg] old: clean up old and large files dirs");
            } else if command[0] == "scan" {
                if command.len() > 1 {
                } else {
                    println!("\n");
                    println!("usage: scan [arg]");
                    println!("  [arg] temp: scan for temp/cache dirs");
                    println!("  [arg] old: scan for old and large files");
                    println!("  [arg] danger: scan for malicious process");
                    println!("\n");
                }
            } else {
                println!("\n");
                println!("'{}' is not a valid command.", command[0]);
                println!("\n");
            }
        }
    }
}
