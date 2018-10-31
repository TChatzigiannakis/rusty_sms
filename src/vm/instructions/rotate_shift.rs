use vm::cpu::flags::Flag;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn rotate_accumulator_left(&mut self) {
        let old_value = self.cpu.state.registers.a as u16;
        let second_most_significant_bit = if old_value & 0x40 != 0 { 1 } else { 0 };
        let new_value = (old_value << 1) + second_most_significant_bit;
        self.cpu.state.registers.a = new_value as u8;
        Flag::Carry.set(&mut self.cpu.state.status, second_most_significant_bit == 1);
        Flag::HalfCarry.set(&mut self.cpu.state.status, false);
        Flag::AddSubtract.set(&mut self.cpu.state.status, false);
        self.clock(4);
    }
}
