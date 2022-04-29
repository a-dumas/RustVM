// logical instructions will live here 
// there should be a struct for the machine here that includes the memory and registers

// add
// mul
// div
// nand

pub use crate::memory_handler::{Memory,PROGRAM_ADDRESS};
pub use crate::memory_loader::Instruction;
use std::io::{Read, stdout, stdin, Write};

// structure to represent the machine (with memory emulation and registers)
pub struct Machine {
    // memory is a defined Memory struct
    pub memory: Memory,

    // registers are eight u32 values, stores in an array
    pub registers: [u32; 8]
}

/*
Some of this stuff is shaping up weird, so let's consider using bitpack like this

let inst = instruction as u64;
let a = bitpack::bitpack::getu(inst, 3, 6).try_into().unwrap();
let b = bitpack::bitpack::getu(inst, 3, 3).try_into().unwrap();
let c = bitpack::bitpack::getu(inst, 3, 0).try_into().unwrap();

*/

impl Machine {
    // initialize a new machine struct with a given set of instruction
    pub fn new(instructions: Vec<u32>) -> Machine {
        Machine {
            memory: Memory::new(instructions),
            registers: [0_u32; 8]
        }
    }

    // get a machine's given instruction at a particular program counter position
    pub fn get_instruction(&self, pc: u32) -> Instruction {
        self.memory.get_instruction(pc)
    }

    // opcode 0 - if $r[C] != 0 then $r[A] := $r[B]
    pub fn cmov(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        if self.registers[c] != 0 {
            let value = self.registers[b];
            self.registers[a] = value;
        }
        
    }

    // opcode 1 - $r[A] := $m[$r[B]][$r[C]]
    pub fn load(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers[b];

        let array = self.memory.get(address)
            .expect("Unallocated array found at the given address");

        let idx = self.registers[c] as usize;

        let value = array[idx];

        self.registers[a] = value;
    }

    // opcode 2 - $m[$r[A]][$r[B]] := $r[C]
    pub fn store(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers[a];
        let idx = self.registers[b];
        let value = self.registers[c];

        self.memory.set(address, idx, value);
    }

    // opcode 3 - $r[A] := ($r[B] + $r[C]) % 2^32
    pub fn add(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = ((self.registers[b] as u64 + self.registers[c] as u64) % 2^32) as u32;

        self.registers[a] = value;
    }

    // opcode 4 - $r[A] := ($r[B] × $r[C]) % 2^32
    pub fn mul(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = ((self.registers[b] as u64 * self.registers[c] as u64) % 2^32) as u32;

        self.registers[a] = value;
    }

    // opcode 5 - $r[A] := ($r[B] ÷ $r[C]) (integer division)
    pub fn div(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.registers[b] / self.registers[c];
        self.registers[a] = value;
    }

    // opcode 6 - $r[A] :=¬($r[B]∧$r[C])
    pub fn nand(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = !(self.registers[b] & self.registers[c]);

        self.registers[a] = value;
    }

    /* TN - UNNECESSARY?
    // DO WE NEEd THIS? or in 'main', we can call an exit instead?
    // opcode 7 - halt computation
    fn halt(&mut self, instruction: Instruction) {
        // we could add a break? Or maybe just have it in main lol
        todo!()
    }
    */

    // opcode 8 - new segment is created with a number of words equal to the value in $r[C]
    pub fn map_segment(&mut self, instruction: Instruction) {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let size = self.registers[c] as usize;

        let address = self.memory.allocate(size);

        self.registers[b] = address as u32;
    }

    // opcode 9 - segment $m[$r[C]] is unmapped
    pub fn unmap_segment(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;
        let address = self.registers[c];

        self.memory.deallocate(address);
    }

    // opcode 10 - value in $r[C] is displayed on the I/O
    // Only values from 0 to 255 are allowed
    pub fn output(&self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;
        let value = self.registers[c];
        let byte = value as u8;
        stdout().write(&[byte]).unwrap();
    }

    // opcode 11 - $r[c] is loaded with I/O input 
    // Must be a value from 0 to 255
    pub fn input(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;

        // let stdin = stdin();

        match stdin().bytes().next().unwrap() { // EOF will be None
            Ok(value) => {
                if value as char == '\n' {
                    self.registers[c] = std::u32::MAX;
                } else {
                    self.registers[c] = value as u32;
                }
            },
            Err(e) => panic!("Error while reading input: {}", e)
        }
    }

    // opcode 12 - $m[$r[B]] is duplicated, program counter is set to point to $m[0][$r[C]]
    pub fn load_program(&mut self, instruction: Instruction) -> u32 {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers[b] as usize;
        if address != PROGRAM_ADDRESS {
            self.memory.load(address);
        }
        // eprintln!("LOAD PROGRAM RC: {}", self.registers[c] as usize);
        self.registers[c]
    }

    // opcode 13 - $r[A] = least significant 25 bits in instruction
    pub fn load_value(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let value = instruction.value.unwrap();
        // eprintln!("LOAD VALUE RA: {}", self.registers[a] as usize);

        self.registers[a] = value;
    }
}

