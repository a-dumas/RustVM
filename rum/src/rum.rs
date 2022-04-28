// this is our main
// initalizing all registers to 0 should happen here 

use std::{env, process};
use std::fs::File;
use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
use rum::um_exec::Machine;
use rumdump::rumdis::Opcode;

// structure to represent the machine (with memory emulation and registers)

/*
pub struct Machine {
    // memory is a defined Memory struct
    pub memory: Memory,

    // registers are eight u32 values, stores in an array
    pub registers: [u32; 8],

    // program counter
    pub counter: u32

}

impl Machine {
    // initialize a new machine struct with a given set of instruction
    pub fn new(&self) -> u32 {
        let instruction = self.memory[0][self.counter as usize];
        self.counter += 1;
        instruction
    }

    pub fn execute(&mut self, word: u32){
		
		// get the opcode
		let opcode: u8 = bitpack::bitpack::getu(word.into(), 4, 28).try_into().unwrap();
		let word_u32: u32 = word.try_into().unwrap();

        // match on the opcode recieved
		match opcode {
			0 =>  cmov(self, word_u32),
			1 =>  load(self, word_u32),
			2 =>  store(self, word_u32),
			3 =>  add(self, word_u32),
			4 =>  mul(self, word_u32),
			5 =>  div(self, word_u32),
			6 =>  nand(self, word_u32),
			7 =>  halt(self),
			8 =>  map_segment(self, word_u32),
			9 =>  unmap_segment(self, word_u32),
			10 => output(self, word_u32),
			11 => input(self, word_u32),
			12 => load_program(self, word_u32),
			13 => load_value(self, word_u32),
			_ => panic!("Invalid instruction.")
			
		};
	}
}
*/


// read machine instructions from file

fn read_instructions(filename: &str) -> Vec<u32> {
    let mut f = File::open(filename).expect("File not found");
    let mut data = Vec::new();
    let mut instructions = Vec::new();

    match f.read_to_end(&mut data) {
        Ok(_) => {
        // Ok(bytes) => {}
            // println!("read {} bytes from {}", bytes, filename);

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
        let instruction = machine.get_instruction(pc);
        pc += 1;

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