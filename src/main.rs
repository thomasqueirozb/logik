mod operator;
mod parse;
mod test;
mod token;

use parse::eval;

use anyhow::{bail, Context, Result};
// use structopt::StructOpt;

/*
/// Logik
#[derive(Debug, StructOpt)]
#[structopt(author, about)]
/// Simple command line calculator
struct Opt {
    /// Input string
    #[structopt(short, long)]
    input: String,
}
*/

fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<i64> {
    // let opt: Opt = Opt::from_args();
    // let input = opt.input;
    let mut args = std::env::args();
    args.next().context("Missing path???")?;
    let input = args.next().context("Missing input")?;
    if args.next().is_some() {
        bail!("Too many arguments expected only one");
    }
    let r = eval(input)?;

    Ok(r)
}
