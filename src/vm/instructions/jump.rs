use vm::cpu::alu;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, condition: fn(&State) -> bool) {
        if condition(&self.cpu.state) {
            let destination = self.next_word();
            self.cpu.goto(destination);
        }
        self.clock(10);
    }

    pub(crate) fn jump_relative(&mut self, condition: fn(&State) -> bool) {
        if condition(&self.cpu.state) {
            let offset = self.next_byte();
            let pc = alu::get_word(self.cpu.state.pc);
            let (destination, _) = pc.overflowing_add(offset as u16);
            self.cpu.goto(destination);
            self.clock(12);
        } else {
            self.clock(7);
        }
    }
}
