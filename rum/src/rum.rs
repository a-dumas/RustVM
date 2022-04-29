use std::{env, process};
use std::fs::File;
use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
use rum::um_exec::Machine;
use rum::memory_loader::Opcode;

// read machine instructions from file
fn read_instructions(filename: &str) -> Vec<u32> {
    let mut f = File::open(filename).expect("File not found");
    let mut data = Vec::new();
    let mut instructions = Vec::new();

    match f.read_to_end(&mut data) {
        Ok(_) => {
            for i in 0..data.len() / 4 {
                let idx = i * 4;
                let buf = &data[idx..idx + 4];
                let mut rdr = Cursor::new(buf);
                instructions.push(rdr.read_u32::<BigEndian>().unwrap());
            }

            instructions
        },
        Err(error) => panic!("{} error while reading from {}", error, filename)
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = read_instructions(filename);
    let mut machine = Machine::new(instructions);
    let mut pc: u32 = 0;

    // "infinite loop" - only way to exit is to hit the "Halt" opcode
    loop { // run the machine until it terminates
        let instruction = &machine.get_instruction(pc);


        // eprintln!("#{}: {:?} {:?}", pc, instruction, machine.registers);
        // eprintln!("#{}: {:?}", pc, instruction);
        
        pc += 1;
        // eprintln!("{:?}", instruction.op);
        
        match instruction.op {
            Opcode::CMov => machine.cmov(instruction),
            Opcode::Load => machine.load(instruction),
            Opcode::Store => machine.store(instruction),
            Opcode::Add => machine.add(instruction),
            Opcode::Mul => machine.mul(instruction),
            Opcode::Div => machine.div(instruction),
            Opcode::Nand => machine.nand(instruction),
            Opcode::Halt => process::exit(0),
            Opcode::MapSegment => machine.map_segment(instruction),
            Opcode::UnmapSegment => machine.unmap_segment(instruction),
            Opcode::Output => machine.output(instruction),
            Opcode::Input => machine.input(instruction),
            Opcode::LoadProgram => pc = machine.load_program(instruction),
            Opcode::LoadValue => machine.load_value(instruction),
            Opcode::Err => panic!(
                "Invalid opcode for instruction {:?}", instruction
            )
        }
    }
}