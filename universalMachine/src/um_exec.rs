// logical instructions will live here 
// there should be a struct for the machine here that includes the memory and registers

// add
// mul
// div
// nand

// return will be something like self.registers.set(inst, value)

pub use crate::memory_handler::Memory;
pub use crate::memory_loader::Instruction;

// structure to represent the machine (with memory emulation and registers)
struct Machine {
    // memory is a defined Memory struct
    memory: Memory,

    // registers are eight u32 values, stores in an array
    registers: [u32; 8]
}

impl Machine {
    // initialize a new machine struct with a given set of instruction
    fn new(instructions: Vec<u32>) -> Machine {
        todo!()
    }

    // get a machine's given instruction at a particular program counter position
    fn get_instruction(&self, pc: u32) -> Instruction {
        todo!()
    }

    // opcode 1 - if $r[C] != 0 then $r[A] := $r[B]
    fn cmov(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 1 - $r[A] := $m[$r[B]][$r[C]]
    fn load(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 2 - $m[$r[A]][$r[B]] := $r[C]
    fn store(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 3 - $r[A] := ($r[B] + $r[C]) % 2^32
    fn add(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 4 - $r[A] := ($r[B] × $r[C]) % 2^32
    fn mul(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 5 - $r[A] := ($r[B] ÷ $r[C]) (integer division)
    fn div(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 6 - $r[A] :=¬($r[B]∧$r[C])
    fn nand(&mut self, instruction: Instruction) {
        todo!()
    }

    // DO WE NEEd THIS? or in 'main', we can call an exit instead?
    // opcode 7 - halt computation
    fn halt(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 8 - new segment is created with a number of words equal to the value in $r[C]
    fn map_segment(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 9 - segment $m[$r[C]] is unmapped
    fn unmap_segment(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 10 - value in $r[C] is displayed on the I/O
    // Only values from 0 to 255 are allowed
    fn output(&self, instruction: Instruction) {
        todo!()
    }

    // opcode 11 - $r[c] is loaded with I/O input 
    // Must be a value from 0 to 255
    fn input(&mut self, instruction: Instruction) {
        todo!()
    }

    // opcode 12 - $m[$r[B]] is duplicated, program counter is set to point to $m[0][$r[C]]
    fn load_program(&mut self, instruction: Instruction) -> usize {
        todo!()
    }

    // opcode 13 - $r[A] = least significant 25 bits in instruction
    fn load_value(&mut self, instruction: Instruction) {
        todo!()
    }
}

