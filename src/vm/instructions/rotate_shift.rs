use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn rotate_accumulator_left(&mut self) {
        let old_value = self.cpu.state.registers.a as u16;
        let carry = alu::get_bit::<u16>(old_value & 0x40 != 0);
        let new_value = (old_value << 1) | carry;
        self.cpu.state.registers.a = new_value as u8;
        {
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, carry == 1);
            Flag::HalfCarry.set(status, false);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }

    pub(crate) fn rotate_accumulator_right(&mut self) {
        let old_value = self.cpu.state.registers.a as u16;
        let carry = alu::get_bit::<u16>(old_value & 0x01 != 0);
        let new_value = (carry << 7) | (old_value >> 1);
        self.cpu.state.registers.a = new_value as u8;
        {
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, carry != 0);
            Flag::HalfCarry.set(status, false);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }
}
