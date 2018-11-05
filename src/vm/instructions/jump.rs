use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn jump(&mut self, condition: fn(&State) -> bool) {
        let dest = self.next_word();
        if condition(&self.cpu.state) {
            self.cpu.goto(dest);
        }
        self.clock(10);
    }
}
