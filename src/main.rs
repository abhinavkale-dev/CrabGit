use clap::{Parser, Subcommand};
use CrabGit::{Result, commands, utils};
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

    Add {
        #[arg(help = "Files to add")]
        paths: Vec<String>
    },

    Commit {
        #[arg(help = "Commit message")]
        message: String,
        
        #[arg(short, long, help = "Author name")]
        author: Option<String>
    },

    Status,

    Log {
        #[arg(short, long, help = "Maximum number of commits to display")]
        max_count: Option<usize>
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            commands::init(path)?;
        }

        Commands::Add { paths } => {
            let repo = utils::get_repository(None)?;
            commands::add(&repo, paths)?;
        }

        Commands::Commit { message, author } => {
            let repo = utils::get_repository(None)?;
            commands::commit(&repo, message, author)?;
        }

        Commands::Status => {
            let repo = utils::get_repository(None)?;
            commands::status(&repo)?;
        }

        Commands::Log { max_count } => {
            let repo = utils::get_repository(None)?;
            commands::log(&repo, max_count)?;
        }
    }
    Ok(())
}