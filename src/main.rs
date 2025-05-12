use std::io::{stdin, stdout, Write};

fn main() {
    println!("Welcome to SweepPC. Let's clean your computer!");
    println!("Type 'help' for usage instructions or 'exit' to exit.");

    loop {
        let mut input: String = String::new();
        print!("> ");

        let _ = stdout().flush();

        stdin().read_line(&mut input).expect("Not a valid command");

        if &input.trim().to_string() == "exit" {
            break;
        } else if &input.trim().to_string() == "help" {
            println!("usage: [command] ... [arg]");
            println!("  help: see this again");
            println!("  exit: exit sweeppc");
            println!("  trim: clean up memory");
            println!("  clean [arg]: clean up the specified argument");
            println!("      [arg] temp: clean up temp/cache dirs");
            println!("      [arg] old: clean up old and large files dirs");
        } else {
            println!("Not a valid command.");
        }
    }
}
