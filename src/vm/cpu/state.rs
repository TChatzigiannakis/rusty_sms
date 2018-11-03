use vm::cpu::alu;
use vm::cpu::registers::Registers;

pub struct State {
    pub registers: Registers,
    pub alt_registers: Registers,
    pub pc: (u8, u8),
    pub sp: (u8, u8),
    pub status: u8,
}

impl State {
    pub fn new() -> State {
        State {
            registers: Registers::new(),
            alt_registers: Registers::new(),
            pc: (0x00, 0x00),
            sp: (0x00, 0x00),
            status: 0,
        }
    }

    pub(crate) fn assign_bytes(
        &mut self,
        target: fn(&mut State) -> &mut (u8, u8),
        value: (u8, u8),
    ) {
        *target(self) = value;
    }

    pub(crate) fn assign_word(&mut self, target: fn(&mut State) -> &mut (u8, u8), value: u16) {
        self.assign_bytes(target, alu::get_octets(value));
    }

    pub(crate) fn get_word(&mut self, target: fn(&mut State) -> &mut (u8, u8)) -> u16 {
        alu::get_word(*target(self))
    }
}
