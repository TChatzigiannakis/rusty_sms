use vm::cpu::flags::Flag;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn nop(&mut self) {
        self.clock(4);
    }

    pub(crate) fn halt(&mut self) {
        self.cpu.halt();
        self.clock(4);
    }

    pub(crate) fn set_carry_flag(&mut self) {
        {
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, true);
            Flag::HalfCarry.set(status, false);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_carry_flag(&mut self) {
        {
            let previous = Flag::Carry.get(&self.cpu.state.status);
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, !previous);
            Flag::HalfCarry.set(status, previous);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_registers(&mut self, selector: fn(&mut Registers) -> &mut u8) {
        *selector(&mut self.cpu.state.registers) = *selector(&mut self.cpu.state.registers);
        Flag::AddSubtract.set(&mut self.cpu.state.status, true);
        Flag::HalfCarry.set(&mut self.cpu.state.status, true);
        self.clock(4);
    }
}
