mod operator;

use operator::Op;

use anyhow::bail;
use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Token {
    Number(i64),
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

    let mut tokens: Vec<InitialToken> = vec![];

    let mut buffer = String::new();

    let tokens_push_buf = |buffer: &mut String, tokens: &mut Vec<InitialToken>| {
        if !buffer.is_empty() {
            tokens.push(InitialToken::MaybeNumber(buffer.clone()));
            buffer.clear();
        }
    };

    for c in opt.input.chars() {
        if !c.is_ascii() {
            continue;
        }

        if let Some(op) = Op::from_char(c) {
            tokens_push_buf(&mut buffer, &mut tokens);
            tokens.push(InitialToken::Op(op));
        } else {
            if c.is_ascii_whitespace() {
                tokens_push_buf(&mut buffer, &mut tokens);
            } else {
                buffer.push(c);
            }
        }
    }

    tokens_push_buf(&mut buffer, &mut tokens);

    let tokens: Vec<Token> = {
        let mut nt = vec![];
        for tk in tokens {
            let v = match tk {
                InitialToken::Op(op) => Token::Op(op),
                InitialToken::MaybeNumber(mb_num) => match mb_num.parse::<i64>() {
                    Ok(num) => Token::Number(num),
                    Err(e) => bail!("Could not convert \"{}\" to i64 - ({})", mb_num, e),
                },
            };
            nt.push(v);
        }
        nt
    };

    if let Some(_first) = tokens.get(0) {
        /*
        if let Token::Op(op) = first {
            if *op == Op::Add || *op == Op::Sub {
                tokens.insert(0, Token::Number(0i64));
            }
        }
        */

        let mut counter = match &tokens[0] {
            Token::Op(op) => {
                bail!("Cannot start with \"{}\"", op);
            }
            Token::Number(num) => *num,
        };

        for i in (1..tokens.len()).filter(|x| x % 2 == 1) {
            let tk = &tokens[i];
            counter = match tk {
                Token::Op(op) => {
                    if (i + 1) >= tokens.len() {
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
        println!("{}", counter);
    } else {
        bail!("Empty string");
    }

    Ok(())
}
