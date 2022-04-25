// handles the reading in and loading of the instructions
// the first task will be to check if we're in bounds
// the next task will store the instructions in a list (likely a vec)
// the last task will be loading the instructions into memory (as long as they're valid)

pub use rumdump::rumdis::Opcode;
pub use bitpack::getu; // used to extract bits from instruction

// struct to hold all possible information in an instruction
pub struct Instruction {
    // 4 most significant bits representing operation type
    op: Opcode,

    // at least 1 register (RA) and up to 3 registers (RB, RC)
    a: u32,
    b: Option<u32>,
    c: Option<u32>,

    // value to be stored in RA, if opcode 13
    value: Option<u32>
}

impl Instruction {
    // create a new instruction struct given u32 representation
    fn new(instruction: u32) -> Instruction {
        todo!()
    }

    // helper functions? to extract info from instruction
}