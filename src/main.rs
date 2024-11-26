use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use clap::{ArgAction, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of `find`
struct Args {
    /// Search paths
    #[arg(default_value = ".", value_name = "PATH")]
    paths: Vec<String>,

    /// Name
    #[arg(
       short = 'n',
       long = "name",
       value_name = "NAME",
       value_parser(Regex::new),
       action(ArgAction::Append),
       num_args(0..)
    )]
    names: Vec<Regex>,

    /// Entry type
    #[arg(
       short = 't',
       long = "type",
       value_name = "TYPE",
       value_parser(clap::value_parser!(EntryType)),
       action(ArgAction::Append),
       num_args(0..)
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    let args = Args::parse();
    println!("{:?}", args);
}

fn run(args: Args) -> Result<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{e}"),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}
