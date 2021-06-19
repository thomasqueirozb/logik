use crate::ast::{FuncCallNode, FuncDefNode, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::rc::Rc;

pub struct Assembler {
    text: String,
    funcs: Rc<RefCell<HashMap<String, FuncDefNode>>>,
    id: usize,
}

impl Assembler {
    pub fn new(base_file: String, funcs: &Rc<RefCell<HashMap<String, FuncDefNode>>>) -> Self {
        let mut text = fs::read_to_string(base_file).unwrap();
        text.push('\n');

        Self {
            text,
            funcs: funcs.clone(),
            id: 0,
        }
    }

    pub fn assemble(&mut self) {
        let fc = FuncCallNode::new("main".to_string(), vec![], &self.funcs);
        fc.assemble(self, &mut HashMap::new(), &mut 0);
        self.push_line("pop ebp\nmov eax, 1\nmov ebx, 0\nint 0x80");

        let mut out = fs::File::create("out.asm").unwrap();
        out.write_all(self.text.as_bytes()).unwrap();
    }

    pub fn push_line<'a>(&mut self, s: &'a str) {
        self.text.push_str(s.as_ref());
        self.text.push('\n');
    }

    pub fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }
}
