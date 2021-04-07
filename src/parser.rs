use crate::ast::*;
use crate::operator::Op;
use crate::parenthesis::*;
use crate::token::*;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Number> {
        let mut parser = Self {
            tokens,
            idx: 0usize.wrapping_sub(1),
        };
        let tree_root = parser.parse_expression()?;
        if parser.cur_token()? == Token::EOF {
            Ok(tree_root.eval())
        } else {
            bail!("Finished parsing but did not find EOF")
        }
    }

    fn select_next(&mut self) {
        self.idx = self.idx.wrapping_add(1);
    }

    fn next_token(&mut self) -> Result<Token> {
        self.select_next();
        self.cur_token()
    }

    fn cur_token(&mut self) -> Result<Token> {
        match self.tokens.get(self.idx) {
            Some(tk) => Ok(*tk),
            None => bail!("Could not get next token"),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node>> {
        match self.next_token()? {
            Token::Number(n) => {
                self.select_next();
                Ok(Box::new(NumberNode::new(n)))
            }

            Token::Op(op) => {
                let value = match op {
                    Op::Add => 1,
                    Op::Sub => -1,
                    _ => bail!("Expected '+' or '-' found '{}'", op),
                };
                Ok(Box::new(UnaryNode::new(value, self.parse_factor()?)))
            }

            Token::Parenthesis(p) => match p {
                Parenthesis::Open => {
                    let r = self.parse_expression()?;

                    let closed = if let Token::Parenthesis(p) = self.cur_token()? {
                        p == Parenthesis::Close
                    } else {
                        false
                    };

                    if !closed {
                        bail!("Unclosed parenthesis");
                    }

                    self.select_next();
                    Ok(r)
                }
                Parenthesis::Close => unreachable!(),
            },
            _ => bail!("Expected number, operator or '(', found EOF"),
        }
    }

    fn parse_term(&mut self) -> Result<Box<dyn Node>> {
        let mut c = self.parse_factor()?;

        loop {
            match self.cur_token()? {
                Token::Op(op) => {
                    match op {
                        Op::Div | Op::Mul => {
                            c = Box::new(BinaryNode::new(op, c, self.parse_factor()?));
                        }

                        _ => break,
                    };
                }
                _ => break,
            }
        }
        Ok(c)
    }

    fn parse_expression(&mut self) -> Result<Box<dyn Node>> {
        let mut c = self.parse_term()?;

        loop {
            match self.cur_token()? {
                Token::Op(op) => {
                    match op {
                        Op::Add | Op::Sub => {
                            c = Box::new(BinaryNode::new(op, c, self.parse_term()?));
                        }
                        _ => break,
                    };
                }

                _ => break,
            }
        }

        Ok(c)
    }
}

pub fn eval<T>(input: T) -> Result<Number>
where
    T: Into<String>,
{
    let tokens = tokenize(input.into())?;

    let result = Parser::parse(tokens)?;
    Ok(result)
}
