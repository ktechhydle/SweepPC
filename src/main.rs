use clap::{CommandFactory, Parser, Subcommand};
use sweeppc::{run_cleanup, run_cleanup_on_dir, run_cleanup_on_temp};

#[derive(Parser)]
#[command(name = "sweeppc", version = env!("CARGO_PKG_VERSION"), about = "SweepPC Cleaning Utility")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run a default cleanup on your entire computer")]
    Run,
    #[command(about = "Run a cleanup on the specified directory")]
    Runtarget { dir: String },
    #[command(about = "Run a cleanup on your cache and temp directories")]
    Cleantemp,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run) => {
            run_cleanup();
        }
        Some(Commands::Runtarget { dir }) => {
            run_cleanup_on_dir(&dir);
        }
        Some(Commands::Cleantemp) => {
            run_cleanup_on_temp();
        }
        None => {
            Cli::command().print_help().unwrap();
        }
    }
}
