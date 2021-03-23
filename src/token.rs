use crate::operator::Op;

use anyhow::{bail, Context, Result};

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(i64),
    Op(Op),
}

pub fn tokenize(input: String) -> Result<Vec<Token>> {
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
