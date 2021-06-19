use crate::token::Number;
use std::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct Variable {
    pub kind: VariableKind,
    pub data: Option<VariableData>,
}

impl Variable {
    pub fn new(kind: VariableKind, data: Option<VariableData>) -> Self {
        Self { kind, data }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd)]
pub enum VariableKind {
    String,
    Number,
    Bool,
    None,
}

#[derive(Clone, Debug, Hash)]
pub enum VariableData {
    String(String),
    Number(Number),
    Bool(bool),
    None,
}

impl PartialEq for VariableData {
    fn eq(&self, other: &Self) -> bool {
        let s_val: Number = match self {
            VariableData::Bool(b) => *b as Number,
            VariableData::Number(n) => *n,
            VariableData::String(s1) => {
                if let VariableData::String(s2) = other {
                    return s1 == s2;
                }
                panic!("Compare string with non string")
            }
            VariableData::None => {
                panic!("Compare None")
            }
        };

        let o_val: Number = match other {
            VariableData::Bool(b) => *b as Number,
            VariableData::Number(n) => *n,
            VariableData::String(_s1) => {
                panic!("Compare string with non string")
            }
            VariableData::None => {
                panic!("Compare None")
            }
        };
        s_val == o_val
    }
}

impl PartialOrd for VariableData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s_val: Number = match self {
            VariableData::Bool(b) => *b as Number,
            VariableData::Number(n) => *n,
            VariableData::String(_) => {
                panic!("Ord string")
            }
            VariableData::None => {
                panic!("Ord None")
            }
        };

        let o_val: Number = match other {
            VariableData::Bool(b) => *b as Number,
            VariableData::Number(n) => *n,
            VariableData::String(_) => {
                panic!("Ord string")
            }
            VariableData::None => {
                panic!("Ord None")
            }
        };
        s_val.partial_cmp(&o_val)
        // self.height.partial_cmp(&other.height)
    }
}

impl From<Number> for VariableData {
    fn from(n: Number) -> Self {
        Self::Number(n)
    }
}
