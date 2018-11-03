use vm::cpu::alu;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, condition: fn(&u8) -> bool) {
        let dest = self.next_word();

        if condition(&self.cpu.state.status) {
            self.cpu.goto(dest);
        }

        self.clock(10);
    }

    pub(crate) fn call(&mut self, condition: fn(&u8) -> bool) {
        let dest = self.next_word();
        if condition(&self.cpu.state.status) {
            self.push_program_counter_to_stack();
            self.cpu.state.pc = alu::get_octets(dest);
            self.clock(17);
        } else {
            self.clock(10);
        }
    }
}
