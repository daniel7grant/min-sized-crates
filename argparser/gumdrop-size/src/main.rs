use gumdrop::Options;

#[derive(Debug, Options)]
struct GitOptions {
    /// print help message
    #[options()]
    help: bool,
    /// print more information (can be specified multiple times)
    #[options()]
    verbose: Option<u8>,

    #[options(command)]
    command: Option<Command>,
}

#[derive(Debug, Options)]
enum Command {
    /// add files to the staging area
    #[options()]
    Add(AddOptions),
    /// commit all changes
    #[options()]
    Commit(CommitOptions),
}

#[derive(Debug, Options)]
struct AddOptions {
    /// print help message
    #[options()]
    help: bool,
    /// the files to add
    #[options(free)]
    files: Vec<String>,
    /// add all files
    #[options(short = "A")]
    all: bool,
}

#[derive(Debug, Options)]
struct CommitOptions {
    /// print help message
    #[options()]
    help: bool,
    /// commit message (required)
    #[options()]
    message: String,
    /// amend last commit
    #[options(no_short)]
    amend: bool,
    /// skip running pre-commit hooks
    #[options(long = "no-verify")]
    no_verify: bool,
}

fn main() {
    let opts = GitOptions::parse_args_default_or_exit();

    println!("{:#?}", opts);
}
