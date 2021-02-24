mod operator;

use operator::Op;

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

/// Calculator
#[derive(Debug, StructOpt)]
struct Opt {
    /// Input string
    input: String,
}

fn main() {
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

    let mut tokens: Vec<Token> = tokens
        .iter()
        .map(|tk| match tk {
            InitialToken::Op(op) => Token::Op(*op),
            InitialToken::MaybeNumber(mb_num) => {
                if let Ok(num) = mb_num.parse::<f64>() {
                    Token::Number(num)
                } else {
                    panic!("{} cannot be converted into a number", mb_num);
                }
            }
        })
        .collect();

    let mut counter = 0f64;

    if let Some(first) = tokens.get(0) {
        if let Token::Op(op) = first {
            if *op == Op::Add || *op == Op::Sub {
                tokens.insert(0, Token::Number(0f64));
            }
        }

        counter = match &tokens[0] {
            Token::Op(op) => {
                panic!("Cannot have {} in the start", op);
            }
            Token::Number(num) => *num,
        };

        for i in (1..tokens.len()).filter(|x| x % 2 == 1) {
            let tk = &tokens[i];
            let tk_after = &tokens[i + 1];
            counter = match tk {
                Token::Op(op) => match tk_after {
                    Token::Op(_) => panic!("Expected number found operator"),
                    Token::Number(num) => op.execute(counter, *num),
                },
                Token::Number(_) => panic!("Expected operator found number"),
            };
        }
    }

    println!("{:?}", tokens);
    println!("{}", counter);
}
