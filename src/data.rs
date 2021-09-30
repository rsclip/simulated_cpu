//! Structs containing data stored in RAM

pub enum Datum {
    DataValue(Value),
    DataInstruction(Instruction),
}

pub enum Value {
    Integer(usize),
    String(String),
    Address(Address),
    None,
}

pub struct Address(pub u8);

pub struct Instruction {
    pub opcode: u8,
    pub operand: Value,
}