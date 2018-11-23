#[cfg(test)]
mod tests {
    use element::Element::Instruction;
    use element::Element::Parameter;
    use program::Program;
    use vm::callbacks::Callbacks;
    use vm::cpu::alu;
    use vm::cpu::flags::Flag;
    use vm::cpu::registers::Registers;
    use vm::instructions::opcodes::Opcode;
    use vm::machine::Machine;

    fn new_vm(regs: fn(&mut Registers), stream: Vec<Opcode>, start: u16) -> Machine {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_vector(stream.iter().map(|i| *i as u8).collect());
        vm.load_at(&p, start);
        regs(&mut vm.cpu.state.registers);
        vm.cpu.goto(start);
        vm
    }

    #[test]
    fn stop() {
        let mut vm = new_vm(|_| {}, vec![], 0);
        let mut callbacks = Callbacks::new();
        callbacks.on_before_instruction_exec_match(Opcode::Nop, Box::new(|m| m.stop()));
        vm.start_with_options(0, &mut callbacks);
    }

    #[test]
    fn nibbles() {
        for iteration in 0..256 {
            let i = iteration as u8;
            let r = alu::add_octets(i, 1);
            assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
            assert_eq!(i == 0xFF, r.carry, "At {}.", i);
            assert_eq!((i & 0x0F) == 0x0F, r.half_carry, "At {}.", i);
            assert_eq!(i == 0x7F, r.overflow, "At {}.", i);
        }
    }

    #[test]
    fn words() {
        for iteration in 0..65536 {
            let i = iteration as u16;
            let r = alu::add_words(i, 1);
            assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
            assert_eq!(i == 0xFFFF, r.carry, "At {}.", i);
            assert_eq!((i & 0x0FFF) == 0x0FFF, r.half_carry, "At {}.", i);
            assert_eq!(i == 0x7FFF, r.overflow, "At {}.", i);
        }
    }

    #[test]
    fn increment() {
        let mut vm = new_vm(|_| {}, (0..256).map(|_| Opcode::IncA).collect(), 0);
        let mut callbacks = Callbacks::new();
        let mut i = 0;
        callbacks.on_before_instruction_exec_match(
            Opcode::IncA,
            Box::new(move |machine| {
                let a = machine.get_register(Registers::a());
                let h = Flag::HalfCarry.get(&machine.cpu.state);
                let s = Flag::Sign.get(&machine.cpu.state);
                let ov = Flag::ParityOverflow.get(&machine.cpu.state);
                assert_eq!(i, a);
                assert_eq!(i >= 0x80, s, "At value {}.", i);
                assert_eq!(i == 0x80, ov, "At value {}.", i);
                if i > 0 {
                    assert_eq!(i & 0x0F == 0, h, "At value {}.", i);
                }
                i = i.wrapping_add(1);
            }),
        );
        callbacks.on_before_instruction_exec_match(Opcode::Nop, Box::new(|m| m.stop()));
        vm.start_with_options(0, &mut callbacks);
    }

    #[test]
    fn increment_pair() {
        let mut vm = new_vm(|_| {}, (0..65536).map(|_| Opcode::IncBC).collect(), 0);
        let mut callbacks = Callbacks::new();
        let mut i = 0;
        callbacks.on_before_instruction_exec_match(
            Opcode::IncBC,
            Box::new(move |machine| {
                let bc = machine.get_register_pair(|cpu| cpu.registers.bc);
                assert_eq!(i, bc);
                i = i.wrapping_add(1);
            }),
        );
        callbacks.on_before_instruction_exec_match(Opcode::Nop, Box::new(|m| m.stop()));
        vm.start_with_options(0, &mut callbacks);
    }

    #[test]
    fn add() {
        let mut vm = new_vm(
            |regs| {
                regs.bc.0 = 0x01;
            },
            (0..256).map(|_| Opcode::AddB).collect(),
            0,
        );
        let mut callbacks = Callbacks::new();
        let mut i = 0;
        callbacks.on_before_instruction_exec_match(
            Opcode::AddB,
            Box::new(move |machine| {
                let a = machine.get_register(Registers::a());
                let b = machine.get_register(Registers::b());
                let h = Flag::HalfCarry.get(&machine.cpu.state);
                let s = Flag::Sign.get(&machine.cpu.state);
                let ov = Flag::ParityOverflow.get(&machine.cpu.state);
                assert_eq!(i, a);
                assert_eq!(i >= 0x80, s, "At value {}.", i);
                assert_eq!(i == 0x80, ov, "At value {}.", i);
                if i > 0 {
                    assert_eq!(i & 0x0F == 0, h, "At value {}.", i);
                }
                i = i.wrapping_add(b);
            }),
        );
        callbacks.on_before_instruction_exec_match(Opcode::Nop, Box::new(|m| m.stop()));
        vm.start_with_options(0, &mut callbacks);
    }

    fn jump_test_flag(varient: Opcode, target: u16, flag: Flag, value: bool, expected: u16) {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_param_word(varient, target);
        vm.load_at(&p, 0);
        flag.set(&mut vm.cpu.state, value);

        let mut callbacks = Callbacks::new();
        callbacks.on_after_instruction_exec(Box::new(|m, _| m.stop()));
        vm.start_with_options(0, &mut callbacks);

        let pc = vm.get_register_pair(|cpu| cpu.pc);
        assert_eq!(expected, pc);
    }

    fn jump_test_dual(variant: Opcode, flag: Flag, value: bool) {
        jump_test_flag(variant, 0x10, flag, value, 0x10);
        jump_test_flag(variant, 0x10, flag, !value, 0x03);
    }

    #[test]
    fn jump() {
        jump_test_flag(Opcode::JpXX, 0x10, Flag::Unused1, false, 0x10);
        jump_test_dual(Opcode::JpNZXX, Flag::Zero, false);
        jump_test_dual(Opcode::JpZXX, Flag::Zero, true);
        jump_test_dual(Opcode::JpNCXX, Flag::Carry, false);
        jump_test_dual(Opcode::JpCXX, Flag::Carry, true);
        jump_test_dual(Opcode::JpPOXX, Flag::ParityOverflow, true);
        jump_test_dual(Opcode::JpPEXX, Flag::ParityOverflow, false);
        jump_test_dual(Opcode::JpPXX, Flag::Sign, false);
        jump_test_dual(Opcode::JpMXX, Flag::Sign, true);
    }

    #[test]
    fn load() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add(Instruction(Opcode::LdBC));
        vm.load(&p);

        vm.cpu.state.registers.bc = (0x00, 0x20);

        vm.start();

        assert_eq!(vm.cpu.state.registers.bc, (0x20, 0x20));
    }

    #[test]
    fn load_param() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add(Instruction(Opcode::LdBX));
        p.add(Parameter(0x42));
        vm.load(&p);

        vm.cpu.state.registers.bc = (0x00, 0x00);

        vm.start();

        assert_eq!(vm.cpu.state.registers.bc, (0x42, 0x00));
    }
}
