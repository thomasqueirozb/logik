use crate::operator::{CondOp, Op};
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

pub enum UnaryNodeKind {
    Pos,
    Neg,
    Not,
}
// Unary Node
pub struct UnaryNode {
    child: Box<dyn Node>,
    kind: UnaryNodeKind,
}

impl UnaryNode {
    pub fn new(kind: UnaryNodeKind, child: Box<dyn Node>) -> Self {
        Self { child, kind }
    }
}

impl Node for UnaryNode {
    fn eval(&self) -> Number {
        match self.kind {
            UnaryNodeKind::Pos => self.child.eval(),
            UnaryNodeKind::Neg => -self.child.eval(),
            UnaryNodeKind::Not => !self.child.eval(),
        }
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

// // Declare Node
// pub struct DeclareNode {
//     name: String,
//     vars: Rc<RefCell<HashMap<String, Number>>>,
// }
// impl DeclareNode {
//     pub fn new(name: String, vars: &Rc<RefCell<HashMap<String, Number>>>) -> Self {
//         Self {
//             name,
//             vars: vars.clone(),
//         }
//     }
// }

// Assign Node
pub struct AssignNode {
    name: String,
    expression: Box<dyn Node>,
    vars: Rc<RefCell<HashMap<String, Number>>>,
}
impl AssignNode {
    pub fn new(
        name: String,
        expression: Box<dyn Node>,
        vars: &Rc<RefCell<HashMap<String, Number>>>,
    ) -> Self {
        Self {
            name,
            expression,
            vars: vars.clone(),
        }
    }
}

impl Node for AssignNode {
    fn eval(&self) -> Number {
        let eval = self.expression.eval();
        let mut borrow = self.vars.borrow_mut(); // NOTE borrow_mut

        // WARNING FIXME *borrow.get_mut(&self.name).unwrap() = value
        borrow.insert(self.name.clone(), eval);

        0
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
        let borrow = self.vars.borrow(); // NOTE borrow

        let val = *borrow
            .get(&self.name)
            .expect("variable used before assignment"); // FIXME remove expect/ make eval return a result
        val
    }
}

// Cond Node
pub struct CondNode {
    cond: CondOp,
    left_child: Box<dyn Node>,
    right_child: Box<dyn Node>,
}

impl CondNode {
    pub fn new(cond: CondOp, left_child: Box<dyn Node>, right_child: Box<dyn Node>) -> Self {
        CondNode {
            cond,
            left_child,
            right_child,
        }
    }
}

impl Node for CondNode {
    fn eval(&self) -> Number {
        let b = match self.cond {
            CondOp::LT => self.left_child.eval() < self.right_child.eval(),
            CondOp::LEQ => self.left_child.eval() <= self.right_child.eval(),
            CondOp::GT => self.left_child.eval() > self.right_child.eval(),
            CondOp::GEQ => self.left_child.eval() >= self.right_child.eval(),
            CondOp::EQ => self.left_child.eval() == self.right_child.eval(),
            CondOp::NEQ => self.left_child.eval() != self.right_child.eval(),
            CondOp::And => self.left_child.eval() != 0 && self.right_child.eval() != 0,
            CondOp::Or => self.left_child.eval() != 0 || self.right_child.eval() != 0,
        };
        b as Number
    }
}

// If Node
pub struct IfNode {
    cond: Box<dyn Node>,
    if_child: Box<dyn Node>,
    else_child: Option<Box<dyn Node>>,
}

impl IfNode {
    pub fn new(
        cond: Box<dyn Node>,
        if_child: Box<dyn Node>,
        else_child: Option<Box<dyn Node>>,
    ) -> Self {
        IfNode {
            cond,
            if_child,
            else_child,
        }
    }
}

impl Node for IfNode {
    fn eval(&self) -> Number {
        if self.cond.eval() != 0 {
            self.if_child.eval()
        } else if let Some(child) = &self.else_child {
            child.eval()
        } else {
            0
        }
    }
}

// While Node
pub struct WhileNode {
    cond: Box<dyn Node>,
    child: Box<dyn Node>,
}

impl WhileNode {
    pub fn new(cond: Box<dyn Node>, child: Box<dyn Node>) -> Self {
        WhileNode { cond, child }
    }
}

impl Node for WhileNode {
    fn eval(&self) -> Number {
        while self.cond.eval() != 0 {
            self.child.eval();
        }
        0
    }
}

// Block Node
pub struct BlockNode {
    children: Vec<Box<dyn Node>>,
}

impl BlockNode {
    pub fn new(children: Vec<Box<dyn Node>>) -> Self {
        BlockNode { children }
    }
}

impl Node for BlockNode {
    fn eval(&self) -> Number {
        for child in &self.children {
            child.eval();
        }
        0
    }
}

// Func Node
pub struct FuncNode {
    name: String,
    params: Vec<Box<dyn Node>>,
}

impl FuncNode {
    pub fn new(name: String, params: Vec<Box<dyn Node>>) -> Self {
        FuncNode { name, params }
    }
}

impl Node for FuncNode {
    fn eval(&self) -> Number {
        match self.name.as_ref() {
            "println" => {
                println!("{}", self.params[0].eval());
                0
            }
            "readln" => {
                use std::io;

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input: Number = input.trim().parse().unwrap(); // Maybe do ? here
                input
            }
            _ => unimplemented!(),
        }
    }
}
