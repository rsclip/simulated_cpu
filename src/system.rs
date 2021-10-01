//! Contains the entire system
//! single lifetime for all values

use crate::cpu::CPU;
use crate::ram::RAM;

/// Acts as a sort of system bus
pub struct System {
    pub CPU: CPU,
}

impl System {
    pub fn build(RAM: RAM) -> System {
        let mut return_val = System {
            CPU: CPU::new(),
        };
        return_val.CPU.set_ram(RAM);
        return_val
    }
}