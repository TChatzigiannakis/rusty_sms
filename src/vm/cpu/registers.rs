use crate::vm::cpu::alu;
use crate::vm::{
    AddressSelector, DoubleRegisterSelector, Register, RegisterSelector,
    TargetDoubleRegisterSelector, TargetRegisterSelector,
};

pub struct Registers {
    pub af: (Register, Register),
    pub bc: (Register, Register),
    pub de: (Register, Register),
    pub hl: (Register, Register),
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: (0x00, 0x00),
            bc: (0x00, 0x00),
            de: (0x00, 0x00),
            hl: (0x00, 0x00),
        }
    }

    pub fn af() -> DoubleRegisterSelector {
        |cpu| cpu.registers.af
    }

    pub fn bc() -> DoubleRegisterSelector {
        |cpu| cpu.registers.bc
    }

    pub fn de() -> DoubleRegisterSelector {
        |cpu| cpu.registers.de
    }

    pub fn hl() -> DoubleRegisterSelector {
        |cpu| cpu.registers.hl
    }

    pub fn sp() -> DoubleRegisterSelector {
        |cpu| cpu.sp
    }

    pub fn a() -> RegisterSelector {
        |cpu| cpu.registers.af.0
    }

    pub fn f() -> RegisterSelector {
        |cpu| cpu.registers.af.1
    }

    pub fn b() -> RegisterSelector {
        |cpu| cpu.registers.bc.0
    }

    pub fn c() -> RegisterSelector {
        |cpu| cpu.registers.bc.1
    }

    pub fn d() -> RegisterSelector {
        |cpu| cpu.registers.de.0
    }

    pub fn e() -> RegisterSelector {
        |cpu| cpu.registers.de.1
    }

    pub fn h() -> RegisterSelector {
        |cpu| cpu.registers.hl.0
    }

    pub fn l() -> RegisterSelector {
        |cpu| cpu.registers.hl.1
    }

    pub fn address_in_bc() -> AddressSelector {
        |cpu| alu::get_word(cpu.registers.bc)
    }

    pub fn address_in_de() -> AddressSelector {
        |cpu| alu::get_word(cpu.registers.de)
    }

    pub fn address_in_hl() -> AddressSelector {
        |cpu| alu::get_word(cpu.registers.hl)
    }

    pub fn into_af() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.af
    }

    pub fn into_bc() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.bc
    }

    pub fn into_de() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.de
    }

    pub fn into_hl() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.hl
    }

    pub fn into_sp() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.sp
    }

    pub fn into_a() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.af.0
    }

    pub fn into_f() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.af.1
    }

    pub fn into_b() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.bc.0
    }

    pub fn into_c() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.bc.1
    }

    pub fn into_d() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.de.0
    }

    pub fn into_e() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.de.1
    }

    pub fn into_h() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.hl.0
    }

    pub fn into_l() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.hl.1
    }
}
