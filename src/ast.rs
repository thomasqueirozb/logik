use crate::assembler::Assembler;
use crate::operator::{CondOp, Op};
use crate::token::Number;
use crate::variable::*;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Node: Debug + Any {
    fn eval(&self, vars: &mut HashMap<String, Variable>) -> VariableData;
    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    );

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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.left_child.assemble(assembler, vars, ebp_offset);
        assembler.push_line("push ebx");
        self.right_child.assemble(assembler, vars, ebp_offset);
        assembler.push_line("pop eax");

        assembler.push_line(self.op.assemble());
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.child.assemble(assembler, vars, ebp_offset);
        match self.kind {
            UnaryNodeKind::Pos => {}
            UnaryNodeKind::Neg => assembler.push_line("xor eax, eax\nsub eax, ebx\nmov eax, ebx"),
            UnaryNodeKind::Not => assembler.push_line("not ebx"),
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.child.assemble(assembler, vars, ebp_offset);
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        assembler.push_line(format!("mov ebx, {}", self.value).as_str())
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        let n = match &self.value {
            VariableData::String(_) => panic!("Cannot convert String to Number"),
            VariableData::Number(n) => *n,
            VariableData::Bool(b) => *b as Number,
            VariableData::None => panic!("Cannot convert None to Number"),
        };
        assembler.push_line(format!("mov ebx, {}", n).as_str());
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
    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.child.assemble(assembler, vars, ebp_offset);
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        let n: Number = self.value.try_into().unwrap();
        assembler.push_line(format!("mov ebx, {}", n).as_str());
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

    fn assemble(
        &self,
        _assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        panic!();
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
    fn assemble(
        &self,
        _assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        panic!();
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        assembler.push_line("push DWORD 0");

        if let Some(e) = &self.expression {
            e.assemble(assembler, vars, ebp_offset);
        }

        *ebp_offset += 4;

        vars.insert(self.name.clone(), (self.kind, *ebp_offset));

        if self.expression.is_some() {
            assembler.push_line(format!("mov [ebp - {}], ebx", ebp_offset).as_str());
        }
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.expression.assemble(assembler, vars, ebp_offset);

        let (_, offset) = vars.get(&self.name).unwrap();

        assembler.push_line(format!("mov [ebp - {}], ebx", offset).as_str());
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        let (_, offset) = vars.get(&self.name).unwrap();

        assembler.push_line(format!("mov ebx, [ebp - {}]", offset).as_str());
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
    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.left_child.assemble(assembler, vars, ebp_offset);
        assembler.push_line("push ebx");
        self.right_child.assemble(assembler, vars, ebp_offset);
        assembler.push_line("pop eax");
        assembler.push_line("cmp eax, ebx");

        let s = match self.cond {
            CondOp::LT => "call binop_jl",
            CondOp::LEQ => "call binop_jle",
            CondOp::GT => "call binop_jg",
            CondOp::GEQ => "call binop_jge",
            CondOp::EQ => "call binop_je",
            CondOp::NEQ => "call binop_jne",
            CondOp::And => "and eax, ebx",
            CondOp::Or => "or ebx, eax",
        };
        assembler.push_line(s);
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        self.cond.assemble(assembler, vars, ebp_offset);
        assembler.push_line("cmp ebx, False");

        let id = assembler.next_id();
        if self.else_child.is_some() {
            assembler.push_line(format!("je else_{}", id).as_str());
        } else {
            assembler.push_line(format!("je end_if_{}", id).as_str());
        }

        self.if_child.assemble(assembler, vars, ebp_offset);

        if let Some(e) = &self.else_child {
            assembler.push_line(format!("jmp end_if_{}", id).as_str());
            assembler.push_line(format!("else_{}:", id).as_str());
            e.assemble(assembler, vars, ebp_offset);
        }
        assembler.push_line(format!("end_if_{}:", id).as_str());
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        let id = assembler.next_id();
        assembler.push_line(format!("while_{}:", id).as_str());
        self.cond.assemble(assembler, vars, ebp_offset);
        assembler.push_line("cmp ebx, False");
        assembler.push_line(format!("je while_end_{}", id).as_str());
        self.child.assemble(assembler, vars, ebp_offset);
        assembler.push_line(format!("jmp while_{}", id).as_str());
        assembler.push_line(format!("while_end_{}:", id).as_str());
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
    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        for child in self.children.iter() {
            child.assemble(assembler, vars, ebp_offset);
        }
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

    fn assemble(
        &self,
        assembler: &mut Assembler,
        vars: &mut HashMap<String, (VariableKind, usize)>,
        ebp_offset: &mut usize,
    ) {
        let fborrow = self.funcs.borrow(); // NOTE: borrow

        match self.name.as_ref() {
            "println" | "print" => {
                let borrow = self.params.borrow();
                assert_eq!(borrow.len(), 1);
                borrow[0].assemble(assembler, vars, ebp_offset);
                assembler.push_line("push ebx\ncall print\npop ebx");
            }
            _ => {
                let func = fborrow.get(&self.name).unwrap();
                for child in func.code.children.iter() {
                    child.assemble(assembler, vars, ebp_offset);
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
    fn assemble(
        &self,
        _assembler: &mut Assembler,
        _vars: &mut HashMap<String, (VariableKind, usize)>,
        _ebp_offset: &mut usize,
    ) {
        unimplemented!()
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
