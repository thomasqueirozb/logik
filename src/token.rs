use crate::operator::Op;
use crate::parenthesis::Parenthesis;

use anyhow::{bail, Context, Result};

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Number(i64),
    Op(Op),
    Parenthesis(Parenthesis),
    EOF,
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
        Parenthesis,
    };

    let mut state = TokenizerState::None;
    let mut parenthesis_count = 0;

    let determine_state = |state: &mut TokenizerState, c: char| {
        let prev_state = state.clone();
        *state = match c {
            _ if c.is_numeric() => TokenizerState::Number,
            _ if Op::is_valid(c) => TokenizerState::Op,
            _ if Parenthesis::is_valid(c) => TokenizerState::Parenthesis,
            _ => bail!("Unparsable char '{}'", c),
        };
        Ok(prev_state != *state)
    };

    for (_pos, c) in input.chars().chain(std::iter::once('\0')).enumerate() {
        let prev_state = state;
        if (c.is_whitespace() || c == '\0') && state != TokenizerState::Comment {
            state = TokenizerState::None;
        } else {
            match state {
                TokenizerState::None | TokenizerState::Number | TokenizerState::Parenthesis => {
                    // State ALWAYS changes to something other than None
                    determine_state(&mut state, c)?;
                }
                TokenizerState::Op => {
                    if !determine_state(&mut state, c)? {
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
                            buffer.clear();
                            state = TokenizerState::None
                        }
                    }
                    _ => buffer.clear(),
                },
            }
            // println!("c {}, state {:?}, buffer {}", c, state, buffer);
        }

        if prev_state != state {
            match prev_state {
                TokenizerState::None | TokenizerState::Comment => {}

                TokenizerState::Op => {
                    for op_c in buffer.chars() {
                        tokens.push(Token::Op(Op::from_char(op_c).context("Op conv")?));
                    }
                    buffer.clear();
                }

                TokenizerState::Parenthesis => {
                    for parenthesis_c in buffer.chars() {
                        let par =
                            Parenthesis::from_char(parenthesis_c).context("Parenthesis conv")?;
                        match par {
                            Parenthesis::Open => parenthesis_count += 1,
                            Parenthesis::Close => parenthesis_count -= 1,
                        };
                        tokens.push(Token::Parenthesis(par));
                    }
                    buffer.clear();
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

    if parenthesis_count > 0 {
        bail!("Unclosed parenthesis");
    } else if parenthesis_count < 0 {
        bail!("Too many closing parenthesis");
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}
