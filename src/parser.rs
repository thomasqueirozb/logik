use crate::ast::*;
use crate::operator::Op;
use crate::parenthesis::*;
use crate::token::*;

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    vars: Rc<Cell<HashMap<String, Number>>>,
}

// Started using macros because using self.method started giving me borrow errors. This probably
// does the same thing tbh

macro_rules! select_next {
    ($self:ident) => {
        // WARNING
        // $self.idx = $self.idx.wrapping_add(1);
    };
}

macro_rules! cur_token {
    ($self:expr) => {{
        let r: Result<Token> = match $self.tokens.get($self.idx) {
            Some(tk) => Ok(tk.clone()),
            None => bail!("Could not get next token"),
        };
        r
    }};
}

macro_rules! next_token {
    ($self:ident) => {{
        select_next!($self);
        cur_token!($self)
    }};
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            idx: 0usize.wrapping_sub(1),
            vars: Rc::new(Cell::new(HashMap::new())),
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<()> {
        let mut parser = Parser::new(tokens);

        parser.parse_block()
    }

    fn parse_factor(&mut self) -> Result<Box<dyn Node + '_>> {
        let tk = next_token!(self)?; // WARNING
        match &tk.kind {
            TokenKind::Number(n) => {
                select_next!(self);
                Ok(Box::new(NumberNode::new(*n)))
            }

            TokenKind::Variable(name) => {
                select_next!(self);

                Ok(Box::new(VariableNode::new(name.clone(), &self.vars)))

                // Ok(Box::new(NumberNode::new(
                //     *self
                //         .vars
                //         .get(&name)
                //         .expect("variable used before assignment"),
                // )))
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
                    // WARNING {}
                    let r = { self.parse_expression()? };

                    {
                        let closed = {
                            if let TokenKind::Parenthesis(p) = cur_token!(self)?.kind {
                                p == Parenthesis::Close
                            } else {
                                false
                            }
                        };

                        if !closed {
                            bail!("Unclosed parenthesis");
                        }
                    }
                    {
                        select_next!(self);
                    }
                    Ok(r)
                }
                Parenthesis::Close => unreachable!(), // FIXME probably reachable
            },

            TokenKind::EOF
            | TokenKind::SemiColon
            | TokenKind::Equals
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

    fn parse_term(&mut self) -> Result<Box<dyn Node + '_>> {
        let mut c = self.parse_factor()?;

        loop {
            let tk = cur_token!(self)?;
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

    fn parse_expression(&mut self) -> Result<Box<dyn Node + '_>> {
        let mut c = self.parse_term()?;

        loop {
            let tk = cur_token!(self)?;
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
        let tk = cur_token!(self)?;
        match &tk.kind {
            TokenKind::Variable(name) => {
                let ntk = next_token!(self)?;
                match ntk.kind {
                    TokenKind::Parenthesis(p) => {
                        match p {
                            Parenthesis::Open => {
                                // Function call
                                match name.as_str() {
                                    "println" => {
                                        println!("{}", self.parse_expression()?.eval());
                                        select_next!(self);
                                    }
                                    _ => bail!("Unrecognized funcion name {}", name),
                                }
                            }
                            Parenthesis::Close => {
                                bail!("Close par") // TODO fix error message
                            }
                        }
                    }
                    TokenKind::Equals => {
                        let val = self.parse_expression()?.eval(); // FIXME ;
                        self.vars.get_mut().insert(name.clone(), val);
                    }
                    _ => bail!("Expected = or (...) after {}", name),
                }
            }
            TokenKind::SemiColon => {}
            _ => bail!("Line not started with variable/function call"),
        }

        if cur_token!(self)?.kind != TokenKind::SemiColon {
            bail!("Command not terminated by ';'")
        }

        Ok(())
    }

    fn parse_block(&mut self) -> Result<()> {
        select_next!(self);
        while cur_token!(self)?.kind != TokenKind::EOF {
            self.parse_command()?;
            select_next!(self);
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

    if cur_token!(&parser)?.kind != TokenKind::EOF {
        bail!("Finished parsing but not EOF")
    }

    Ok(tree.eval())
}
