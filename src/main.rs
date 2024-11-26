use anyhow::{anyhow, Result};
use clap::Parser;
use clap::{builder::PossibleValue, ValueEnum};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of `find`
struct Args {
    /// Search paths
    #[arg(default_value = ".", value_name = "PATH")]
    paths: Vec<String>,

    /// Name
    #[arg(short = 'n', long = "name", value_name = "<NAME>")]
    names: Vec<Regex>,

    /// Entry type
    #[arg(short = 't', long = "type", value_name = "<TYPE>")]
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
    let args = Args::parse();
    println!("{:?}", args);
}
