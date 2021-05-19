use crate::bracket::Bracket;
use crate::operator::Op;
use crate::parenthesis::Parenthesis;

use std::fmt;

use anyhow::{bail, Result};

pub type Number = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(Number),
    Op(Op),
    Parenthesis(Parenthesis),
    Bracket(Bracket),
    Variable(String),
    If,
    Else,
    While,
    Assign,
    SemiColon,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match self {
                Number(n) => n.to_string(),
                Op(op) => op.to_string(),
                Parenthesis(p) => p.to_string(),
                Bracket(b) => b.to_string(),
                Variable(v) => v.into(),
                If => "If".into(),
                Else => "Else".into(),
                While => "While".into(),
                Assign => "=".into(),
                SemiColon => ";".into(),
                EOF => "EOF".into(),
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: usize,
    pub col: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(line: usize, col: usize, token_type: TokenKind) -> Self {
        Token {
            line,
            col,
            kind: token_type,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct PreToken {
    line: usize,
    col: usize,
    c: char,
}

impl PreToken {
    fn new(line: usize, col: usize, c: char) -> Self {
        PreToken { line, col, c }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum TokenizerState {
    None,
    Number,
    Op,
    Comment,
    Parenthesis,
    Bracket,
    Text,
    Equals,
    SemiColon,
}

impl Into<TokenKind> for TokenizerState {
    fn into(self) -> TokenKind {
        match self {
            TokenizerState::Equals => TokenKind::Assign,
            TokenizerState::SemiColon => TokenKind::SemiColon,
            _ => unreachable!(),
        }
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = vec![];

    let mut buffer: Vec<PreToken> = vec![];

    let mut state = TokenizerState::None;
    let mut parenthesis_count: u32 = 0;
    let mut bracket_count: u32 = 0;

    let determine_state = |state: &mut TokenizerState, c: char| {
        let prev_state = *state;
        *state = match c {
            '=' => TokenizerState::Equals,
            ';' => TokenizerState::SemiColon,
            c if c.is_numeric() => TokenizerState::Number,

            // Needs to start with alphabetic char or _ but can contain numbers after
            c if (c.is_alphabetic() || c == '_') => TokenizerState::Text,

            c if Op::is_valid(c) => TokenizerState::Op,
            c if Parenthesis::is_valid(c) => TokenizerState::Parenthesis,
            c if Bracket::is_valid(c) => TokenizerState::Bracket,
            _ => bail!("Unparsable char '{}'", c),
        };
        Ok(prev_state != *state)
    };

    let mut line = 0;
    let mut col = 0;

    for c in input.chars().chain(std::iter::once('\0')) {
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }

        let prev_state = state;

        if (c.is_whitespace() || c == '\0' || c == '\n') && state != TokenizerState::Comment {
            state = TokenizerState::None;
        } else {
            match state {
                TokenizerState::Op => {
                    if !determine_state(&mut state, c)? {
                        if let Some(lt) = buffer.iter().last() {
                            if lt.c == '/' && c == '*' {
                                buffer.pop();
                                state = TokenizerState::Comment;
                            }
                        }
                    }
                }

                TokenizerState::Comment => match c {
                    '*' => {
                        buffer.clear();
                        buffer.push(PreToken::new(line, col, c));
                    }
                    '/' => {
                        if !buffer.is_empty() {
                            buffer.clear();
                            state = TokenizerState::None
                        }
                    }
                    _ => buffer.clear(),
                },

                TokenizerState::Text => {
                    if !determine_state(&mut state, c)? {
                        if state == TokenizerState::Number {
                            state = TokenizerState::Text;
                        }
                    }
                }

                _ => {
                    // State ALWAYS changes to something other than None
                    determine_state(&mut state, c)?;
                }
            }
        }

        if prev_state != state {
            match prev_state {
                TokenizerState::None | TokenizerState::Comment => {}

                TokenizerState::Op => {
                    for tk in &buffer {
                        tokens.push(Token::new(
                            tk.line,
                            tk.col,
                            TokenKind::Op(Op::from_char(tk.c).unwrap()),
                        ));
                    }
                }

                TokenizerState::Parenthesis => {
                    for tk in &buffer {
                        let par = Parenthesis::from_char(tk.c).unwrap();
                        match par {
                            Parenthesis::Open => parenthesis_count += 1,
                            Parenthesis::Close => {
                                if parenthesis_count == 0 {
                                    bail!("Too many closing parenthesis");
                                }
                                parenthesis_count -= 1;
                            }
                        };
                        tokens.push(Token::new(tk.line, tk.col, TokenKind::Parenthesis(par)));
                    }
                }

                TokenizerState::Bracket => {
                    for tk in &buffer {
                        let bracket = Bracket::from_char(tk.c).unwrap();
                        match bracket {
                            Bracket::Open => bracket_count += 1,
                            Bracket::Close => {
                                if bracket_count == 0 {
                                    bail!("Too many closing parenthesis");
                                }
                                bracket_count -= 1;
                            }
                        };
                        tokens.push(Token::new(tk.line, tk.col, TokenKind::Bracket(bracket)));
                    }
                }

                TokenizerState::Number => {
                    if buffer.len() != 0 {
                        let s = buffer.iter().map(|tk| tk.c).collect::<String>();
                        let tk = buffer[0];
                        match s.parse::<Number>() {
                            Ok(num) => {
                                tokens.push(Token::new(tk.line, tk.col, TokenKind::Number(num)))
                            }
                            Err(e) => {
                                bail!("Could not convert \"{}\" to a number - ({})", s, e)
                            }
                        }
                    }
                }

                TokenizerState::Text => {
                    if buffer.len() != 0 {
                        let s = buffer.iter().map(|tk| tk.c).collect::<String>();
                        let tkind = match s.as_str() {
                            "if" => TokenKind::If,
                            "else" => TokenKind::Else,
                            "while" => TokenKind::While,
                            _ => TokenKind::Variable(s),
                        };
                        let tk = buffer[0];
                        tokens.push(Token::new(tk.line, tk.col, tkind))
                    }
                }

                TokenizerState::SemiColon | TokenizerState::Equals => {
                    for tk in &buffer {
                        tokens.push(Token::new(tk.line, tk.col, prev_state.into()))
                    }
                }
            }

            if prev_state != TokenizerState::None {
                buffer.clear();
            }
        }

        match state {
            TokenizerState::None | TokenizerState::Comment => {}
            _ => buffer.push(PreToken::new(line, col, c)),
        }
    }

    if state == TokenizerState::Comment {
        bail!("Unterminated comment");
    }

    if parenthesis_count > 0 {
        bail!("Unclosed parenthesis");
    }

    if bracket_count > 0 {
        bail!("Unclosed bracket");
    }

    tokens.push(Token::new(line, col, TokenKind::EOF));
    Ok(tokens)
}
