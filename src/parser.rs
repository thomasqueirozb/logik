use crate::ast::*;
use crate::operator::Op;
use crate::parenthesis::*;
use crate::token::*;

use std::collections::HashMap;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    vars: HashMap<String, Number>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            idx: 0usize.wrapping_sub(1),
            vars: HashMap::new(),
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<()> {
        // dbg!(tokens.clone());
        let mut parser = Parser::new(tokens);

        parser.parse_block()
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
            Some(tk) => Ok(tk.clone()), // FIXME remove clone
            None => bail!("Could not get next token"),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node>> {
        match self.next_token()? {
            Token::Number(n) => {
                self.select_next();
                Ok(Box::new(NumberNode::new(n)))
            }

            Token::Variable(name) => {
                self.select_next();
                // FIXME use VariableNode instead
                Ok(Box::new(NumberNode::new(
                    *self
                        .vars
                        .get(&name)
                        .expect("variable used before assignment"),
                )))
            }

            // TODO: fix error messages here
            Token::SemiColon => bail!("Unexpected ;"),
            Token::Equals => bail!("Unexpected ="),

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
                Parenthesis::Close => unreachable!(), // FIXME probably reachable
            },
            Token::EOF => bail!("Expected number, operator or '(', found EOF"),
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

    fn parse_command(&mut self) -> Result<()> {
        if let Token::Variable(name) = self.cur_token()? {
            match self.next_token()? {
                Token::Parenthesis(p) => {
                    match p {
                        Parenthesis::Open => {
                            // Function call
                            match name.as_str() {
                                "println" => {
                                    println!("{}", self.parse_expression()?.eval());
                                    self.select_next();
                                }
                                _ => bail!("Unrecognized funcion name {}", name),
                            }
                        }
                        Parenthesis::Close => {
                            bail!("Close par") // TODO fix error message

                            // unreachable!() // ?
                        }
                    }
                }
                Token::Equals => {
                    let val = self.parse_expression()?.eval(); // FIXME ;
                    self.vars.insert(name, val);
                }
                _ => bail!("Expected = or (...) after {}", name),
            }
        } else {
            bail!("Line not started with variable/function call")
        }
        Ok(())
    }

    fn parse_block(&mut self) -> Result<()> {
        self.select_next();
        while self.cur_token()? != Token::EOF {
            self.parse_command()?;
            if self.cur_token()? != Token::SemiColon {
                bail!("Command not terminated by ';'")
            }
            self.select_next();
        }
        Ok(())
    }
}

pub fn eval<T>(input: T) -> Result<()>
where
    T: Into<String>,
{
    let tokens = tokenize(input.into())?;

    Parser::parse(tokens)
}

#[allow(dead_code)]
pub(crate) fn eval_expression<T>(input: T) -> Result<Number>
where
    T: Into<String>,
{
    let tokens = tokenize(input.into())?;
    let mut parser = Parser::new(tokens);

    Ok(parser.parse_expression()?.eval())
}
