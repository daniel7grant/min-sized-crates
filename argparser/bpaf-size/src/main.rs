use bpaf::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options("git"))]
pub enum Options {
	/// add files to the staging area
    #[bpaf(command("add"))]
    Add {
        /// add all files
        #[bpaf(short('A'), long("all"))]
        all: bool,
        /// the files to add
        #[bpaf(positional("FILES"))]
        files: Vec<String>,
    },
    /// commit all changes
    #[bpaf(command("commit"))]
    Commit {
        /// commit message (required)
        #[bpaf(short, long, argument("MESSAGE"))]
        message: String,
        /// amend last commit
        #[bpaf(long("amend"))]
        amend: bool,
        /// skip running pre-commit hooks
        #[bpaf(short, long("no-verify"))]
        no_verify: bool,
    },
}

fn main() {
	let opts = options().run();

    println!("{:?}", opts);
}
