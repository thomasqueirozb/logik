mod operator;

use operator::Op;

use anyhow::{bail, Context, Result};
// use structopt::StructOpt;

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(i64),
    Op(Op),
}

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
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn tokenize(input: String) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];

    let mut buffer = String::new();

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum TokenizerState {
        None,
        Number,
        Op,
        Comment,
    };

    let mut state = TokenizerState::None;

    for (_pos, c) in input.chars().chain(std::iter::once('\0')).enumerate() {
        let prev_state = state;
        if (c.is_whitespace() || c == '\0') && state != TokenizerState::Comment {
            state = TokenizerState::None;
        } else {
            match state {
                TokenizerState::None => {
                    if c.is_numeric() {
                        state = TokenizerState::Number;
                    } else if Op::is_op(c) {
                        state = TokenizerState::Op;
                    } else {
                        bail!("Unparsable char '{}'", c);
                    }
                }
                TokenizerState::Number => {
                    if !c.is_numeric() {
                        if Op::is_op(c) {
                            state = TokenizerState::Op;
                        }
                    }
                }
                TokenizerState::Op => {
                    if !Op::is_op(c) {
                        state = TokenizerState::Number;
                    } else {
                        if let Some(lc) = buffer.chars().last() {
                            if lc == '/' && c == '*' {
                                buffer.pop();
                                state = TokenizerState::Comment;
                            }
                        }
                    }
                }
                TokenizerState::Comment => match c {
                    '*' => {
                        buffer.clear();
                        buffer.push(c);
                    }
                    '/' => {
                        if !buffer.is_empty() {
                            state = TokenizerState::None
                        }
                    }
                    _ => buffer.clear(),
                },
            }
            // println!("c {}, state {:?}", c, state);
        }

        if prev_state != state {
            match prev_state {
                TokenizerState::None | TokenizerState::Comment => {
                    buffer.clear();
                }
                TokenizerState::Op => {
                    if buffer.len() > 1 {
                        bail!("Multiple operators together");
                    } else if buffer.len() != 0 {
                        tokens.push(Token::Op(
                            Op::from_char(buffer.chars().next().unwrap()).context("Op conv")?,
                        ));

                        buffer.clear();
                    }
                }
                TokenizerState::Number => {
                    if buffer.len() != 0 {
                        match buffer.parse::<i64>() {
                            Ok(num) => tokens.push(Token::Number(num)),
                            Err(e) => bail!("Could not convert \"{}\" to i64 - ({})", buffer, e),
                        }

                        buffer.clear();
                    }
                }
            }
        }
        match state {
            TokenizerState::None | TokenizerState::Comment => {}
            _ => buffer.push(c),
        }
    }

    if state == TokenizerState::Comment {
        bail!("Unterminated comment");
    }
    Ok(tokens)
}

fn parse(mut tokens: Vec<Token>) -> Result<i64> {
    // println!("tokens {:?}", tokens);
    if let Some(_first) = tokens.get(0) {
        /*
        if let Token::Op(op) = first {
            if *op == Op::Add || *op == Op::Sub {
                tokens.insert(0, Token::Number(0i64));
            }
        }
        */

        let get_num = |tokens: &mut Vec<Token>, i: usize| -> Result<i64> {
            match tokens.get(i).context("Missing token")? {
                Token::Op(op) => bail!("Expected number found operator \"{}\"", op),
                Token::Number(num) => Ok(*num),
            }
        };

        let get_op = |tokens: &mut Vec<Token>, i: usize| -> Result<Op> {
            match tokens.get(i).context("Missing token")? {
                Token::Op(op) => Ok(*op),
                Token::Number(num) => bail!("Expected operator found number \"{}\"", num),
            }
        };

        let mut has_changed = true;
        while has_changed {
            has_changed = false;

            let mut i = 0;
            while i < tokens.len() {
                let num_0 = get_num(&mut tokens, i)?;

                if i + 1 >= tokens.len() {
                    break;
                }

                let op_1 = get_op(&mut tokens, i + 1)?;

                if op_1 == Op::Add || op_1 == Op::Sub {
                    i += 2;
                } else {
                    let num_2 = get_num(&mut tokens, i + 2)?;

                    tokens.remove(i);
                    tokens.remove(i);
                    tokens.remove(i);
                    tokens.insert(i, Token::Number(op_1.execute(num_0, num_2)));
                    has_changed = true;
                }
            }
        }

        let mut counter = get_num(&mut tokens, 0)?;

        let mut i = 1;
        while i < tokens.len() {
            let op = get_op(&mut tokens, i)?;

            if i + 1 >= tokens.len() {
                bail!("Expected number after operator");
            }

            let num = get_num(&mut tokens, i + 1)?;
            counter = op.execute(counter, num);
            i += 2;
        }

        Ok(counter)
    } else {
        bail!("Empty string");
    }
}

