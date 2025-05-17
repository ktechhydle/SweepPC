use std::env;
use sweeppc::{run_cleanup, run_cleanup_on_dir};
mod indexing;

fn main() {
    let args = env::args();

    if args.len() > 1 {
        let commands: Vec<String> = args.collect();

        if commands[1] == "run" {
            run_cleanup()
        } else if commands[1] == "runtarget" {
            if commands.len() > 2 {
                run_cleanup_on_dir(&commands[2])
            } else {
                println!("Usage: runtarget [dirname]")
            }
        }
    } else {
        println!("SweepPC Version 1.0");
        println!("Usage: sweeppc [command] ... [arg]\n");
        println!("Commands:");
        println!("  run             Run a default cleanup");
        println!("  runtarget [dir] Run a cleanup on a specific directory");
    }
}
