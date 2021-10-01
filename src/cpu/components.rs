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
}

impl MDR {
    pub fn new() -> MDR {
        MDR {content: None}
    }
}

impl CIR {
    pub fn new() -> CIR {
        CIR {instruction: None}
    }
}

impl Accumulator {
    pub fn new() -> Accumulator {
        Accumulator {content: None}
    }
}