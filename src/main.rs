use colored::Colorize;
use std::env;
use sweeppc::{run_cleanup, run_cleanup_on_dir, run_cleanup_on_temp};

fn main() {
    let args = env::args();

    if args.len() > 1 {
        let commands: Vec<String> = args.collect();

        if commands[1] == "run" {
            run_cleanup();
        } else if commands[1] == "runtarget" {
            if commands.len() > 2 {
                run_cleanup_on_dir(&commands[2])
            } else {
                println!("Usage: runtarget [dirname]")
            }
        } else if commands[1] == "cleantemp" {
            run_cleanup_on_temp();
        }
    } else {
        println!("\n{}", "SweepPC Version 1.0".green().bold());
        println!(
            "{}\n",
            "See https://github.com/ktechhydle/SweepPC for command usage"
                .italic()
                .blue()
        );
        println!(
            "{} {} ... {}",
            "Usage -> sweeppc".yellow().bold(),
            "[command]".green(),
            "[arg]".green()
        );
        println!("{}", "Commands:".white());
        println!("{}", "  run             Run a default cleanup".italic());
        println!(
            "{}",
            "  runtarget [dir] Run a cleanup on a specific directory".italic()
        );
        println!(
            "{}",
            "  cleantemp       Cleanup temp files and caches".italic()
        );
        println!(
            "\n{}\n",
            "Before using this command, make sure to close any open programs!".red()
        );
    }
}
