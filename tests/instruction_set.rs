extern crate rusty_sms;

use rusty_sms::element::Element::Instruction;
use rusty_sms::element::Element::Parameter;
use rusty_sms::program::Program;
use rusty_sms::vm::callbacks::Callbacks;
use rusty_sms::vm::cpu::flags::Flag;
use rusty_sms::vm::cpu::registers::Registers;
use rusty_sms::vm::instructions::mnemonics::Mnemonic;
use rusty_sms::vm::machine::Machine;

fn new_vm(regs: fn(&mut Registers), stream: Vec<Mnemonic>, start: u16) -> Machine {
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
    callbacks.on_before_instruction_exec_match(Mnemonic::Nop, Box::new(|m| m.stop()));
    vm.start_with_options(0, &mut callbacks);
}

#[test]
fn increment() {
    let mut vm = new_vm(|_| {}, (0..256).map(|_| Mnemonic::IncA).collect(), 0);
    let mut callbacks = Callbacks::new();
    let mut i = 0;
    callbacks.on_before_instruction_exec_match(
        Mnemonic::IncA,
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
    callbacks.on_before_instruction_exec_match(Mnemonic::Nop, Box::new(|m| m.stop()));
    vm.start_with_options(0, &mut callbacks);
}

#[test]
fn increment_pair() {
    let mut vm = new_vm(|_| {}, (0..65536).map(|_| Mnemonic::IncBC).collect(), 0);
    let mut callbacks = Callbacks::new();
    let mut i = 0;
    callbacks.on_before_instruction_exec_match(
        Mnemonic::IncBC,
        Box::new(move |machine| {
            let bc = machine.get_register_pair(|cpu| cpu.registers.bc);
            assert_eq!(i, bc);
            i = i.wrapping_add(1);
        }),
    );
    callbacks.on_before_instruction_exec_match(Mnemonic::Nop, Box::new(|m| m.stop()));
    vm.start_with_options(0, &mut callbacks);
}

#[test]
fn add() {
    let mut vm = new_vm(
        |regs| {
            regs.bc.0 = 0x01;
        },
        (0..256).map(|_| Mnemonic::AddB).collect(),
        0,
    );
    let mut callbacks = Callbacks::new();
    let mut i = 0;
    callbacks.on_before_instruction_exec_match(
        Mnemonic::AddB,
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
    callbacks.on_before_instruction_exec_match(Mnemonic::Nop, Box::new(|m| m.stop()));
    vm.start_with_options(0, &mut callbacks);
}

fn jump_test_flag(varient: Mnemonic, target: u16, flag: Flag, value: bool, expected: u16) {
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

fn jump_test_dual(variant: Mnemonic, flag: Flag, value: bool) {
    jump_test_flag(variant, 0x10, flag, value, 0x10);
    jump_test_flag(variant, 0x10, flag, !value, 0x03);
}

#[test]
fn jump() {
    jump_test_flag(Mnemonic::JpXX, 0x10, Flag::Unused1, false, 0x10);
    jump_test_dual(Mnemonic::JpNZXX, Flag::Zero, false);
    jump_test_dual(Mnemonic::JpZXX, Flag::Zero, true);
    jump_test_dual(Mnemonic::JpNCXX, Flag::Carry, false);
    jump_test_dual(Mnemonic::JpCXX, Flag::Carry, true);
    jump_test_dual(Mnemonic::JpPOXX, Flag::ParityOverflow, true);
    jump_test_dual(Mnemonic::JpPEXX, Flag::ParityOverflow, false);
    jump_test_dual(Mnemonic::JpPXX, Flag::Sign, false);
    jump_test_dual(Mnemonic::JpMXX, Flag::Sign, true);
}

#[test]
fn load() {
    let mut vm = Machine::new();
    let mut p = Program::new();
    p.add(Instruction(Mnemonic::LdBC));
    vm.load(&p);

    vm.cpu.state.registers.bc = (0x00, 0x20);

    vm.start();
    assert_eq!(vm.cpu.state.registers.bc, (0x20, 0x20));
}

#[test]
fn load_param() {
    let mut vm = Machine::new();
    let mut p = Program::new();
    p.add(Instruction(Mnemonic::LdBX));
    p.add(Parameter(0x42));
    vm.load(&p);

    vm.cpu.state.registers.bc = (0x00, 0x00);

    vm.start();

    assert_eq!(vm.cpu.state.registers.bc, (0x42, 0x00));
}
