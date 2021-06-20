use crate::ast::*;
use crate::operator::Op;
use crate::token::*;
use crate::variable::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    idx: usize,
    vars: Rc<RefCell<HashMap<String, Variable>>>,
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

        parser.parse_block()?.eval();
        Ok(())
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

    fn select_prev(&mut self) {
        self.idx = self.idx.wrapping_sub(1);
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
                Ok(Box::new(NumberLiteralNode::new(*n)))
            }

            TokenKind::String(s) => {
                self.select_next();
                Ok(Box::new(StringLiteralNode::new(s.clone())))
            }

            TokenKind::True => {
                self.select_next();
                Ok(Box::new(BoolLiteralNode::new(true)))
            }

            TokenKind::False => {
                self.select_next();
                Ok(Box::new(BoolLiteralNode::new(false)))
            }

            TokenKind::Identifier(name) => {
                let ntk = self.next_token()?;
                if ntk.kind == TokenKind::ParenthesisOpen {
                    Ok(Box::new(FuncNode::new(name.clone(), self.get_func_args()?)))
                } else {
                    Ok(Box::new(VariableNode::new(name.clone(), &self.vars)))
                }
            }

            TokenKind::Op(op) => {
                let kind = match op {
                    Op::Add => UnaryNodeKind::Pos,
                    Op::Sub => UnaryNodeKind::Neg,
                    Op::Not => UnaryNodeKind::Not,
                    _ => bail!("Expected '+' or '-' or '!' found '{}'", op),
                };

                Ok(Box::new(UnaryNode::new(kind, self.parse_factor()?)))
            }

            TokenKind::ParenthesisOpen => {
                let r = self.parse_cond()?;

                if self.cur_token()?.kind != TokenKind::ParenthesisClose {
                    bail!("Unclosed parenthesis");
                }

                self.select_next();
                Ok(r)
            }

            TokenKind::CondOp(op) => {
                bail!("Expected number, variable, operator or '(', found '{}'", op)
            }

            TokenKind::EOF
            | TokenKind::ParenthesisClose
            | TokenKind::BracketOpen
            | TokenKind::BracketClose
            | TokenKind::TypeInt
            | TokenKind::TypeString
            | TokenKind::TypeBool
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

    fn get_func_args(&mut self) -> Result<Vec<Box<dyn Node>>> {
        let mut v = vec![];

        if self.next_token()?.kind != TokenKind::ParenthesisClose {
            self.select_prev();

            let arg = self.parse_cond()?;
            v.push(arg);

            let tk = self.cur_token()?;
            if tk.kind != TokenKind::ParenthesisClose {
                bail!("Function argument expected ')', found {}", tk.kind);
            }
        }
        self.select_next();

        Ok(v)
    }

    fn parse_command(&mut self) -> Result<Box<dyn Node>> {
        let tk = self.cur_token()?;
        let ret = match &tk.kind {
            TokenKind::TypeInt | TokenKind::TypeBool | TokenKind::TypeString => {
                let ntk = self.next_token()?;
                if let TokenKind::Identifier(name) = &ntk.kind {
                    let kind = match tk.kind {
                        TokenKind::TypeInt => VariableKind::Number,
                        TokenKind::TypeBool => VariableKind::Bool,
                        TokenKind::TypeString => VariableKind::String,
                        _ => unreachable!(),
                    };

                    let ntk = self.next_token()?;
                    if ntk.kind == TokenKind::Assign {
                        let node = self.parse_cond()?;
                        // self.select_next(); // WARNING check if necessary

                        let v: Box<dyn Node> = match tk.kind {
                            TokenKind::TypeInt => Box::new(NumberNode::new(node)),
                            TokenKind::TypeBool => Box::new(StringNode::new(node)),
                            TokenKind::TypeString => Box::new(BoolNode::new(node)),
                            _ => unreachable!(),
                        };
                        Box::new(DeclareNode::new(name.clone(), Some(v), kind, &self.vars))
                    } else {
                        self.select_prev();

                        Box::new(DeclareNode::new(name.clone(), None, kind, &self.vars))
                    }
                    // Box::new(DeclareNode::new(name.clone(), None))
                    // FIXME None
                } else {
                    bail!("Expected identifier after {}, got {}", tk.kind, ntk.kind)
                }
            }
            TokenKind::Identifier(name) => {
                let ntk = self.next_token()?;
                let r: Box<dyn Node> = match ntk.kind {
                    TokenKind::ParenthesisOpen => {
                        // Function call
                        Box::new(FuncNode::new(name.clone(), self.get_func_args()?))
                    }
                    TokenKind::Assign => Box::new(AssignNode::new(
                        name.clone(),
                        self.parse_cond()?,
                        &self.vars,
                    )),
                    _ => bail!("Expected = or (...) after {}", name),
                };
                r
            }
            TokenKind::If => {
                let ntk = self.next_token()?;
                if ntk.kind != TokenKind::ParenthesisOpen {
                    bail!("Expected '(' after if");
                }

                let cond = self.parse_cond()?;

                let ntk = self.cur_token()?;
                if ntk.kind != TokenKind::ParenthesisClose {
                    bail!("Expected ')' closing if, got {}", ntk);
                }
                self.select_next();

                let if_child = self.parse_command()?;

                let ntk = self.next_token()?;

                if ntk.kind == TokenKind::Else {
                    self.select_next();
                    Box::new(IfNode::new(cond, if_child, Some(self.parse_command()?)))
                } else {
                    self.select_prev();
                    Box::new(IfNode::new(cond, if_child, None))
                }
            }
            TokenKind::BracketOpen => self.parse_block()?,
            TokenKind::While => {
                let ntk = self.next_token()?;
                if ntk.kind != TokenKind::ParenthesisOpen {
                    bail!("Expected '(' after while");
                }

                let cond = self.parse_cond()?;

                let ntk = self.cur_token()?;
                if ntk.kind != TokenKind::ParenthesisClose {
                    bail!("Expected ')' closing while");
                }
                self.select_next();

                let child = self.parse_command()?;

                Box::new(WhileNode::new(cond, child))
            }

            TokenKind::SemiColon => {
                self.select_next();
                self.parse_command()? // WARNING FIXME
            }
            _ => bail!(
                "Expected line to be started with variable/function call, got {}",
                tk
            ),
        };

        // if self.cur_token()?.kind != TokenKind::SemiColon {
        //     bail!("Command not terminated by ';'")
        // }

        Ok(ret)
    }

    fn parse_cond(&mut self) -> Result<Box<dyn Node>> {
        let expr = self.parse_expression()?;
        let tk = self.cur_token()?;
        if let TokenKind::CondOp(cop) = tk.kind {
            let mut cn = Box::new(CondNode::new(cop, expr, self.parse_expression()?));
            loop {
                let ltk = self.cur_token()?;
                if let TokenKind::CondOp(cop) = ltk.kind {
                    cn = Box::new(CondNode::new(cop, cn, self.parse_expression()?));
                } else {
                    break;
                }
            }
            Ok(cn)
        } else {
            Ok(expr)
        }
    }

    fn parse_block(&mut self) -> Result<Box<dyn Node>> {
        let mut commands = vec![];

        loop {
            let tk = self.next_token()?;
            match tk.kind {
                TokenKind::EOF | TokenKind::BracketClose => break,
                _ => commands.push(self.parse_command()?),
                // // Always start with {
                // TokenKind::BracketOpen => {
                //     loop {
                //         match self.next_token()?.kind {
                //             TokenKind::BracketOpen => bail!("Open after open"), // TODO fix message
                //             TokenKind::BracketClose => break,                   // TODO fix message
                //             _ => commands.push(self.parse_command()?),
                //         }
                //     }
                // }
                // _ => bail!(
                //     "Expected '{{' or EOF, got {} at {}:{}",
                //     tk.kind,
                //     tk.line,
                //     tk.col
                // ),
            }
        }
        Ok(Box::new(BlockNode::new(commands)))
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

    match tree.eval() {
        VariableData::Number(n) => Ok(n),
        VariableData::Bool(b) => Ok(b as Number),
        _ => bail!("Wrong type"),
    }
}
