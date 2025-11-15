use clap::{Parser, Subcommand};
use CrabGit::{Result, commands, utils};
use std::path::PathBuf;

pub const BANNER: &str = r#"
 $$$$$$\  $$$$$$$\   $$$$$$\  $$$$$$$\   $$$$$$\  $$$$$$\ $$$$$$$$\ 
$$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ $$  __$$\ \_$$  _|\__$$  __|
$$ /  \__|$$ |  $$ |$$ /  $$ |$$ |  $$ |$$ /  \__|  $$ |     $$ |   
$$ |      $$$$$$$  |$$$$$$$$ |$$$$$$$\ |$$ |$$$$\   $$ |     $$ |   
$$ |      $$  __$$< $$  __$$ |$$  __$$\ $$ |\_$$ |  $$ |     $$ |   
$$ |  $$\ $$ |  $$ |$$ |  $$ |$$ |  $$ |$$ |  $$ |  $$ |     $$ |   
\$$$$$$  |$$ |  $$ |$$ |  $$ |$$$$$$$  |\$$$$$$  |$$$$$$\    $$ |   
 \______/ \__|  \__|\__|  \__|\_______/  \______/ \______|   \__|   
                                                                    
                                                                    
                                                                    
"#;

pub const RUST_COLOR: &str = "\x1b[38;2;217;155;121m";
pub const RESET_COLOR: &str = "\x1b[0m";               

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
    },

    Branch {
        #[arg(help = "Branch name")]
        name: Option<String>,
        
        #[arg(short, long, help = "Delete branch")]
        delete: bool
    },

    Diff {
        #[arg(help = "Files to diff (optional)")]
        paths: Vec<String>
    },

    Checkout {
        #[arg(help = "Branch or commit to checkout")]
        branch_or_commit: String
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        println!("{}{}{}", RUST_COLOR, BANNER, RESET_COLOR);
        println!("{}🦀 CrabGit - Git Implementation from Scratch in Rust{}", RUST_COLOR, RESET_COLOR);
        println!("{}Usage: {} <COMMAND>{}", RUST_COLOR, args[0], RESET_COLOR);
        println!("{}Run with --help for more information{}", RUST_COLOR, RESET_COLOR);
        return Ok(());
    }
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

        Commands::Branch { name, delete } => {
            let repo = utils::get_repository(None)?;
            commands::branch(&repo, name, delete)?;
        }

        Commands::Diff { paths } => {
            let repo = utils::get_repository(None)?;
            commands::diff(&repo, paths)?;
        }

        Commands::Checkout { branch_or_commit } => {
            let repo = utils::get_repository(None)?;
            commands::checkout(&repo, branch_or_commit)?;
        }
    }
    Ok(())
}