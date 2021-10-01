//! All CPU components

use crate::data::{Datum, Value, Address, Instruction};

pub struct MAR {
    pub address: Option<Address>,
}

pub struct MDR {
    pub content: Option<Datum>,
}

pub struct CIR {
    pub instruction: Option<Instruction>,
}

pub struct Accumulator {
    pub content: Option<Value>,
}

impl MAR {
    pub fn new() -> MAR {
        MAR {address: None}
    }
    
    pub fn set(&mut self, address: Address) {
        self.address = Some(address);
    }
}

impl MDR {
    pub fn new() -> MDR {
        MDR {content: None}
    }
    
    pub fn set(&mut self, content: Datum) {
        self.content = Some(content);
    }
}

impl CIR {
    pub fn new() -> CIR {
        CIR {instruction: None}
    }
    
    pub fn set(&mut self, instruction: Instruction) {
        self.instruction = Some(instruction);
    }
}

impl Accumulator {
    pub fn new() -> Accumulator {
        Accumulator {content: None}
    }
    
    pub fn set(&mut self, content: Value) {
        self.content = Some(content);
    }
}