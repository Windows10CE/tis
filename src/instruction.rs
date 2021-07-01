use crate::node::*;

#[derive(Clone)]
pub enum Instruction {
    NOP,
    MOV(Location, Location),
    SWP,
    SAV,
    ADD(Location),
    SUB(Location),
    NEG,
    JMP(Label),
    JEZ(Label),
    JNZ(Label),
    JGZ(Label),
    JLZ(Label),
    JRO(i32),
}

impl Instruction {
    pub fn parse(line: &str) -> Instruction {
        let segs: Vec<&str> = line.split(' ').collect();
        
        match segs[0] {
            _ => Instruction::NOP,
        }
    }
    
    pub fn execute(&self, node: &mut Compute) {
        
    }
}

#[derive(Clone)]
pub struct Label {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug)]
pub struct ParseError(pub String);
impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ParseError for type {}", self.0))
    }
}
