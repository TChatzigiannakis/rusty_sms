use element::Element::Instruction;
use element::Element::Parameter;
use vm::instructions::opcodes::Opcode;

pub enum Element {
    Instruction(Opcode),
    Parameter(u8),
}

impl Element {
    pub fn get_u8(self) -> u8 {
        match self {
            Instruction(instr) => instr as u8,
            Parameter(value) => value,
        }
    }
}
