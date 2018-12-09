use crate::vm::cpu::alu;
use crate::vm::cpu::state::State;
use crate::vm::machine::Machine;

impl Machine {
    pub(crate) fn call(&mut self, condition: fn(&State) -> bool) {
        let dest = self.next_word();
        if condition(&self.cpu.state) {
            self.push_program_counter_to_stack();
            self.cpu.state.pc = alu::get_octets(dest);
            self.clock(17);
        } else {
            self.clock(10);
        }
    }

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
