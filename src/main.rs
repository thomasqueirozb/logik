mod operator;

use operator::Op;

use anyhow::bail;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Op(Op),
}

#[derive(Debug)]
pub enum InitialToken {
    MaybeNumber(String),
    Op(Op),
}

/// Logik
#[derive(Debug, StructOpt)]
#[structopt(author, about)]
/// Simple command line calculator
struct Opt {
    /// Input string
    input: String,
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let opt: Opt = Opt::from_args();

    let mut buffer = String::new();

    let mut tokens: Vec<InitialToken> = vec![];

    for c in opt.input.chars().chain(['\0'].iter().cloned()) {
        if !c.is_ascii() {
            continue;
        }

        if let Some(op) = Op::from_char(c) {
            if !buffer.is_empty() {
                tokens.push(InitialToken::MaybeNumber(buffer.clone()));
                buffer.clear();
            }
            tokens.push(InitialToken::Op(op));
        } else {
            if c.is_ascii_whitespace() || c == '\0' {
                if !buffer.is_empty() {
                    tokens.push(InitialToken::MaybeNumber(buffer.clone()));
                    buffer.clear();
                }
            } else {
                buffer.push(c);
            }
        }
    }

    let tokens: Vec<Token> = {
        let mut nt = vec![];
        for tk in tokens {
            let v = match tk {
                InitialToken::Op(op) => Token::Op(op),
                InitialToken::MaybeNumber(mb_num) => match mb_num.parse::<f64>() {
                    Ok(num) => Token::Number(num),
                    Err(e) => bail!("Could not convert \"{}\" to f64 - ({})", mb_num, e),
                },
            };
            nt.push(v);
        }
        nt
    };

    let mut counter = 0f64;

    if let Some(_first) = tokens.get(0) {
        /*
        if let Token::Op(op) = first {
            if *op == Op::Add || *op == Op::Sub {
                tokens.insert(0, Token::Number(0f64));
            }
        }
        */

        counter = match &tokens[0] {
            Token::Op(op) => {
                bail!("Cannot start with \"{}\"", op);
            }
            Token::Number(num) => *num,
        };

        for i in (1..tokens.len()).filter(|x| x % 2 == 1) {
            let tk = &tokens[i];
            counter = match tk {
                Token::Op(op) => {
                    if (i + 1) > tokens.len() {
                        bail!("Expected something after \"{}\"", op);
                    }

                    let tk_after = &tokens[i + 1];
                    match tk_after {
                        Token::Op(after_op) => {
                            bail!("Expected number found operator \"{}\"", after_op)
                        }
                        Token::Number(num) => op.execute(counter, *num),
                    }
                }
                Token::Number(num) => bail!("Expected operator found number \"{}\"", num),
            };
        }
    }

    println!("{:?}", tokens);
    println!("{}", counter);
    Ok(())
}
