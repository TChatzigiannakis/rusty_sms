use vm::cpu::flags::Flag;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn and_register(&mut self, selector: fn(&State) -> u8) {
        self.bitwise_with_register(selector, |a, b| a & b, true);
    }

    pub(crate) fn or_register(&mut self, selector: fn(&State) -> u8) {
        self.bitwise_with_register(selector, |a, b| a | b, false);
    }

    pub(crate) fn xor_register(&mut self, selector: fn(&State) -> u8) {
        self.bitwise_with_register(selector, |a, b| a ^ b, false);
    }

    pub(crate) fn and_value(&mut self) {
        self.bitwise_with_value(|a, b| a & b, true);
    }

    pub(crate) fn or_value(&mut self) {
        self.bitwise_with_value(|a, b| a | b, false);
    }

    pub(crate) fn xor_value(&mut self) {
        self.bitwise_with_value(|a, b| a ^ b, false);
    }

    fn bitwise_with_register(
        &mut self,
        selector: fn(&State) -> u8,
        operation: fn(u8, u8) -> u8,
        half_carry_value: bool,
    ) {
        let operand = selector(&self.cpu.state);
        self.bitwise_operation(operand, operation, half_carry_value);
        self.clock(4);
    }

    fn bitwise_with_value(&mut self, operation: fn(u8, u8) -> u8, half_carry_value: bool) {
        let operand = self.next_byte();
        self.bitwise_operation(operand, operation, half_carry_value);
        self.clock(7);
    }

    fn bitwise_operation(
        &mut self,
        operand: u8,
        operation: fn(u8, u8) -> u8,
        half_carry_value: bool,
    ) {
        let op1 = self.cpu.state.registers.af.0;
        let op2 = operand;
        let result = operation(op1, op2);
        let parity = (0..8).fold(0, |acc, b| acc + (result >> b) & 1) % 2 == 0;

        let state = &mut self.cpu.state;
        Flag::ParityOverflow.set(state, parity);
        Flag::Carry.set(state, false);
        Flag::HalfCarry.set(state, half_carry_value);
        Flag::AddSubtract.set(state, false);
        Flag::Zero.set(state, result == 0x00);
        Flag::Sign.set(state, result > 0x7F);
    }
}
