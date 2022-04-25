// allocation and free fo here
// u32 for each identifier and Vec<u32> for the actual space

// structure to emulate a memory space
pub struct Memory {
    // stack of reusable memory addresses
    free_addresses: Vec<u32>,

    // heap of reusable memory locations
    heap: Vec<Vec<u32>>,

    // highest memory address currently in use or waiting to be reused
    max_id: u32
}

impl Memory {
    // allocate memory using a vector of a given size filled with 0s
    fn allocate(&mut self, size: u32) -> u32 {
        // return address of newly allocated memory 
        todo!()
    }

    // deallocate memory at a given memory address
    fn abandon(&mut self, address: u32) {
        todo!()
    }

    // return Some(value at a given address) if available, otherwise, return None.
    fn get(&self, address: u32) -> Option<&Vec<u32>> {
        todo!()
    }

    // write a value into a given address.
    fn set(&mut self, address: u32, value: u32) {
        todo!()
    }

    // load the program with the vector stored at the given address
    fn load(&mut self, address: usize) {
        todo!()
    }
}


