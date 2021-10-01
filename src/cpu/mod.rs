//! Central Processing Unit

mod components;
mod operations;

use crate::ram::RAM;
use components::{MAR, MDR, CIR, Accumulator};
use crate::data::{Address, Datum};

/// Main CPU struct
pub struct CPU {
    pub RAM: Option<RAM>,

    // components
    program_counter: Address,   // stores the next instruction's mem address
    memory_address_register: MAR,       // stores the address of the current instruction being fetched
    memory_data_register: MDR,          // stores the contents of the memory address
    current_instruction_register: CIR,  // stores the current instruction
    accumulator: Accumulator,           // stores the arithmetic or logic result
}

impl CPU {
    /// Create a new CPU instance
    pub fn new() -> CPU {
        CPU {
            RAM: None,
            program_counter: Address(0u8),
            memory_address_register: MAR::new(),
            memory_data_register: MDR::new(),
            current_instruction_register: CIR::new(),
            accumulator: Accumulator::new(),
        }
    }

    pub fn set_ram(&mut self, RAM: RAM) -> &mut Self {
        self.RAM = Some(RAM);
        self
    }

    pub fn start_cycle(&mut self) {
        self.cycle();
    }

    fn cycle(&mut self) {
        loop {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    fn fetch(&mut self) {
        // get the memory address of the next instruction
        // store in the memory address register
        self.memory_address_register.set(self.program_counter);

        // get the address' contents and store in the MDR
        self.memory_data_register.set(
            match self.get_ram().from_addr(self.memory_address_register.get()) {
                Ok(x) => x,
                Err(_) => panic!("failed to get memory address contents")
            }
        );

        // increment program counter for the next fetch
        self.program_counter.increment();
    }

    fn decode(&mut self) {
        // from the content in the MDR, decode it and store the instruction
        // fortunately there isnt much to decode as Instruction is a struct
        // containing the opcode and operand as separate fields
        self.current_instruction_register.set(
            match self.memory_data_register.get() {
                Datum::DataValue(_) => panic!("attempted to decode a value"),
                Datum::DataInstruction(x) => x,
            }
        );
    }

    fn execute(&mut self) {
        let instruction = self.current_instruction_register.get();

        // match the opcode and execute
        match instruction.opcode {
            0u8 => operations::LOAD_VAL(self, instruction.operand),
            1u8 => operations::ADD_VAL(self, instruction.operand),
            2u8 => operations::STORE_VAL(self, instruction.operand),
            3u8 => operations::JUMP(self, instruction.operand),
            _ => panic!("invalid opcode")
        };
    }

    fn get_ram(&self) -> &RAM {
        match self.RAM {
            Some(ref x) => x,
            None => panic!("No RAM in CPU")
        }
    }
}