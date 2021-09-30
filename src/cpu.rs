//! Central Processing Unit

use crate::ram;
use crate::data::{Datum, Value, Address, Instruction};

/// Main CPU struct
pub struct CPU {
    pub RAM: &'static ram::RAM,

    // components
    program_counter: ProgramCounter,    // stores the next instruction's mem address
    memory_address_register: MAR,       // stores the address of the current instruction being fetched
    memory_data_register: MDR,          // stores the contents of the memory address
    current_instruction_register: CIR,  // stores the current instruction
    accumulator: Accumulator,           // stores the arithmetic or logic result
}

struct ProgramCounter {
}

struct MAR {}

struct MDR {}

struct CIR {}

struct Accumulator {}