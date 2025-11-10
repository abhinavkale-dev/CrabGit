use clap::{Parser, Subcommand};
use CrabGit::{Result, commands};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "crab_git")]
#[command(about = "A simple git implementation in Rust", version)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(help = "Directory to initialize")]
        path: Option<PathBuf>
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            commands::init(path)?;
        }
    }
    Ok(())
}