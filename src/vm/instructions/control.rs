use vm::machine::Machine;

impl Machine {
    pub(crate) fn nop(&mut self) {
        self.clock(4);
    }

    pub(crate) fn halt(&mut self) {
        self.cpu.halt();
        self.clock(4);
    }

    pub(crate) fn jump(&mut self, condition: fn(&u8) -> bool) {
        let dest = self.next_word();

        if condition(&self.cpu.state.status) {
            self.cpu.goto(dest);
        }

        self.clock(10);
    }
}
