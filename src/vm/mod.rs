use vm::cpu::state::State;

pub mod cpu;
pub mod instructions;
pub mod machine;
pub mod ram;

pub type Register = u8;
pub type DoubleRegister = (u8, u8);
pub type Address = u16;

pub type RegisterSelector = fn(&State) -> Register;
pub type DoubleRegisterSelector = fn(&State) -> DoubleRegister;

pub type TargetRegisterSelector = fn(&mut State) -> &mut Register;
pub type TargetDoubleRegisterSelector = fn(&mut State) -> &mut DoubleRegister;

pub type AddressSelector = fn(&State) -> Address;
