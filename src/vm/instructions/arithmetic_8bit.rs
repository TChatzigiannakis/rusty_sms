use crate::vm::cpu::alu;
use crate::vm::cpu::flags::Flag;
use crate::vm::cpu::operation::Operation;
use crate::vm::cpu::registers::Registers;
use crate::vm::cpu::state::State;
use crate::vm::machine::Machine;

impl Machine {
    pub(crate) fn add_register(&mut self, selector: fn(&State) -> u8) {
        self.op_register(Operation::Add, selector);
    }

    pub(crate) fn add_carry_register(&mut self, selector: fn(&State) -> u8) {
        self.op_carry_register(Operation::Add, selector);
    }

    pub(crate) fn add_carry_memory(&mut self) {
        let address = self.get_register_pair(Registers::hl());
        let operand = self.ram.read_u8(address);
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
        self.clock(7);
    }

    pub(crate) fn subtract_register(&mut self, selector: fn(&State) -> u8) {
        self.op_register(Operation::Subtract, selector);
    }

    pub(crate) fn subtract_carry_register(&mut self, selector: fn(&State) -> u8) {
        self.op_carry_register(Operation::Subtract, selector);
    }

    fn op_register(&mut self, operation: Operation, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        self.operate_on_register(
            operation,
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

    fn op_carry_register(&mut self, operation: Operation, selector: fn(&State) -> u8) {
        let operand = self.get_register(selector);
        let carry = Flag::Carry.get_bit(&self.cpu.state);
        self.operate_on_register(
            operation,
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
        self.op_register_by_1(target, Operation::Add);
    }

    pub(crate) fn decrement_register(&mut self, target: fn(&mut State) -> &mut u8) {
        self.op_register_by_1(target, Operation::Subtract);
    }

    fn op_register_by_1(&mut self, target: fn(&mut State) -> &mut u8, operation: Operation) {
        self.operate_on_register(
            operation,
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

    pub(crate) fn increment_memory(&mut self) {
        self.op_memory_by_1(Operation::Add);
    }

    pub(crate) fn decrement_memory(&mut self) {
        self.op_memory_by_1(Operation::Subtract)
    }

    fn op_memory_by_1(&mut self, operation: Operation) {
        self.operate_on_memory(
            operation,
            1,
            &[
                Flag::Sign,
                Flag::Zero,
                Flag::HalfCarry,
                Flag::ParityOverflow,
                Flag::AddSubtract,
            ],
        );
    }

    pub(crate) fn add_memory(&mut self) {
        self.op_accumulator_memory(Operation::Add);
    }

    pub(crate) fn sub_memory(&mut self) {
        self.op_accumulator_memory(Operation::Subtract);
    }

    fn op_accumulator_memory(&mut self, operation: Operation) {
        let address = self.get_register_pair(|state| state.registers.hl);
        let operand = self.ram.read_u8(address);
        self.operate_on_register(
            operation,
            |state| &mut state.registers.af.0,
            operand,
            &[
                Flag::Sign,
                Flag::Zero,
                Flag::Carry,
                Flag::ParityOverflow,
                Flag::AddSubtract,
                Flag::Carry,
            ],
        );
        self.clock(7);
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

    fn operate_on_memory(&mut self, operation: Operation, operand: u8, affected_flags: &[Flag]) {
        let address = self.get_register_pair(|state| state.registers.hl);
        let op1 = self.ram.read_u8(address);
        let op2 = operation.maybe_negate(operand);
        let result = alu::add_octets(op1, op2);
        self.ram.write_u8(address, result.value);
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
