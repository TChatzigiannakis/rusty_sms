use crate::vm::cpu::alu;
use crate::vm::cpu::flags::Flag;
use crate::vm::machine::Machine;

impl Machine {
    pub(crate) fn rotate_accumulator_copy_left(&mut self) {
        let old_value = self.cpu.state.registers.af.0 as u16;
        let carry = alu::get_bit::<u16>(old_value & 0x40 != 0);
        let new_value = (old_value << 1) | carry;
        self.cpu.state.registers.af.0 = new_value as u8;
        {
            let state = &mut self.cpu.state;
            Flag::Carry.set(state, carry == 1);
            Flag::HalfCarry.set(state, false);
            Flag::AddSubtract.set(state, false);
        }
        self.clock(4);
    }

    pub(crate) fn rotate_accumulator_left(&mut self) {
        let old_value = {
            let a = self.cpu.state.registers.af.0 as u16;
            let carry = Flag::Carry.get_bit(&mut self.cpu.state) as u16;
            carry << 8 | a
        };

        let shifted = old_value << 1;
        let extra = if shifted & 0x0200 != 0 { 1 } else { 0 };
        let carry = shifted & 0x0100 != 0;
        let a = (shifted as u8) | extra;
        self.cpu.state.registers.af.0 = a;
        Flag::Carry.set(&mut self.cpu.state, carry);
    }

    pub(crate) fn rotate_accumulator_copy_right(&mut self) {
        let old_value = self.cpu.state.registers.af.0 as u16;
        let carry = alu::get_bit::<u16>(old_value & 0x01 != 0);
        let new_value = (carry << 7) | (old_value >> 1);
        self.cpu.state.registers.af.0 = new_value as u8;
        {
            let state = &mut self.cpu.state;
            Flag::Carry.set(state, carry != 0);
            Flag::HalfCarry.set(state, false);
            Flag::AddSubtract.set(state, false);
        }
        self.clock(4);
    }

    pub(crate) fn rotate_accumulator_right(&mut self) {
        let old_value = {
            let a = self.cpu.state.registers.af.0 as u16;
            let carry = Flag::Carry.get_bit(&mut self.cpu.state) as u16;
            a << 8 | carry << 7
        };

        let shifted = old_value >> 1;
        let extra = if shifted & 0x0050 != 0 { 1 } else { 0 } as u16;
        let carry = shifted & 0x0070 != 0;
        let a = ((shifted | extra << 15) >> 8) as u8;
        self.cpu.state.registers.af.0 = a;
        Flag::Carry.set(&mut self.cpu.state, carry);
    }
}
