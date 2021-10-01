//! All operation instructions

use crate::data::{Value, Datum};
use crate::cpu::CPU;
use log::*;

pub fn LOAD_VAL(cpu: &mut CPU, operand: Value) {
    info!("Called LOAD_VAL w/ operand {:?}", operand);
    // load value and store it into CPU's accumulator
    let value: Value = load_mem_value(cpu, operand);

    cpu.accumulator.set(value);
}

pub fn ADD_VAL(cpu: &mut CPU, operand: Value) {
    info!("Called ADD_VAL w/ operand {:?}", operand);
    // load a value and add it onto accumulator value
    let value: Value = load_mem_value(cpu, operand);
    cpu.accumulator.add(value);
}

pub fn STORE_VAL(cpu: &mut CPU, operand: Value) {
    info!("Called STORE_VAL w/ operand {:?}", operand);
    // store current accumulator value to RAM address
    let val = Datum::DataValue(cpu.accumulator.get());

    let address = match operand {
        Value::Address(x) => x,
        _ => panic!("cannot store value to non-address")
    };

    if let Some(ref mut x) = cpu.RAM {
        x.set(address, val);
    }
}

pub fn JUMP(cpu: &mut CPU, operand: Value) {
    info!("Called JUMP w/ operand {:?}", operand);
    // set the program counter's address
    cpu.program_counter = match operand {
        Value::Address(x) => x,
        _ => panic!("cannot jump to non-address")
    };
}


/// Load a DataValue from a memory address
fn load_mem_value(cpu: &mut CPU, operand: Value) -> Value {
    match operand {
        Value::Address(addr) => {
            match cpu.get_ram().from_addr(addr) {
                Ok(Datum::DataValue(x)) => x,
                Err(_) => panic!("couldnt retrieve data from memory address"),
                _ => panic!("invalid datum type (not Datum::DataValue)")
            }
        },
        _ => panic!("attempted to load non-address"),
    }
}