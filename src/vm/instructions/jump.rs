use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn ret(&mut self) {
        self.pop_stack_to_program_counter();
        self.clock(10);
    }

    // Need to separate conditional ret because of clock counts
    pub(crate) fn ret_conditional(&mut self, condition: fn(&State) -> bool) {
        if condition(&self.cpu.state) {
            self.pop_stack_to_program_counter();
            self.clock(11);
        } else {
            self.clock(5);
        }
    }
}
