use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Parenthesis {
    Open,
    Close,
}

impl Parenthesis {
    pub fn from_char(c: char) -> Option<Self> {
        use Parenthesis::*;
        match c {
            '(' => Some(Open),
            ')' => Some(Close),
            _ => None,
        }
    }

    pub fn is_valid(c: char) -> bool {
        match c {
            '(' => true,
            ')' => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Parenthesis {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Parenthesis::*;
        match c {
            '(' => Ok(Open),
            ')' => Ok(Close),
            _ => Err("Parenthesis: not ( or )"),
        }
    }
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Parenthesis::Open => '(',
                Parenthesis::Close => ')',
            },
        )
    }
}
