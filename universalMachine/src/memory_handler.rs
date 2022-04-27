// allocation and free fo here
// u32 for each identifier and Vec<u32> for the actual space

pub use crate::memory_loader::Instruction;
use std::mem;

// Global Variables
pub const PROGRAM_ADDRESS: usize = 0;

// structure to emulate a memory space
pub struct Memory {
    // stack of reusable memory addresses
    free_addresses: Vec<u32>,

    // heap of reusable memory locations
    heap: Vec<Vec<u32>>,
}

impl Memory {
    pub fn new(instructions: Vec<u32>) -> Memory {
        Memory {
            free_addresses: Vec::new(),
            heap: vec![instructions],
        }
    }

    // allocate memory using a vector of a given size filled with 0s
    pub fn allocate(&mut self, size: usize) -> u32 {
        let zeros_vec = vec![0_u32; size];

        if self.free_addresses.len() == 0 {
            self.heap.push(zeros_vec);
            (self.heap.len() - 1) as u32

        } else {
            let address = self.free_addresses
                .pop()
                .expect("Currently no free addresses available for reuse");

            mem::replace(
                self.heap
                        .get_mut(address as usize)
                        .expect("Memory address is not allocated"),
                zeros_vec
            );

            address
        }
    }

    // deallocate memory at a given memory address
    pub fn deallocate(&mut self, address: u32) {
        self.free_addresses.push(address);

        mem::replace(
            self.heap
                    .get_mut(address as usize)
                    .expect("Memory address is not allocated"),
            Vec::new()
        );
    }

    // return Some(value at a given address) if available, otherwise, return None.
    pub fn get(&self, address: u32) -> Option<&Vec<u32>> {
        self.heap.get(address as usize)
    }

    // write a value into a given address.
    pub fn set(&mut self, address: u32, idx: u32, value: u32) {
        let memory = self.heap.get_mut(address as usize)
            .expect("Memory is not allocated");

        mem::replace(
            memory
                    .get_mut(idx as usize)
                    .expect("Given index does not contain value"),
            value
        );
    }

    // load the program with the vector stored at the given address
    pub fn load(&mut self, address: usize) {
        let program = self.heap
            .get(address)
            .expect("No program found at given address")
            .clone();

        mem::replace(
            self.heap
                    .get_mut(PROGRAM_ADDRESS)
                    .expect("No existing program found"),
            program
        );
    }

    // get the instruction at given program counter
    pub fn get_instruction(&self, pc: u32) -> Instruction {
        match self.heap.get(PROGRAM_ADDRESS) {
            Some(program) => Instruction::new(program[pc as usize]),
            None => panic!("Error! The program was unallocated.")
        }
    }
}