pub fn eval<T>(input: T) -> Result<i64>
where
    T: Into<String>,
{
    let tokens = tokenize(input.into())?;

    let result = parse(tokens)?;
    Ok(result)
}

fn run() -> Result<()> {
    // let opt: Opt = Opt::from_args();
    // let input = opt.input;
    let mut args = std::env::args();
    args.next().context("Missing path???")?;
    let input = args.next().context("Missing input")?;
    if args.next().is_some() {
        bail!("Too many arguments expected only one");
    }
    println!("{}", eval(input)?);

    Ok(())
}

#[test]
fn comment() {
    assert_eq!(eval("/* a */ 1 /* b */").unwrap(), 1);
    assert_eq!(eval("/* /* 3 */ 1 /* b */").unwrap(), 1);
    assert_eq!(eval("/* /**/ 2+1 /* b */").unwrap(), 3);
    assert_eq!(eval("1-/*/**/ 2+1 /* b */").unwrap(), 0);
    assert_eq!(eval("2*/*/**/ 3+5 /* b */").unwrap(), 11);
    assert_eq!(eval("2 /* * /+ 2 */ + 2").unwrap(), 4);
}

#[test]
fn sum_sub() {
    assert_eq!(eval("1  -   04 ").unwrap(), -3);
    assert_eq!(eval("11-4").unwrap(), 7);
    assert_eq!(eval("11-4 + 22 -23").unwrap(), 6);
    assert_eq!(eval("1  +   4 ").unwrap(), 5);
    assert_eq!(eval("1 + 40 ").unwrap(), 41);
}

#[test]
fn div_mul() {
    assert_eq!(eval("81/ 9 + 3").unwrap(), 12);
    assert_eq!(eval("4/2").unwrap(), 2);
    assert_eq!(eval("4/2 + 3").unwrap(), 5);
    assert_eq!(eval("3 + 4/2").unwrap(), 5);
    assert_eq!(eval("3*5 + 10").unwrap(), 25);
    assert_eq!(eval("10 + 3*5").unwrap(), 25);
}

#[test]
fn errors() {
    assert_eq!(
        eval("1 a  -   04 ").unwrap_err().to_string(),
        "Unparsable char 'a'"
    );
    assert_eq!(
        eval("1a1-4").unwrap_err().to_string(),
        "Could not convert \"1a1\" to i64 - (invalid digit found in string)"
    );
    assert_eq!(
        eval("11-4/* + 22 -23").unwrap_err().to_string(),
        "Unterminated comment"
    );
    assert_eq!(
        eval("3- 3 /* a").unwrap_err().to_string(),
        "Unterminated comment"
    );

    assert_eq!(
        eval("3+ /* a */").unwrap_err().to_string(),
        "Expected number after operator"
    );

    assert_eq!(
        eval("-3 /* a */").unwrap_err().to_string(),
        "Expected number found operator \"-\""
    );

    assert_eq!(
        eval("3+ /* a */-").unwrap_err().to_string(),
        "Expected number found operator \"-\""
    );

    assert_eq!(eval("/* */").unwrap_err().to_string(), "Empty string");
}
