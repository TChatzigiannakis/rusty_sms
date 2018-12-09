use crate::vm::cpu::alu;
use crate::vm::cpu::state::State;

pub struct Processor {
    pub state: State,
    halted: bool,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            state: State::new(),
            halted: false,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }
    pub fn halt(&mut self) {
        self.halted = true;
    }
    pub fn unhalt(&mut self) {
        self.halted = false;
    }

    pub fn goto(&mut self, address: u16) {
        self.state.pc = alu::get_octets(address);
    }
}
