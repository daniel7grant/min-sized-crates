use clap::{Parser, Subcommand};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
struct Cli {
    /// print more information (can be specified multiple times)
    #[arg()]
    verbose: Option<u8>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// add files to the staging area
    Add {
        /// the files to add
        #[arg()]
        files: Vec<String>,
        /// add all files
        #[arg(short = 'A', long = "all")]
        all: bool,
    },
    /// commit all changes
    Commit {
        /// commit message (required)
        #[arg(short, long)]
        message: String,
        /// amend last commit
        #[arg(long = "amend")]
        amend: bool,
        /// skip running pre-commit hooks
        #[arg(short, long = "no-verify")]
        no_verify: bool,
    },
}

fn main() {
    let args = Cli::parse();

    println!("{:#?}", args);
}
