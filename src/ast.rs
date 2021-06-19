use crate::operator::{CondOp, Op};
use crate::token::Number;
use crate::variable::*;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Node: Debug + Any {
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData;

    fn as_any(&self) -> &dyn Any;
}

// Binary Node
#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let n1 = self.left_child.eval(vars);
        let n2 = self.right_child.eval(vars);

        // println!(
        //     "n1 {:?} n2 {:?}",
        //     self.left_child.eval(vars),
        //     self.right_child.eval(vars)
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

#[derive(Debug)]
pub enum UnaryNodeKind {
    Pos,
    Neg,
    Not,
}

// Unary Node
#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let eval = self.child.eval(vars);
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
#[derive(Debug)]
pub struct NumberNode {
    child: Box<dyn Node>,
}

impl NumberNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for NumberNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let v = self.child.eval(vars);
        match v {
            VariableData::Number(_) => v,
            _ => panic!("NumberNode"),
        }
    }
}

#[derive(Debug)]
pub struct NumberLiteralNode {
    value: Number,
}

impl NumberLiteralNode {
    pub fn new(value: Number) -> Self {
        Self { value }
    }
}

impl Node for NumberLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, _vars: &mut HashMap<String, Variable>) -> VariableData {
        VariableData::Number(self.value)
    }
}

#[derive(Debug)]
pub struct SimpleVariableNode {
    value: VariableData,
}

impl SimpleVariableNode {
    pub fn new(value: VariableData) -> Self {
        Self { value }
    }
}

impl Node for SimpleVariableNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, _vars: &mut HashMap<String, Variable>) -> VariableData {
        self.value.clone()
    }
}

// Bool Node
#[derive(Debug)]
pub struct BoolNode {
    child: Box<dyn Node>,
}

impl BoolNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for BoolNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let v = self.child.eval(vars);
        match v {
            VariableData::Bool(_) => v,
            _ => panic!("BoolNode"),
        }
    }
}

#[derive(Debug)]
pub struct BoolLiteralNode {
    value: bool,
}

impl BoolLiteralNode {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Node for BoolLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, _vars: &mut HashMap<String, Variable>) -> VariableData {
        VariableData::Bool(self.value)
    }
}

// String Node
#[derive(Debug)]
pub struct StringNode {
    child: Box<dyn Node>,
}

impl StringNode {
    pub fn new(child: Box<dyn Node>) -> Self {
        Self { child }
    }
}

impl Node for StringNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let v = self.child.eval(vars);
        match v {
            VariableData::String(_) => v,
            _ => panic!("StringNode"),
        }
    }
}

#[derive(Debug)]
pub struct StringLiteralNode {
    value: String,
}

impl StringLiteralNode {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteralNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, _vars: &mut HashMap<String, Variable>) -> VariableData {
        VariableData::String(self.value.clone())
    }
}

// Declare Node
#[derive(Debug)]
pub struct DeclareNode {
    name: String,
    expression: Option<Box<dyn Node>>,
    kind: VariableKind,
}
impl DeclareNode {
    pub fn new(name: String, expression: Option<Box<dyn Node>>, kind: VariableKind) -> Self {
        Self {
            name,
            expression,
            kind,
        }
    }
}

impl Node for DeclareNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let eval = match &self.expression {
            Some(ex) => Some(ex.eval(vars)),
            None => None,
        };

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
        vars.insert(self.name.clone(), v);
        // println!("vars {:#?}", vars);

        VariableData::None
    }
}

// Assign Node
#[derive(Debug)]
pub struct AssignNode {
    name: String,
    expression: Box<dyn Node>,
}
impl AssignNode {
    pub fn new(name: String, expression: Box<dyn Node>) -> Self {
        Self { name, expression }
    }
}

impl Node for AssignNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let eval = self.expression.eval(vars);

        let var = vars.get_mut(&self.name).unwrap();

        let kind = var.kind;

        let v = Variable::match_data_kind(eval, kind);
        var.data = Some(v);

        VariableData::None
    }
}

// Variable Node
#[derive(Debug)]
pub struct VariableNode {
    name: String,
}
impl VariableNode {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Node for VariableNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let val = vars.get(&self.name);
        // if val.is_none() {
        //     println!("none {:?}", vars);
        // }

        let val = val.expect("variable used before assignment");
        val.data.clone().unwrap()
    }
}

// Cond Node
#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        let b = match self.cond {
            CondOp::LT => self.left_child.eval(vars) < self.right_child.eval(vars),
            CondOp::LEQ => self.left_child.eval(vars) <= self.right_child.eval(vars),
            CondOp::GT => self.left_child.eval(vars) > self.right_child.eval(vars),
            CondOp::GEQ => self.left_child.eval(vars) >= self.right_child.eval(vars),
            CondOp::EQ => self.left_child.eval(vars) == self.right_child.eval(vars),
            CondOp::NEQ => self.left_child.eval(vars) != self.right_child.eval(vars),
            CondOp::And => {
                self.left_child.eval(vars) != VariableData::Number(0)
                    && self.right_child.eval(vars) != VariableData::Number(0)
            }
            CondOp::Or => {
                self.left_child.eval(vars) != VariableData::Number(0)
                    || self.right_child.eval(vars) != VariableData::Number(0)
            }
        };
        VariableData::Bool(b)
    }
}

