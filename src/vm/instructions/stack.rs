use crate::vm::cpu::alu;
use crate::vm::cpu::state::State;
use crate::vm::machine::Machine;

impl Machine {
    pub(crate) fn push_to_stack(&mut self, selector: fn(&State) -> (u8, u8)) {
        let value = alu::get_word(selector(&self.cpu.state));
        let sp = alu::get_word(self.cpu.state.sp);
        self.ram.write_u16(sp, value);
        self.cpu.state.sp = alu::get_octets(sp - 2);
        self.clock(11);
    }

    pub(crate) fn push_program_counter_to_stack(&mut self) {
        let pc = alu::get_word(self.cpu.state.pc);
        let sp = alu::get_word(self.cpu.state.sp);
        self.ram.write_u16(sp, pc);
        self.cpu.state.sp = alu::get_octets(sp - 2);
    }

    pub(crate) fn pop_from_stack(&mut self, selector: fn(&mut State) -> &mut (u8, u8)) {
        let sp = alu::get_word(self.cpu.state.sp);
        {
            let (high_reg, low_reg) = selector(&mut self.cpu.state);
            let value = alu::get_octets(self.ram.read_u16(sp));
            *high_reg = value.0;
            *low_reg = value.1;
        }
        self.cpu.state.sp = alu::get_octets(sp + 2);
        self.clock(10);
    }

    pub(crate) fn pop_stack_to_program_counter(&mut self) {
        let sp = alu::get_word(self.cpu.state.sp);
        let value = self.ram.read_u16(sp);
        self.cpu.state.pc = alu::get_octets(value);
        self.cpu.state.sp = alu::get_octets(sp + 2);
    }
}
