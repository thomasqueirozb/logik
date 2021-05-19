use crate::operator::Op;
use crate::token::Number;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Node {
    fn eval(&self) -> Number;
}

// Binary Node
pub struct BinaryNode {
    op: Op,
    left_child: Box<dyn Node>,
    right_child: Box<dyn Node>,
}

impl BinaryNode {
    pub fn new(op: Op, left_child: Box<dyn Node>, right_child: Box<dyn Node>) -> Self {
        Self {
            op,
            left_child,
            right_child,
        }
    }
}

impl Node for BinaryNode {
    fn eval(&self) -> Number {
        self.op
            .execute(self.left_child.eval(), self.right_child.eval())
    }
}

// Unary Node
pub struct UnaryNode {
    child: Box<dyn Node>,
    value: Number,
}

impl UnaryNode {
    pub fn new(value: Number, child: Box<dyn Node>) -> Self {
        Self { child, value }
    }
}

impl Node for UnaryNode {
    fn eval(&self) -> Number {
        self.value * self.child.eval()
    }
}

// Number Node
pub struct NumberNode {
    value: Number,
}

impl NumberNode {
    pub fn new(value: Number) -> Self {
        Self { value }
    }
}

impl Node for NumberNode {
    fn eval(&self) -> Number {
        self.value
    }
}

// Variable Node
pub struct VariableNode {
    name: String,
    vars: Rc<RefCell<HashMap<String, Number>>>,
}
impl VariableNode {
    pub fn new(name: String, vars: &Rc<RefCell<HashMap<String, Number>>>) -> Self {
        Self {
            name,
            vars: vars.clone(),
        }
    }
}

impl Node for VariableNode {
    fn eval(&self) -> Number {
        *self
            .vars
            .borrow() // WARNING borrow
            .get(&self.name)
            .expect("variable used before assignment") // FIXME remove expect/ make eval return a result
    }
}
