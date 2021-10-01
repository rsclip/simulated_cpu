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
use log::*;

#[derive(Clone, Debug)]
pub enum Datum {
    DataValue(Value),
    DataInstruction(Instruction),
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Integer(usize),
    Address(Address),
    None,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Address(pub u8);

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: Value,
}

impl Address {
    #[allow(dead_code)]
    pub fn increment(&mut self) {
        self.0 += 1u8;
        info!("Incremented address to {:?}", self.0);
    }

    #[allow(dead_code)]
    pub fn decrement(&mut self) {
        self.0 -= 1u8;
        info!("Decremented address to {:?}", self.0);
    }
}