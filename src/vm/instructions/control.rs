use crate::vm::cpu::flags::Flag;
use crate::vm::cpu::state::State;
use crate::vm::machine::Machine;

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
            let state = &mut self.cpu.state;
            Flag::Carry.set(state, true);
            Flag::HalfCarry.set(state, false);
            Flag::AddSubtract.set(state, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_carry_flag(&mut self) {
        {
            let previous = Flag::Carry.get(&self.cpu.state);
            let state = &mut self.cpu.state;
            Flag::Carry.set(state, !previous);
            Flag::HalfCarry.set(state, previous);
            Flag::AddSubtract.set(state, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_registers(&mut self, selector: fn(&mut State) -> &mut u8) {
        *selector(&mut self.cpu.state) = *selector(&mut self.cpu.state);
        {
            let state = &mut self.cpu.state;
            Flag::AddSubtract.set(state, true);
            Flag::HalfCarry.set(state, true);
        }
        self.clock(4);
    }
}
