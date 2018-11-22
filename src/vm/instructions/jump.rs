use vm::cpu::alu;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, condition: fn(&State) -> bool) {
        let destination = self.next_word();
        if condition(&self.cpu.state) {
            self.cpu.goto(destination);
        }
        self.clock(10);
    }

    pub(crate) fn jump_relative(&mut self, condition: fn(&State) -> bool) {
        if condition(&self.cpu.state) {
            let offset = self.next_byte();
            let pc = alu::get_word(self.cpu.state.pc) - 1;
            let (destination, _) = pc.overflowing_add(offset as u16);
            self.cpu.goto(destination);
            self.clock(12);
        } else {
            self.clock(7);
        }
    }

    pub(crate) fn decrement_and_jump_on_non_zero(&mut self) {
        let pc = alu::get_word(self.get_register(|s| s.pc)) - 1;
        let e = alu::sign_extend(self.next_byte());
        let b = self.get_register(|s| s.registers.bc.0);
        let (r, _) = b.overflowing_sub(1);
        self.set_register(|s| &mut s.registers.bc.0, r);
        if r != 0 {
            let target = alu::get_octets(alu::add_words(pc, e).value);
            self.set_register(|s| &mut s.pc, target);
            self.clock(3);
        } else {
            self.clock(2);
        }
    }
}
