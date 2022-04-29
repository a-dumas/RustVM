// handles the reading in and loading of the instructions
// the first task will be to check if we're in bounds
// the next task will store the instructions in a list (likely a vec)
// the last task will be loading the instructions into memory (as long as they're valid)

// pub use rumdump::rumdis::Opcode;
use bitpack::getu; // used to extract bits from instruction

// pub enum for Opcodes and operation type
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Err
}

// helper function to extract opcode from instruction
fn get_op(instruction: u32) -> Opcode{
    let opcode = getu(instruction as u64, 4, 28) as u32;
    // eprintln!("OPCODE {}", opcode);
    match opcode {
        0 => Opcode::CMov,
        1 => Opcode::Load,
        2 => Opcode::Store,
        3 => Opcode::Add,
        4 => Opcode::Mul,
        5 => Opcode::Div,
        6 => Opcode::Nand,
        7 => Opcode::Halt,
        8 => Opcode::MapSegment,
        9 => Opcode::UnmapSegment,
        10 => Opcode::Output,
        11 => Opcode::Input,
        12 => Opcode::LoadProgram,
        13 => Opcode::LoadValue,
        _ => Opcode::Err
    }
}

// struct to hold all possible information in an instruction
#[derive(Debug)]
pub struct Instruction {
    // 4 most significant bits representing operation type
    pub op: Opcode,

    // at least 1 register (RA) and up to 3 registers (RB, RC)
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,

    // value to be stored in RA, if opcode 13
    pub value: Option<u32>
}

impl Instruction {
    // create a new instruction struct given u32 representation
    pub fn new(instruction: u32) -> Instruction {
        let op = get_op(instruction);

        // special opcode - load value
        let (a, b, c, value) = 
            if op == Opcode::LoadValue {
                (getu(instruction as u64, 3, 25) as u32, 
                None,
                None,
                Some(getu(instruction as u64, 25, 0) as u32))
            } else {
                (getu(instruction as u64, 3, 6) as u32,
                Some(getu(instruction as u64, 3, 3) as u32),
                Some(getu(instruction as u64, 3, 0) as u32),
                None)
            };
        
        // let b = Some(getu(instruction as u64, 3, 3) as u32);
        // let c = Some(getu(instruction as u64, 3, 0) as u32);

        Instruction {
            op,
            a,
            b,
            c,
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_op, Opcode};

    #[test]
    fn test_extract_opcode_0s() {
        let word: u32 = 0;
        let opcode = get_op(word);
        assert_eq!(Opcode::CMov, opcode);
    }

    #[test]
    fn test_extract_opcode_1s() {
        let word: u32 = 4294967295;
        let opcode = get_op(word);
        assert_eq!(Opcode::Err, opcode);
    }

    #[test]
    fn test_extract_bits_13() {
        let word: u32 = 3722304989;
        let opcode = get_op(word);
        assert_eq!(Opcode::LoadValue, opcode);
    }

    #[test]
    fn test_extract_bits_9() {
        let word: u32 = 2576980377;
        let opcode = get_op(word);
        assert_eq!(Opcode::UnmapSegment, opcode);
    }
}