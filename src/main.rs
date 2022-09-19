mod assembler;
mod ast;
mod operator;
mod parser;
mod tests;
mod token;
mod variable;

use parser::eval;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;

/// Logik
#[derive(Parser, Debug)]
#[clap(author, about, rename_all = "kebab-case")]
/// Simple command line calculator
struct Opt {
    /// Program passed in as a string instead of using a file
    #[clap(short, long)]
    command: Option<String>,

    /// Input file path
    #[clap(conflicts_with = "command", required_unless = "command")]
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
    color_eyre::install()?;

    let opt: Opt = Opt::parse();

    let input = {
        if let Some(ifp) = opt.input_file {
            fs::read_to_string(ifp)?
        } else {
            opt.command.unwrap()
        }
    };

    eval(input)
}
