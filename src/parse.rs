use crate::operator::Op;
use crate::token::*;

use anyhow::{bail, Context, Result};

pub fn parse(mut tokens: Vec<Token>) -> Result<i64> {
    // println!("tokens {:?}", tokens);
    if tokens.len() > 0 {
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
        bail!("No tokens found");
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
