use crate::opcodes::OpCode;

#[derive(Debug)]
pub enum Value {
    Address(String),
    Value(u32), 
    String(String),
}

#[derive(Debug)]
pub struct Instruction {
    pub op_code: OpCode,
    pub values: Vec<Value>,
}