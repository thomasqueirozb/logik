use crate::operator::{is_operator_char, CondOp, Op};

use std::fmt;

use color_eyre::eyre::{bail, Result};

pub type Number = i64;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(Number),
    CondOp(CondOp),
    Assign,
    Op(Op),
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    Identifier(String),
    String(String),
    If,
    Else,
    While,
    TypeNumber,
    TypeBool,
    TypeString,
    Return,
    True,
    False,
    SemiColon,
    Comma,
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
                CondOp(cop) => cop.to_string(),
                TypeNumber => "int".to_string(),
                TypeBool => "bool".to_string(),
                TypeString => "string".to_string(),
                Return => "return".to_string(),
                String(s) => format!("\"{}\"", s.clone()),
                ParenthesisOpen => "(".into(),
                ParenthesisClose => ")".into(),
                BracketOpen => "{".into(),
                BracketClose => "}".into(),
                Identifier(v) => v.into(),
                Assign => "=".into(),
                If => "If".into(),
                Else => "Else".into(),
                While => "While".into(),
                SemiColon => ";".into(),
                Comma => ",".into(),
                EOF => "EOF".into(),
                True => "true".into(),
                False => "false".into(),
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {}:{}", self.kind, self.line, self.col)
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
    String,
    SemiColon,
    Comma,
}

impl Into<TokenKind> for TokenizerState {
    fn into(self) -> TokenKind {
        match self {
            TokenizerState::SemiColon => TokenKind::SemiColon,
            TokenizerState::Comma => TokenKind::Comma,
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

    let mut string_quote = false;

    let determine_state = |state: &mut TokenizerState, c: char| {
        let prev_state = *state;
        *state = match c {
            ';' => TokenizerState::SemiColon,
            ',' => TokenizerState::Comma,
            c if c.is_numeric() => TokenizerState::Number,

            // Needs to start with alphabetic char or _ but can contain numbers after
            c if (c.is_alphabetic() || c == '_') => TokenizerState::Text,

            c if is_operator_char(c) => TokenizerState::Op,
            c if (c == '(' || c == ')') => TokenizerState::Parenthesis,
            c if (c == '{' || c == '}') => TokenizerState::Bracket,
            c if (c == '"') => TokenizerState::String,
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

        if (c.is_whitespace() || c == '\0' || c == '\n')
            && state != TokenizerState::Comment
            && state != TokenizerState::String
        {
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

                TokenizerState::String => match c {
                    '"' => {
                        if !string_quote {
                            string_quote = true;
                            state = TokenizerState::None;
                        }
                    }
                    _ => buffer.push(PreToken::new(line, col, c)),
                },

                TokenizerState::Text => {
                    if determine_state(&mut state, c)? {
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

                TokenizerState::String => {
                    string_quote = false;
                    let s = buffer.iter().map(|tk| tk.c).collect::<String>();
                    let tk = buffer[0]; // Will crash if buffer has no elements
                    tokens.push(Token::new(tk.line, tk.col, TokenKind::String(s)))
                }

                TokenizerState::Op => {
                    let mut it = buffer.iter().peekable().into_iter();

                    let mut skip_next = false;
                    while let Some(tk) = it.next() {
                        if skip_next {
                            skip_next = false;
                            continue;
                        }

                        if let Some(ntk) = it.peek() {
                            if let Some(cop) = CondOp::from_chars(tk.c, ntk.c) {
                                skip_next = true;
                                tokens.push(Token::new(tk.line, tk.col, TokenKind::CondOp(cop)));
                                continue;
                            }

                            // if let Some(bop) = BoolOp::from_chars(tk.c, ntk.c) {
                            //     skip_next = true;
                            //     tokens.push(Token::new(tk.line, tk.col, TokenKind::BoolOp(bop)));
                            //     continue;
                            // }
                        }

                        let t = {
                            if let Some(cop) = CondOp::from_char(tk.c) {
                                TokenKind::CondOp(cop)
                            } else if let Some(op) = Op::from_char(tk.c) {
                                TokenKind::Op(op)
                            } else {
                                // Not == and not any other operator so it has to be assign
                                assert_eq!(tk.c, '=');
                                TokenKind::Assign
                            }
                        };
                        tokens.push(Token::new(tk.line, tk.col, t));
                    }
                }

                TokenizerState::Parenthesis => {
                    for tk in &buffer {
                        let par = match tk.c {
                            '(' => TokenKind::ParenthesisOpen,
                            ')' => TokenKind::ParenthesisClose,
                            _ => unreachable!(),
                        };
                        match par {
                            TokenKind::ParenthesisOpen => parenthesis_count += 1,
                            TokenKind::ParenthesisClose => {
                                if parenthesis_count == 0 {
                                    bail!("Too many closing parenthesis");
                                }
                                parenthesis_count -= 1;
                            }
                            _ => unreachable!(),
                        };
                        tokens.push(Token::new(tk.line, tk.col, par));
                    }
                }

                TokenizerState::Bracket => {
                    for tk in &buffer {
                        let bracket = match tk.c {
                            '{' => {
                                bracket_count += 1;
                                TokenKind::BracketOpen
                            }
                            '}' => {
                                if bracket_count == 0 {
                                    bail!("Too many closing brackets");
                                }
                                bracket_count -= 1;
                                TokenKind::BracketClose
                            }
                            _ => unreachable!(),
                        };
                        tokens.push(Token::new(tk.line, tk.col, bracket));
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
                            "int" => TokenKind::TypeNumber,
                            "bool" => TokenKind::TypeBool,
                            "string" => TokenKind::TypeString,
                            "true" => TokenKind::True,
                            "false" => TokenKind::False,
                            "return" => TokenKind::Return,
                            _ => TokenKind::Identifier(s),
                        };
                        let tk = buffer[0];
                        tokens.push(Token::new(tk.line, tk.col, tkind))
                    }
                }

                TokenizerState::SemiColon | TokenizerState::Comma => {
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
            TokenizerState::None | TokenizerState::Comment | TokenizerState::String => {}
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
    // Ok(dbg!(tokens))
    Ok(tokens)
}
