use vm::cpu::state::State;

type Register = u8;
type DoubleRegister = (u8, u8);

type RegisterSelector = fn(&State) -> Register;
type DoubleRegisterSelector = fn(&State) -> DoubleRegister;

type TargetRegisterSelector = fn(&mut State) -> &mut Register;
type TargetDoubleRegisterSelector = fn(&mut State) -> &mut DoubleRegister;

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

    pub fn from_af() -> DoubleRegisterSelector {
        |cpu| cpu.registers.af
    }

    pub fn from_bc() -> DoubleRegisterSelector {
        |cpu| cpu.registers.bc
    }

    pub fn from_de() -> DoubleRegisterSelector {
        |cpu| cpu.registers.de
    }

    pub fn from_hl() -> DoubleRegisterSelector {
        |cpu| cpu.registers.hl
    }

    pub fn from_sp() -> DoubleRegisterSelector {
        |cpu| cpu.sp
    }

    pub fn from_a() -> RegisterSelector {
        |cpu| cpu.registers.af.0
    }

    pub fn from_f() -> RegisterSelector {
        |cpu| cpu.registers.af.1
    }

    pub fn from_b() -> RegisterSelector {
        |cpu| cpu.registers.bc.0
    }

    pub fn from_c() -> RegisterSelector {
        |cpu| cpu.registers.bc.1
    }

    pub fn from_d() -> RegisterSelector {
        |cpu| cpu.registers.de.0
    }

    pub fn from_e() -> RegisterSelector {
        |cpu| cpu.registers.de.1
    }

    pub fn from_h() -> RegisterSelector {
        |cpu| cpu.registers.hl.0
    }

    pub fn from_l() -> RegisterSelector {
        |cpu| cpu.registers.hl.1
    }

    pub fn to_af() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.af
    }

    pub fn to_bc() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.bc
    }

    pub fn to_de() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.de
    }

    pub fn to_hl() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.registers.hl
    }

    pub fn to_sp() -> TargetDoubleRegisterSelector {
        |cpu| &mut cpu.sp
    }

    pub fn to_a() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.af.0
    }

    pub fn to_f() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.af.1
    }

    pub fn to_b() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.bc.0
    }

    pub fn to_c() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.bc.1
    }

    pub fn to_d() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.de.0
    }

    pub fn to_e() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.de.1
    }

    pub fn to_h() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.hl.0
    }

    pub fn to_l() -> TargetRegisterSelector {
        |cpu| &mut cpu.registers.hl.1
    }
}
