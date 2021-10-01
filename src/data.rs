//! Structs containing data stored in RAM

use std::cmp::Eq;
use std::hash::Hash;

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

#[derive(PartialEq, Eq, Hash)]
pub struct Address(pub u8);

pub struct Instruction {
    pub opcode: u8,
    pub operand: Value,
}

impl Address {
    pub fn increment(&mut self) {
        self.0 += 1u8;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1u8;
    }
}