// If Node
#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        if self.cond.eval(vars) != VariableData::Number(0) {
            self.if_child.eval(vars)
        } else if let Some(child) = &self.else_child {
            child.eval(vars)
        } else {
            VariableData::None
        }
    }
}

// While Node
#[derive(Debug)]
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
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        while self.cond.eval(vars) != VariableData::Number(0) {
            self.child.eval(vars);
        }
        VariableData::None
    }
}

// Block Node
#[derive(Debug)]
pub struct BlockNode {
    children: Vec<Box<dyn Node>>,
}

impl BlockNode {
    pub fn new(children: Vec<Box<dyn Node>>) -> Self {
        BlockNode { children }
    }
}

impl Node for BlockNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        for child in self.children.iter() {
            let any_child = child.as_any();

            let return_node = any_child.downcast_ref::<ReturnNode>().is_some();
            let v = child.eval(vars);

            if return_node || v != VariableData::None {
                return v;
            }
        }
        VariableData::None
    }
}

// Func Node
#[derive(Debug)]
pub struct FuncCallNode {
    name: String,
    params: Rc<RefCell<Vec<Box<dyn Node>>>>,
    funcs: Rc<RefCell<HashMap<String, FuncDefNode>>>,
}

impl FuncCallNode {
    pub fn new(
        name: String,
        params: Vec<Box<dyn Node>>,
        funcs: &Rc<RefCell<HashMap<String, FuncDefNode>>>,
    ) -> Self {
        FuncCallNode {
            name,
            params: Rc::new(RefCell::new(params)),
            funcs: funcs.clone(),
        }
    }
}

impl Node for FuncCallNode {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        match self.name.as_ref() {
            "println" => {
                let borrow = self.params.borrow();
                assert_eq!(borrow.len(), 1);
                let eval = borrow[0].eval(vars);
                match eval {
                    VariableData::Number(n) => println!("{}", n),
                    VariableData::Bool(b) => println!("{}", b as Number),
                    VariableData::String(s) => println!("{}", s),
                    VariableData::None => panic!("Print None"),
                };
                VariableData::None
            }
            "readln" => {
                let borrow = self.params.borrow();
                assert_eq!(borrow.len(), 0);
                use std::io;

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input: Number = input.trim().parse().unwrap(); // Maybe do ? here
                VariableData::Number(input)
            }
            _ => {
                let mut new_vars = HashMap::new();
                let fborrow = self.funcs.borrow(); // NOTE: borrow
                let pborrow = self.params.borrow(); // NOTE: borrow
                if let Some(func) = fborrow.get(&self.name) {
                    assert_eq!(pborrow.len(), func.args.len());
                    let plen = pborrow.len();

                    for idx in 0..plen {
                        let param = &pborrow[idx]; //.pop().unwrap();
                        let (arg_kind, arg_name) = &func.args[idx];
                        let param = param.eval(vars);

                        let d_node = DeclareNode::new(
                            arg_name.clone(),
                            Some(Box::new(SimpleVariableNode::new(param))),
                            *arg_kind,
                        );

                        d_node.eval(&mut new_vars);
                    }

                    for child in func.code.children.iter() {
                        let any_child = child.as_any();

                        let return_node = any_child.downcast_ref::<ReturnNode>().is_some();
                        let func_node = any_child.downcast_ref::<FuncCallNode>().is_some();

                        let v = child.eval(&mut new_vars);

                        if return_node || (!func_node && v != VariableData::None) {
                            return Variable::match_data_kind(v, func.kind);
                        }
                    }
                    VariableData::None
                } else {
                    panic!("Function not in funcs")
                }
            }
        }
    }
}

// Return Node
#[derive(Debug)]
pub struct ReturnNode {
    child: Option<Box<dyn Node>>,
}

impl ReturnNode {
    pub fn new(child: Option<Box<dyn Node>>) -> Self {
        Self { child }
    }
}

impl Node for ReturnNode {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData {
        if let Some(c) = &self.child {
            c.eval(vars)
        } else {
            VariableData::Number(1) // WARNING FIXME
        }
    }
}

// FuncDef Node
#[derive(Debug)]
pub struct FuncDefNode {
    kind: VariableKind,
    name: String,
    args: Vec<(VariableKind, String)>,
    code: BlockNode,
}

impl FuncDefNode {
    pub fn new(
        kind: VariableKind,
        name: String,
        args: Vec<(VariableKind, String)>,
        code: BlockNode,
    ) -> Self {
        Self {
            kind,
            name,
            args,
            code,
        }
    }
}
