#![allow(dead_code)]
use std::{ffi::OsString, process};

#[derive(Debug)]
struct AddOptions {
    files: Vec<OsString>,
    all: bool,
}

#[derive(Debug)]
struct CommitOptions {
    message: String,
    amend: bool,
    no_verify: bool,
}

fn main() -> Result<(), pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    match pargs.subcommand()?.as_deref() {
        Some("add") => {
            let add_options = AddOptions {
                all: pargs.contains(["-A", "--all"]),
                files: pargs.finish(),
            };

            println!("{:?}", add_options);
        }
        Some("commit") => {
            let commit_options = CommitOptions {
                message: pargs.value_from_str(["-m", "--message"])?,
                amend: pargs.contains("--amend"),
                no_verify: pargs.contains(["-n", "--no-verify"]),
            };

            if !pargs.finish().is_empty() {
                eprintln!("You can't have additional options.");
                process::exit(1);
            }

            println!("{:?}", commit_options);
        }
        _ => {
            eprintln!("You have to use a subcommand: add or commit.");
            process::exit(1);
        }
    }

    Ok(())
}
