use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    // Mul,
    // Div,
    Add,
    Sub,
}

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        use Op::*;
        match c {
            // '*' => Some(Mul),
            // '/' => Some(Div),
            '+' => Some(Add),
            '-' => Some(Sub),
            _ => None,
        }
    }

    pub fn execute<T>(self, lhs: T, rhs: T) -> T
    where
        // T: ops::Mul<Output = T>,
        // T: ops::Div<Output = T>,
        T: ops::Add<Output = T>,
        T: ops::Sub<Output = T>,
    {
        match self {
            // Op::Mul => lhs * rhs,
            // Op::Div => lhs / rhs,
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Op::Mul => '*',
                // Op::Div => '/',
                Op::Add => '+',
                Op::Sub => '-',
            },
        )
    }
}
