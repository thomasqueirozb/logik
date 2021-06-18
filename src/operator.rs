use std::convert::TryFrom;
use std::fmt;
use std::ops;

pub fn is_operator_char(c: char) -> bool {
    match c {
        '*' => true,
        '/' => true,
        '+' => true,
        '-' => true,
        '!' => true,
        '>' => true,
        '<' => true,
        '=' => true,
        '&' => true,
        '|' => true,
        _ => false,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CondOp {
    LT,
    LEQ,
    GT,
    GEQ,
    EQ,
    NEQ,
    And,
    Or,
}

impl fmt::Display for CondOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CondOp::LT => "<",
                CondOp::LEQ => "<=",
                CondOp::GT => ">",
                CondOp::GEQ => ">=",
                CondOp::EQ => "==",
                CondOp::NEQ => "!=",
                CondOp::And => "&&",
                CondOp::Or => "||",
            },
        )
    }
}

impl CondOp {
    pub fn from_char(c: char) -> Option<Self> {
        use CondOp::*;
        match c {
            '>' => Some(GT),
            '<' => Some(LT),
            _ => None,
        }
    }
    pub fn from_chars(c1: char, c2: char) -> Option<Self> {
        let pair = (c1, c2);
        use CondOp::*;
        match pair {
            ('>', '=') => Some(GEQ),
            ('<', '=') => Some(LEQ),
            ('=', '=') => Some(EQ),
            ('!', '=') => Some(NEQ),
            ('&', '&') => Some(And),
            ('|', '|') => Some(Or),
            _ => None,
        }
    }
}

// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum BoolOp {
//     And,
//     Or,
// }

// impl fmt::Display for BoolOp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 BoolOp::And => "&&",
//                 BoolOp::Or => "||",
//             },
//         )
//     }
// }

// impl BoolOp {
//     pub fn from_chars(c1: char, c2: char) -> Option<Self> {
//         let pair = (c1, c2);
//         use BoolOp::*;
//         match pair {
//             ('&', '&') => Some(And),
//             ('|', '|') => Some(Or),
//             _ => None,
//         }
//     }
// }

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        use Op::*;
        match c {
            '*' => Some(Mul),
            '/' => Some(Div),
            '+' => Some(Add),
            '-' => Some(Sub),
            '!' => Some(Not),
            _ => None,
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
            Op::Not => unimplemented!(),
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
            '!' => Ok(Not),
            _ => Err("Op: Char not in \"*/+-!\""),
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
                Op::Not => '!',
            },
        )
    }
}
