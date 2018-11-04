use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::cpu::operation::Operation;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn add_register(&mut self, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        self.operate_on_register(
            Operation::Add,
            |cpu| &mut cpu.registers.af.0,
            operand,
            &[
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn add_carry_register(&mut self, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        let carry = Flag::Carry.get_bit(&self.cpu.state);
        self.operate_on_register(
            Operation::Add,
            |cpu| &mut cpu.registers.af.0,
            operand + carry,
            &[
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn subtract_register(&mut self, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        self.operate_on_register(
            Operation::Subtract,
            |cpu| &mut cpu.registers.af.0,
            operand,
            &[
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn subtract_carry_register(&mut self, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        let carry = Flag::Carry.get_bit(&self.cpu.state);
        self.operate_on_register(
            Operation::Subtract,
            |cpu| &mut cpu.registers.af.0,
            operand + carry,
            &[
                Flag::AddSubtract,
                Flag::Carry,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::Sign,
                Flag::Zero,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn increment_register(&mut self, target: fn(&mut State) -> &mut u8) {
        self.operate_on_register(
            Operation::Add,
            target,
            1,
            &[
                Flag::AddSubtract,
                Flag::ParityOverflow,
                Flag::HalfCarry,
                Flag::Zero,
                Flag::Sign,
            ],
        );
        self.clock(4);
    }

    pub(crate) fn decrement_register(&mut self, target: fn(&mut State) -> &mut u8) {
        self.operate_on_register(
            Operation::Subtract,
            target,
            1,
            &[
                Flag::AddSubtract,
                Flag::ParityOverflow,
                Flag::HalfCarry,
                Flag::Zero,
                Flag::Sign,
            ],
        );
        self.clock(4);
    }

    fn operate_on_register(
        &mut self,
        operation: Operation,
        target: fn(&mut State) -> &mut u8,
        operand: u8,
        affected_flags: &[Flag],
    ) {
        let op1 = self.get_register_mut(target);
        let op2 = operation.maybe_negate(operand);
        let result = alu::add_octets(op1, op2);
        *target(&mut self.cpu.state) = result.value;
        Flag::set_values(
            &mut self.cpu.state,
            affected_flags,
            &[
                (Flag::Zero, result.value == 0x00),
                (Flag::Sign, result.value > 0x7F),
                (Flag::HalfCarry, result.half_carry),
                (Flag::ParityOverflow, result.overflow),
                (Flag::AddSubtract, operation == Operation::Subtract),
                (Flag::Carry, result.carry),
            ],
        );
    }
}
