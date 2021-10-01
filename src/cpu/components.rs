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

    pub fn get(&self) -> Address {
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
    }

    pub fn get(&self) -> Datum {
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
    }

    pub fn get(&self) -> Instruction {
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
    }

    pub fn get(&self) -> Value {
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

        self.set(Value::Integer(current_val + other_val));
    }
}