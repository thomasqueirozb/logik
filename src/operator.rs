use std::convert::TryFrom;
use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        use Op::*;
        match c {
            '*' => Some(Mul),
            '/' => Some(Div),
            '+' => Some(Add),
            '-' => Some(Sub),
            _ => None,
        }
    }

    pub fn is_op(c: char) -> bool {
        match c {
            '*' => true,
            '/' => true,
            '+' => true,
            '-' => true,
            _ => false,
        }
    }

    pub fn execute<T>(self, lhs: T, rhs: T) -> T
    where
        T: ops::Mul<Output = T>,
        T: ops::Div<Output = T>,
        T: ops::Add<Output = T>,
        T: ops::Sub<Output = T>,
    {
        match self {
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
        }
    }
}

impl TryFrom<char> for Op {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Op::*;
        match c {
            '*' => Ok(Mul),
            '/' => Ok(Div),
            '+' => Ok(Add),
            '-' => Ok(Sub),
            _ => Err("Op: Char not in \"*/+-\""),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Mul => '*',
                Op::Div => '/',
                Op::Add => '+',
                Op::Sub => '-',
            },
        )
    }
}
