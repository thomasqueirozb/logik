use crate::operator::Op;
use crate::parenthesis::Parenthesis;

use anyhow::{bail, Result};

pub type Number = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Op(Op),
    Parenthesis(Parenthesis),
    Variable(String),
    Equals,
    SemiColon,
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
        Variable,
        Equals,
        SemiColon,
    }

    let mut state = TokenizerState::None;
    let mut parenthesis_count: u32 = 0;

    let determine_state = |state: &mut TokenizerState, c: char| {
        let prev_state = *state;
        *state = match c {
            '=' => TokenizerState::Equals,
            ';' => TokenizerState::SemiColon,
            c if c.is_numeric() => TokenizerState::Number,

            // Needs to start with alphabetic char or _ but can contain numbers after
            c if (c.is_alphabetic() || c == '_') => TokenizerState::Variable,

            c if Op::is_valid(c) => TokenizerState::Op,
            c if Parenthesis::is_valid(c) => TokenizerState::Parenthesis,
            _ => bail!("Unparsable char '{}'", c),
        };
        Ok(prev_state != *state)
    };

    for (_pos, c) in input.chars().chain(std::iter::once('\0')).enumerate() {
        let prev_state = state;
        if (c.is_whitespace() || c == '\0' || c == '\n') && state != TokenizerState::Comment {
            state = TokenizerState::None;
        } else {
            match state {
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

                TokenizerState::Variable => {
                    if determine_state(&mut state, c)? {
                        if state == TokenizerState::Number {
                            state = TokenizerState::Variable;
                        }
                    }
                }
                _ => {
                    // State ALWAYS changes to something other than None
                    determine_state(&mut state, c)?;
                }
            }
            // println!("c {}, state {:?}, buffer {}", c, state, buffer);
        }

        if prev_state != state {
            match prev_state {
                TokenizerState::None | TokenizerState::Comment => {}

                TokenizerState::Op => {
                    for op_c in buffer.chars() {
                        tokens.push(Token::Op(Op::from_char(op_c).unwrap()));
                    }
                }

                TokenizerState::Parenthesis => {
                    for parenthesis_c in buffer.chars() {
                        let par = Parenthesis::from_char(parenthesis_c).unwrap();
                        match par {
                            Parenthesis::Open => parenthesis_count += 1,
                            Parenthesis::Close => {
                                if parenthesis_count == 0 {
                                    bail!("Too many closing parenthesis");
                                }
                                parenthesis_count -= 1;
                            }
                        };
                        tokens.push(Token::Parenthesis(par));
                    }
                }

                TokenizerState::Number => {
                    if buffer.len() != 0 {
                        match buffer.parse::<Number>() {
                            Ok(num) => tokens.push(Token::Number(num)),
                            Err(e) => {
                                bail!("Could not convert \"{}\" to a number - ({})", buffer, e)
                            }
                        }
                    }
                }

                TokenizerState::Variable => tokens.push(Token::Variable(buffer.clone())),

                TokenizerState::Equals => tokens.push(Token::Equals),
                TokenizerState::SemiColon => tokens.push(Token::SemiColon),
            }

            if prev_state != TokenizerState::None {
                buffer.clear();
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
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}
