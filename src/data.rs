//! Structs containing data stored in RAM
//! 
//! opcodes:
//! | opcode | instruction |
//! |--------|-------------|
//! | 0u8    | LOAD VAL    |
//! | 1u8    | ADD VAL     |
//! | 2u8    | STORE VAL   |
//! | 3u8    | JUMP        |

use std::cmp::Eq;
use std::hash::Hash;

#[derive(Clone)]
pub enum Datum {
    DataValue(Value),
    DataInstruction(Instruction),
}

#[derive(Clone)]
pub enum Value {
    Integer(usize),
    String(String),
    Address(Address),
    None,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Address(pub u8);

#[derive(Clone)]
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