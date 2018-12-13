use crate::element::Element::Instruction;
use crate::element::Element::Parameter;
use crate::vm::instructions::mnemonics::Mnemonic;

pub enum Element {
    Instruction(Mnemonic),
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
