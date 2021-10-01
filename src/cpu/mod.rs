//! Central Processing Unit

mod components;

use crate::ram::RAM;
use components::{MAR, MDR, CIR, Accumulator};
use crate::data::{Address};

/// Main CPU struct
pub struct CPU<'a> {
    pub RAM: Option<&'a RAM>,

    // components
    program_counter: Address,   // stores the next instruction's mem address
    memory_address_register: MAR,       // stores the address of the current instruction being fetched
    memory_data_register: MDR,          // stores the contents of the memory address
    current_instruction_register: CIR,  // stores the current instruction
    accumulator: Accumulator,           // stores the arithmetic or logic result
}

impl<'a> CPU<'a> {
    /// Create a new CPU instance
    pub fn new() -> CPU<'a> {
        CPU {
            RAM: None,
            program_counter: Address(0u8),
            memory_address_register: MAR::new(),
            memory_data_register: MDR::new(),
            current_instruction_register: CIR::new(),
            accumulator: Accumulator::new(),
        }
    }

    pub fn set_ram(&mut self, RAM: &'a RAM) -> &mut Self {
        self.RAM = Some(RAM);
        self
    }

    pub fn start_cycle(&mut self) {
        self.cycle();
    }

    fn cycle(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
        self.cycle();
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
        unimplemented!()
    }

    fn execute(&mut self) {
        unimplemented!()
    }

    fn get_ram(&self) -> &'a RAM {
        match self.RAM {
            Some(x) => x,
            None => panic!("No RAM in CPU")
        }
    }
}