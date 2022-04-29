use rumasm::rumasm::asm;
use rumasm::rumasm::halt;

pub fn main() { 
    asm(halt());
}