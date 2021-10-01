//! Contains the entire system
//! single lifetime for all values

use crate::cpu::CPU;
use crate::ram::RAM;

/// Acts as a sort of system bus
pub struct System<'a> {
    pub CPU: CPU<'a>,
    pub RAM: &'a RAM,
}

impl<'a> System<'a> {
    pub fn build(RAM: &'a RAM) -> System<'a> {
        let mut return_val = System {
            CPU: CPU::new(),
            RAM,
        };
        return_val.CPU.set_ram(&RAM);
        return_val
    }
}