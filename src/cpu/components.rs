//! All CPU components

use crate::data::{Datum, Value, Address, Instruction};
use log::*;

#[derive(Debug)]
pub struct MAR {
    pub address: Option<Address>,
}

#[derive(Debug)]
pub struct MDR {
    pub content: Option<Datum>,
}

#[derive(Debug)]
pub struct CIR {
    pub instruction: Option<Instruction>,
}

#[derive(Debug)]
pub struct Accumulator {
    pub content: Option<Value>,
}

impl MAR {
    pub fn new() -> MAR {
        MAR {address: None}
    }
    
    pub fn set(&mut self, address: Address) {
        self.address = Some(address);
        info!("Set MAR to {:?}", self.address)
    }

    pub fn get(&self) -> Address {
        info!("Getting MAR: {:?}", self);
        match self.address {
            Some(x) => x.clone(),
            None => panic!("Empty register")
        }
    }
}

impl MDR {
    pub fn new() -> MDR {
        MDR {content: None}
    }
    
    pub fn set(&mut self, content: Datum) {
        self.content = Some(content);
        info!("Set MDR to {:?}", self.content);
    }

    pub fn get(&self) -> Datum {
        info!("Getting MDR: {:?}", self);
        match &self.content {
            Some(x) => x.clone(),
            None => panic!("Empty register")
        }
    }
}

impl CIR {
    pub fn new() -> CIR {
        CIR {instruction: None}
    }
    
    pub fn set(&mut self, instruction: Instruction) {
        self.instruction = Some(instruction);
        info!("Set CIR to {:?}", self.instruction);
    }

    pub fn get(&self) -> Instruction {
        info!("Getting CIR: {:?}", self);
        match self.instruction {
            Some(x) => x.clone(),
            None => panic!("Empty register")
        }
    }
}

impl Accumulator {
    pub fn new() -> Accumulator {
        Accumulator {content: None}
    }
    
    pub fn set(&mut self, content: Value) {
        self.content = Some(content);
        info!("Set ACC to {:?}", self.content);
    }

    pub fn get(&self) -> Value {
        info!("Getting ACC: {:?}", self);
        match self.content {
            Some(x) => x.clone(),
            None => panic!("Empty register")
        }
    }

    /// Add a value to it
    pub fn add(&mut self, other: Value) {
        let other_val = match other {
            Value::Integer(val) => val,
            _ => panic!("invalid value type")
        };

        let current_val = match self.get() {
            Value::Integer(x) => x,
            _ => panic!("cannot add Value::Integer to current value in accumulator (non-integer)")
        };

        info!("Added ACC {:?} + OTHER {:?}", current_val, other_val);
    
        self.set(Value::Integer(current_val + other_val));
    }
}