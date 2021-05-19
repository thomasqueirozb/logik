mod ast;
mod bracket;
mod operator;
mod parenthesis;
mod parser;
mod tests;
mod token;

use parser::eval;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

/// Logik
#[derive(Debug, StructOpt)]
#[structopt(author, about, rename_all = "kebab-case")]
/// Simple command line calculator
struct Opt {
    /// Program passed in as a string instead of using a file
    #[structopt(short, long)]
    command: Option<String>,

    /// Input file path
    #[structopt(conflicts_with = "command", required_unless = "command")]
    input_file: Option<PathBuf>,
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

fn run() -> Result<()> {
    let opt: Opt = Opt::from_args();

    let input = {
        if let Some(ifp) = opt.input_file {
            fs::read_to_string(ifp)?
        } else {
            opt.command.unwrap()
        }
    };

    eval(input)
}
