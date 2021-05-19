use crate::operator::Op;
use crate::token::Number;

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Node {
    fn eval(&self) -> Number;
}

// Binary Node
pub struct BinaryNode<'a> {
    op: Op,
    left_child: Box<dyn Node + 'a>,
    right_child: Box<dyn Node + 'a>,
}

impl<'a> BinaryNode<'a> {
    pub fn new(op: Op, left_child: Box<dyn Node + 'a>, right_child: Box<dyn Node + 'a>) -> Self {
        Self {
            op,
            left_child,
            right_child,
        }
    }
}

impl<'a> Node for BinaryNode<'a> {
    fn eval(&self) -> Number {
        self.op
            .execute(self.left_child.eval(), self.right_child.eval())
    }
}

// Unary Node
pub struct UnaryNode<'a> {
    child: Box<dyn Node + 'a>,
    value: Number,
}

impl<'a> UnaryNode<'a> {
    pub fn new(value: Number, child: Box<dyn Node + 'a>) -> Self {
        Self { child, value }
    }
}

impl<'a> Node for UnaryNode<'a> {
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
pub struct VariableNode<'a> {
    name: String,
    vars: &'a Rc<Cell<HashMap<String, Number>>>,
}

impl<'a> VariableNode<'a> {
    pub fn new(name: String, vars: &'a Rc<Cell<HashMap<String, Number>>>) -> Self {
        Self { name, vars }
    }
}

impl<'a> Node for VariableNode<'a> {
    fn eval(&self) -> Number {
        1
        // WARNING
        // use std::borrow::*; // FIXME
        // self.vars
        //     .borrow()

        //     .get(&self.name)
        //     .expect("variable used before assignment")
        //     .clone() // FIXME remove expect
    }
}
