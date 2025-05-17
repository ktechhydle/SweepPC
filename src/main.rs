use std::env;
use sweeppc::run_cleanup;

fn main() {
    let args = env::args();

    if args.len() > 1 {
        let command: Vec<String> = args.collect();

        if command[1] == "run" {
            run_cleanup()
        }
    } else {
        println!("Usage: sweeppc [command] ... [arg]\n");
        println!("Commands:");
        println!("  run             Run a default cleanup");
        println!("  runtarget [dir] Run a cleanup on a specific directory");
    }
}
