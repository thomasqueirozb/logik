use crate::ast::*;
use crate::bracket::Bracket;
use crate::operator::Op;
use crate::parenthesis::*;
use crate::token::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    vars: Rc<RefCell<HashMap<String, Number>>>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            idx: 0usize.wrapping_sub(1),
            vars: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<()> {
        let mut parser = Parser::new(tokens);

        parser.parse_block()
    }

    fn cur_token(&mut self) -> Result<Token> {
        match self.tokens.get(self.idx) {
            Some(tk) => Ok(tk.clone()),
            None => bail!("Could not get next token"),
        }
    }

    fn select_next(&mut self) {
        self.idx = self.idx.wrapping_add(1);
    }

    fn next_token(&mut self) -> Result<Token> {
        self.select_next();
        self.cur_token()
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node>> {
        let tk = self.next_token()?;
        match &tk.kind {
            TokenKind::Number(n) => {
                self.select_next();
                Ok(Box::new(NumberNode::new(*n)))
            }

            TokenKind::Variable(name) => {
                self.select_next();

                Ok(Box::new(VariableNode::new(name.clone(), &self.vars)))
            }

            TokenKind::Op(op) => {
                let value = match op {
                    Op::Add => 1,
                    Op::Sub => -1,
                    _ => bail!("Expected '+' or '-' found '{}'", op),
                };
                Ok(Box::new(UnaryNode::new(value, self.parse_factor()?)))
            }

            TokenKind::Parenthesis(p) => match p {
                Parenthesis::Open => {
                    let r = self.parse_expression()?;

                    let closed = {
                        if let TokenKind::Parenthesis(p) = self.cur_token()?.kind {
                            p == Parenthesis::Close
                        } else {
                            false
                        }
                    };

                    if !closed {
                        bail!("Unclosed parenthesis");
                    }
                    self.select_next();
                    Ok(r)
                }
                Parenthesis::Close => unreachable!(), // FIXME probably reachable
            },

            TokenKind::EOF
            | TokenKind::SemiColon
            | TokenKind::Assign
            | TokenKind::While
            | TokenKind::If
            | TokenKind::Else => {
                bail!(
                    "Expected number, variable, operator or '(', found '{}'",
                    tk.kind
                )
            }

            TokenKind::Bracket(inner) => bail!(
                "Expected number, variable, operator or '(', found '{}'",
                inner
            ),
        }
    }

    fn parse_term(&mut self) -> Result<Box<dyn Node>> {
        let mut c = self.parse_factor()?;

        loop {
            let tk = self.cur_token()?;
            match tk.kind {
                TokenKind::Op(op) => {
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
            let tk = self.cur_token()?;
            match tk.kind {
                TokenKind::Op(op) => {
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
        let tk = self.cur_token()?;
        match &tk.kind {
            TokenKind::Variable(name) => {
                let ntk = self.next_token()?;
                match ntk.kind {
                    TokenKind::Parenthesis(p) => {
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
                            }
                        }
                    }
                    TokenKind::Assign => {
                        let val = self.parse_expression()?.eval(); // FIXME ;
                        self.vars.borrow_mut().insert(name.clone(), val); // WARNING borrow_mut
                    }
                    _ => bail!("Expected = or (...) after {}", name),
                }
            }
            TokenKind::SemiColon => {}
            _ => bail!("Line not started with variable/function call"),
        }

        if self.cur_token()?.kind != TokenKind::SemiColon {
            bail!("Command not terminated by ';'")
        }

        Ok(())
    }

    fn parse_block(&mut self) -> Result<()> {
        loop {
            match self.next_token()?.kind {
                TokenKind::EOF => break,
                // Always start with {
                TokenKind::Bracket(b) => match b {
                    Bracket::Open => {
                        loop {
                            match self.next_token()?.kind {
                                TokenKind::Bracket(b) => match b {
                                    Bracket::Open => bail!("Open after open"), // TODO fix message
                                    Bracket::Close => break, // After { there needs to be a }
                                },
                                _ => self.parse_command()?,
                            }
                        }
                    }
                    Bracket::Close => bail!("Close before open"), // TODO fix message
                },
                _ => bail!("block"), // TODO fix message
            }
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

    let tree = parser.parse_expression()?;

    if parser.cur_token()?.kind != TokenKind::EOF {
        bail!("Finished parsing but not EOF")
    }

    Ok(tree.eval())
}
