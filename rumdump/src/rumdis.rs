
type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Opcode {
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
    LoadValue
}


static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};


fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

pub fn op(instruction: Umi) -> u32{
    (instruction >> OP.lsb) & mask(OP.width)
}

//get(&RC, inst) in the format 
pub fn disassemble(inst: Umi) -> String { 
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => { 
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        },

        o if o == Opcode::Load as u32 => { 
            format!("r{} := m[r{}][r{}];", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Store as u32 => { 
            format!("m[r{}][r{}] := r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Add as u32 => { 
            format!("r{} := (r{} + r{}) % 2^32;", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Mul as u32 => { 
            format!("r{} := (r{} * r{}) % 2^32;", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Div as u32 => { 
            format!("r{} := (r{} / r{});", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Nand as u32 => { 
            format!("r{} := !(r{} ^ r{});", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },

        o if o == Opcode::Halt as u32 => { 
            format!("Halt!")
        },

        o if o == Opcode::MapSegment as u32 => { 
            format!("New segment with {} words mapped as r{};", get(&RC, inst), get(&RB, inst))
        },

        o if o == Opcode::UnmapSegment as u32 => { 
            format!("Unmap m[r{}];", get(&RC, inst))
        },

        o if o == Opcode::Output as u32 => { 
            format!("Output value of r[C]: {}", get(&RC, inst))
        },

        o if o == Opcode::Input as u32 => { 
            format!("r{} is loaded with input", get(&RC, inst))
        },

        o if o == Opcode::LoadProgram as u32 => { 
            format!("m[0] := m[r{}];", get(&RB, inst))
        },

        o if o == Opcode::LoadValue as u32 => { 
            format!("r{} := {};", get(&RL, inst), get(&VL, inst))
        },

        _ => {
            format!(".data 0x({:x})", inst)
        }
    }
}
