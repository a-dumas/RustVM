use std::env;
use rumdump::rumdis;
use rumdump::rumload;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    println!("{} instructions", instructions.len());
    
    for instruction in instructions {
        println!("{}", rumdis::disassemble(instruction))
    }
}
