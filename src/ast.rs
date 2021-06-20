use crate::operator::{CondOp, Op};
use crate::token::Number;
use crate::variable::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Node {
    fn eval(&self) -> VariableData;
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
    fn eval(&self) -> VariableData {
        let n1 = self.left_child.eval();
        let n2 = self.right_child.eval();

        // println!(
        //     "n1 {:?} n2 {:?}",
        //     self.left_child.eval(),
        //     self.right_child.eval()
        // );

        let n2 = match n2 {
            VariableData::Number(n2) => n2,
            VariableData::Bool(n2) => n2 as Number,
            VariableData::String(_) => panic!("n2 string"),
            VariableData::None => panic!("n2 None"),
        };

        let n1 = match n1 {
            VariableData::Number(n1) => n1,
            VariableData::Bool(n1) => n1 as Number,

            VariableData::String(_) => panic!("n1 string"),
            VariableData::None => panic!("n1 None"),
        };

        self.op.execute(n1, n2).into()
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
    fn eval(&self) -> VariableData {
        let eval = self.child.eval();
        match eval {
            VariableData::Number(n) => {
                let n = match &self.kind {
                    UnaryNodeKind::Pos => n,
                    UnaryNodeKind::Neg => -n,
                    UnaryNodeKind::Not => !n,
                };
                VariableData::Number(n)
            }

            VariableData::Bool(b) => match &self.kind {
                UnaryNodeKind::Pos => VariableData::Bool(b),
                UnaryNodeKind::Neg => VariableData::Number(-(b as Number)),
                UnaryNodeKind::Not => VariableData::Bool(!b),
            },
            _ => unreachable!(),
        }
    }
}

// Number Node
pub struct NumberNode {
    child: Box<dyn Node>,
}

impl NumberNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for NumberNode {
    fn eval(&self) -> VariableData {
        let v = self.child.eval();
        match v {
            VariableData::Number(_) => v,
            _ => panic!("NumberNode"),
        }
    }
}

pub struct NumberLiteralNode {
    value: Number,
}

impl NumberLiteralNode {
    pub fn new(value: Number) -> Self {
        Self { value }
    }
}

impl Node for NumberLiteralNode {
    fn eval(&self) -> VariableData {
        VariableData::Number(self.value)
    }
}

// Bool Node
pub struct BoolNode {
    child: Box<dyn Node>,
}

impl BoolNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for BoolNode {
    fn eval(&self) -> VariableData {
        let v = self.child.eval();
        match v {
            VariableData::Bool(_) => v,
            _ => panic!("BoolNode"),
        }
    }
}

pub struct BoolLiteralNode {
    value: bool,
}

impl BoolLiteralNode {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolLiteralNode {
    fn eval(&self) -> VariableData {
        VariableData::Bool(self.value)
    }
}

// String Node
pub struct StringNode {
    child: Box<dyn Node>,
}

impl StringNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for StringNode {
    fn eval(&self) -> VariableData {
        let v = self.child.eval();
        match v {
            VariableData::String(_) => v,
            _ => panic!("StringNode"),
        }
    }
}

pub struct StringLiteralNode {
    value: String,
}

impl StringLiteralNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteralNode {
    fn eval(&self) -> VariableData {
        VariableData::String(self.value.clone())
    }
}

// Declare Node
pub struct DeclareNode {
    name: String,
    expression: Option<Box<dyn Node>>,
    kind: VariableKind,
    vars: Rc<RefCell<HashMap<String, Variable>>>,
}
impl DeclareNode {
    pub fn new(
        name: String,
        expression: Option<Box<dyn Node>>,
        kind: VariableKind,
        vars: &Rc<RefCell<HashMap<String, Variable>>>,
    ) -> Self {
        Self {
            name,
            expression,
            kind,
            vars: vars.clone(),
        }
    }
}

impl Node for DeclareNode {
    fn eval(&self) -> VariableData {
        let eval = match &self.expression {
            Some(ex) => Some(ex.eval()),
            None => None,
        };

        let mut borrow = self.vars.borrow_mut(); // NOTE borrow_mut

        let v = match eval {
            Some(VariableData::String(_)) => {
                assert_eq!(self.kind, VariableKind::String);
                Variable::new(self.kind, eval)
            }
            Some(VariableData::Number(n)) => {
                if self.kind == VariableKind::Number {
                    Variable::new(self.kind, eval)
                } else if self.kind == VariableKind::Bool {
                    Variable::new(self.kind, Some(VariableData::Bool(n != 0)))
                } else {
                    panic!()
                }
            }

            Some(VariableData::Bool(b)) => {
                if self.kind == VariableKind::Number {
                    Variable::new(self.kind, Some(VariableData::Number(b as Number)))
                } else if self.kind == VariableKind::Bool {
                    Variable::new(self.kind, eval)
                } else {
                    panic!()
                }
            }
            Some(VariableData::None) => {
                assert_eq!(self.kind, VariableKind::None);
                Variable::new(self.kind, eval)
            }
            None => Variable::new(self.kind, eval),
        };

        assert!(borrow.get(&self.name).is_none());
        let sn: &str = self.name.as_ref();
        assert_ne!(sn, "println");
        assert_ne!(sn, "readln");

        borrow.insert(self.name.clone(), v);

        VariableData::None
    }
}

// Assign Node
pub struct AssignNode {
    name: String,
    expression: Box<dyn Node>,
    vars: Rc<RefCell<HashMap<String, Variable>>>,
}
impl AssignNode {
    pub fn new(
        name: String,
        expression: Box<dyn Node>,
        vars: &Rc<RefCell<HashMap<String, Variable>>>,
    ) -> Self {
        Self {
            name,
            expression,
            vars: vars.clone(),
        }
    }
}

impl Node for AssignNode {
    fn eval(&self) -> VariableData {
        let eval = self.expression.eval();
        let mut borrow = self.vars.borrow_mut(); // NOTE borrow_mut

        let var = borrow.get_mut(&self.name).unwrap();

        let kind = var.kind;

        let v = match eval {
            VariableData::String(_) => {
                assert_eq!(kind, VariableKind::String);
                eval
            }
            VariableData::Number(n) => {
                if kind == VariableKind::Number {
                    eval
                } else if kind == VariableKind::Bool {
                    VariableData::Bool(n != 0)
                } else {
                    panic!()
                }
            }

            VariableData::Bool(b) => {
                if kind == VariableKind::Number {
                    VariableData::Number(b as Number)
                } else if kind == VariableKind::Bool {
                    eval
                } else {
                    panic!()
                }
            }
            VariableData::None => {
                assert_eq!(kind, VariableKind::None);
                eval
            }
        };
        var.data = Some(v);
        // *r = eval;

        VariableData::None
    }
}

// Variable Node
pub struct VariableNode {
    name: String,
    vars: Rc<RefCell<HashMap<String, Variable>>>,
}
impl VariableNode {
    pub fn new(name: String, vars: &Rc<RefCell<HashMap<String, Variable>>>) -> Self {
        Self {
            name,
            vars: vars.clone(),
        }
    }
}

impl Node for VariableNode {
    fn eval(&self) -> VariableData {
        let borrow = self.vars.borrow(); // NOTE borrow

        let val = borrow.get(&self.name);

        let val = val.expect("variable used before assignment"); // FIXME remove expect/ make eval return a result
        val.data.clone().unwrap()
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
    fn eval(&self) -> VariableData {
        let b = match self.cond {
            CondOp::LT => self.left_child.eval() < self.right_child.eval(),
            CondOp::LEQ => self.left_child.eval() <= self.right_child.eval(),
            CondOp::GT => self.left_child.eval() > self.right_child.eval(),
            CondOp::GEQ => self.left_child.eval() >= self.right_child.eval(),
            CondOp::EQ => self.left_child.eval() == self.right_child.eval(),
            CondOp::NEQ => self.left_child.eval() != self.right_child.eval(),
            CondOp::And => {
                self.left_child.eval() != VariableData::Number(0)
                    && self.right_child.eval() != VariableData::Number(0)
            }
            CondOp::Or => {
                self.left_child.eval() != VariableData::Number(0)
                    || self.right_child.eval() != VariableData::Number(0)
            }
        };
        VariableData::Bool(b)
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
    fn eval(&self) -> VariableData {
        if self.cond.eval() != VariableData::Number(0) {
            self.if_child.eval();
        } else if let Some(child) = &self.else_child {
            child.eval();
        }
        VariableData::None
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
    fn eval(&self) -> VariableData {
        while self.cond.eval() != VariableData::Number(0) {
            self.child.eval();
        }
        VariableData::None
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
    fn eval(&self) -> VariableData {
        for child in &self.children {
            child.eval();
        }
        VariableData::None
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
    fn eval(&self) -> VariableData {
        match self.name.as_ref() {
            "println" => {
                let eval = self.params[0].eval();
                match eval {
                    VariableData::Number(n) => println!("{}", n),
                    VariableData::Bool(b) => println!("{}", b as Number),
                    VariableData::String(s) => println!("{}", s),
                    VariableData::None => panic!("Print None"),
                };
                VariableData::None
            }
            "readln" => {
                use std::io;

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input: Number = input.trim().parse().unwrap(); // Maybe do ? here
                VariableData::Number(input)
            }
            _ => unimplemented!(),
        }
    }
}
