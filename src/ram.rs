//! Main memory

use crate::data::{Datum, Value, Address, Instruction};
use std::collections::HashMap;
use log::*;

// fixed capacity but not memory size for simplicity
#[derive(Debug)]
pub struct RAM {
    pub data: HashMap<Address, Datum>,
    pub size: usize,
}

impl RAM {
    /// Create new growable RAM
    pub fn new() -> RAM {
        RAM {
            data: HashMap::new(),
            size: 0,
        }
    }

    /// Set data onto RAM and return itself
    pub fn set(&mut self, address: Address, data: Datum) -> &mut Self {
        info!("Setting address {:?} to data {:?}", address, data);
        self.data.insert(address, data);
        self.size += 1;
        self
    }

    /// Get the contents at the address
    pub fn from_addr(&self, address: Address) -> Result<Datum, String> {
        match self.data.get(&address) {
            Some(x) => Ok(x.clone()),
            None => Err(format!("Couldn't find data at address {:?}", address.0)),
        }
    }
}

impl Instruction {
    pub fn new(opcode: u8, operand: Value) -> Instruction {
        Instruction {
            opcode,
            operand,
        }
    }
}