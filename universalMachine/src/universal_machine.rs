// this is our main
// initalizing all registers to 0 should happen here 

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