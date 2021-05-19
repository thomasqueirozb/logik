use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bracket {
    Open,
    Close,
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Self> {
        use Bracket::*;
        match c {
            '{' => Some(Open),
            '}' => Some(Close),
            _ => None,
        }
    }

    pub fn is_valid(c: char) -> bool {
        match c {
            '{' => true,
            '}' => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Bracket::*;
        match c {
            '{' => Ok(Open),
            '}' => Ok(Close),
            _ => Err("Bracket: not { or }"),
        }
    }
}

impl fmt::Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Bracket::Open => '{',
                Bracket::Close => '}',
            },
        )
    }
}
