mod ram;

use ram::{RAM, Datum, Value, Address, Instruction};

/// Return a prepared RAM:
/// 0   load value      4
/// 1   add value       5
/// 2   store value     6
/// 3   stop            
/// 4   5               
/// 5   6               
/// 6                   
fn get_ram() -> RAM {
    let mut ram = RAM::new();
    // add at addr 0u8        an instruction   new instruction opcode 0u8    operand address at 4u8
    ram.append(Address(0u8), Datum::DataInstruction(Instruction::new(0u8, Value::Address(Address(4u8)))))
        .append(Address(1u8), Datum::DataInstruction(Instruction::new(1u8, Value::Address(Address(5u8)))))
        .append(Address(2u8), Datum::DataInstruction(Instruction::new(2u8, Value::Address(Address(6u8)))))
        .append(Address(3u8), Datum::DataInstruction(Instruction::new(3u8, Value::None)))
        .append(Address(4u8), Datum::DataValue(Value::Integer(5)))
        .append(Address(5u8), Datum::DataValue(Value::Integer(6)))
        .append(Address(6u8), Datum::DataValue(Value::None));

    ram
}

fn main() {
    let mut ram = get_ram();
}
