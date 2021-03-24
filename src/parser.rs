use crate::operator::Op;
use crate::parenthesis::*;
use crate::token::*;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<i64> {
        let mut p = Self {
            tokens,
            idx: 0usize.wrapping_sub(1), // FIXME
        };
        p.parse_expression()
    }

    fn get_next(&mut self) {
        self.idx = self.idx.wrapping_add(1);
    }

    fn cur_token(&mut self) -> Result<Token> {
        match self.tokens.get(self.idx) {
            Some(tk) => Ok(*tk),
            None => bail!("Could not get next token"),
        }
    }

    fn parse_factor(&mut self) -> Result<i64> {
        self.get_next();
        let tk = self.cur_token()?;
        match tk {
            Token::Number(n) => {
                self.get_next();
                Ok(n)
            }
            Token::Op(op) => {
                let s = match op {
                    Op::Add => 1,
                    Op::Sub => -1,
                    _ => unreachable!(),
                };
                let r = s * self.parse_factor()?;
                Ok(r)
            }

            Token::Parenthesis(p) => match p {
                Parenthesis::Open => {
                    let r = self.parse_expression()?;
                    self.get_next();
                    Ok(r)
                }
                Parenthesis::Close => unreachable!(),
            },
            _ => bail!("Expected number, operator or (, found EOF"),
        }
    }

    fn parse_term(&mut self) -> Result<i64> {
        let mut c = self.parse_factor()?;

        loop {
            let tk = self.cur_token()?;
            match tk {
                Token::Op(op) => {
                    match op {
                        Op::Div | Op::Mul => {
                            c = op.execute(c, self.parse_factor()?);
                        }
                        _ => break,
                    };
                }
                _ => break,
            }
        }
        Ok(c)
    }

    fn parse_expression(&mut self) -> Result<i64> {
        let mut c = self.parse_term()?;

        loop {
            let tk = self.cur_token()?;
            match tk {
                Token::Op(op) => {
                    let s = match op {
                        Op::Add => 1,
                        Op::Sub => -1,
                        _ => break,
                    };
                    c += s * self.parse_term()?;
                }

                _ => break,
            }
        }

        Ok(c)
    }
}

pub fn eval<T>(input: T) -> Result<i64>
where
    T: Into<String>,
{
    let tokens = tokenize(input.into())?;

    let result = Parser::parse(tokens)?;
    Ok(result)
}
