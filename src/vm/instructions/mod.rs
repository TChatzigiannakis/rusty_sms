mod arithmetic_16bit;
mod arithmetic_8bit;
mod bits;
mod bitwise;
mod call_return;
mod control;
mod exchange;
mod jump;
mod load_16bit;
mod load_8bit;
pub mod mnemonics;
mod rotate_shift;
mod stack;

use crate::vm::callbacks::Callbacks;
use crate::vm::cpu::alu;
use crate::vm::cpu::flags::Flag;
use crate::vm::cpu::registers::Registers;
use crate::vm::instructions::mnemonics::Mnemonic;
use crate::vm::machine::Machine;

impl Machine {
    pub fn execute(&mut self) {
        self.execute_with(&mut Callbacks::new());
    }

    pub fn execute_with(&mut self, callbacks: &mut Callbacks) {
        callbacks.do_before_instruction_fetch(self);

        let instruction = Mnemonic::from(self.next_byte());
        callbacks.do_before_instruction_exec_match(self, instruction);
        callbacks.do_before_instruction_exec(self, instruction);

        match instruction {
            Mnemonic::BITS => self.execute_bits(),

            Mnemonic::Nop => self.nop(),
            Mnemonic::SCF => self.set_carry_flag(),
            Mnemonic::CCF => self.complement_carry_flag(),
            Mnemonic::CPL => self.complement_registers(Registers::into_a()),
            Mnemonic::Halt => self.halt(),

            Mnemonic::Exx => self.shadow_exchange_bc_de_hl(),
            Mnemonic::ExAFAF => self.shadow_exchange_af(),
            Mnemonic::ExDEHL => self.exhange_de_with_hl(),
            Mnemonic::ExVSPHL => self.exchage_memory_from_sp_with_hl(),

            Mnemonic::IncA => self.increment_register(Registers::into_a()),
            Mnemonic::IncB => self.increment_register(Registers::into_b()),
            Mnemonic::IncC => self.increment_register(Registers::into_c()),
            Mnemonic::IncD => self.increment_register(Registers::into_d()),
            Mnemonic::IncE => self.increment_register(Registers::into_e()),
            Mnemonic::IncH => self.increment_register(Registers::into_h()),
            Mnemonic::IncL => self.increment_register(Registers::into_l()),

            Mnemonic::DecA => self.decrement_register(Registers::into_a()),
            Mnemonic::DecB => self.decrement_register(Registers::into_b()),
            Mnemonic::DecC => self.decrement_register(Registers::into_c()),
            Mnemonic::DecD => self.decrement_register(Registers::into_d()),
            Mnemonic::DecE => self.decrement_register(Registers::into_e()),
            Mnemonic::DecH => self.decrement_register(Registers::into_h()),
            Mnemonic::DecL => self.decrement_register(Registers::into_l()),

            Mnemonic::IncBC => self.increment_register_pair(Registers::into_bc()),
            Mnemonic::IncDE => self.increment_register_pair(Registers::into_de()),
            Mnemonic::IncHL => self.increment_register_pair(Registers::into_hl()),
            Mnemonic::IncSP => self.increment_register_pair(Registers::into_sp()),
            Mnemonic::IncVHL => self.increment_memory(),

            Mnemonic::DecBC => self.decrement_register_pair(Registers::into_bc()),
            Mnemonic::DecDE => self.decrement_register_pair(Registers::into_de()),
            Mnemonic::DecHL => self.decrement_register_pair(Registers::into_hl()),
            Mnemonic::DecSP => self.decrement_register_pair(Registers::into_sp()),
            Mnemonic::DecVHL => self.decrement_memory(),

            Mnemonic::AddA => self.add_register(Registers::a()),
            Mnemonic::AddB => self.add_register(Registers::b()),
            Mnemonic::AddC => self.add_register(Registers::c()),
            Mnemonic::AddD => self.add_register(Registers::d()),
            Mnemonic::AddE => self.add_register(Registers::e()),
            Mnemonic::AddH => self.add_register(Registers::h()),
            Mnemonic::AddL => self.add_register(Registers::l()),
            Mnemonic::AddVHL => self.add_memory(),

            Mnemonic::SubA => self.subtract_register(Registers::a()),
            Mnemonic::SubB => self.subtract_register(Registers::b()),
            Mnemonic::SubC => self.subtract_register(Registers::c()),
            Mnemonic::SubD => self.subtract_register(Registers::d()),
            Mnemonic::SubE => self.subtract_register(Registers::e()),
            Mnemonic::SubH => self.subtract_register(Registers::h()),
            Mnemonic::SubL => self.subtract_register(Registers::l()),
            Mnemonic::SubVHL => self.sub_memory(),

            Mnemonic::AddHLBC => self.add_register_pair_to_hl(Registers::bc()),
            Mnemonic::AddHLDE => self.add_register_pair_to_hl(Registers::de()),
            Mnemonic::AddHLHL => self.add_register_pair_to_hl(Registers::hl()),
            Mnemonic::AddHLSP => self.add_register_pair_to_hl(Registers::sp()),

            Mnemonic::AdcA => self.add_carry_register(Registers::a()),
            Mnemonic::AdcB => self.add_carry_register(Registers::b()),
            Mnemonic::AdcC => self.add_carry_register(Registers::c()),
            Mnemonic::AdcD => self.add_carry_register(Registers::d()),
            Mnemonic::AdcE => self.add_carry_register(Registers::e()),
            Mnemonic::AdcH => self.add_carry_register(Registers::h()),
            Mnemonic::AdcL => self.add_carry_register(Registers::l()),
            Mnemonic::AdcAVHL => self.add_carry_memory(),

            Mnemonic::SbcA => self.subtract_carry_register(Registers::a()),
            Mnemonic::SbcB => self.subtract_carry_register(Registers::b()),
            Mnemonic::SbcC => self.subtract_carry_register(Registers::c()),
            Mnemonic::SbcD => self.subtract_carry_register(Registers::d()),
            Mnemonic::SbcE => self.subtract_carry_register(Registers::e()),
            Mnemonic::SbcH => self.subtract_carry_register(Registers::h()),
            Mnemonic::SbcL => self.subtract_carry_register(Registers::l()),

            Mnemonic::JpXX => self.jump(|_| true),
            Mnemonic::JpNZXX => self.jump(|status| !Flag::Zero.get(status)),
            Mnemonic::JpZXX => self.jump(|status| Flag::Zero.get(status)),
            Mnemonic::JpNCXX => self.jump(|status| !Flag::Carry.get(status)),
            Mnemonic::JpCXX => self.jump(|status| Flag::Carry.get(status)),
            Mnemonic::JpPOXX => self.jump(|status| Flag::ParityOverflow.get(status)),
            Mnemonic::JpPEXX => self.jump(|status| !Flag::ParityOverflow.get(status)),
            Mnemonic::JpPXX => self.jump(|status| !Flag::Sign.get(status)),
            Mnemonic::JpMXX => self.jump(|status| Flag::Sign.get(status)),

            Mnemonic::JrX => self.jump_relative(|_| true),
            Mnemonic::JrCX => self.jump_relative(|status| Flag::Carry.get(status)),
            Mnemonic::JrNCX => self.jump_relative(|status| !Flag::Carry.get(status)),
            Mnemonic::JrZX => self.jump_relative(|status| Flag::Zero.get(status)),
            Mnemonic::JrNZX => self.jump_relative(|status| !Flag::Zero.get(status)),

            Mnemonic::DjNZX => self.decrement_and_jump_on_non_zero(),

            Mnemonic::CallXX => self.call(|_| true),
            Mnemonic::CallNZXX => self.call(|status| !Flag::Zero.get(status)),
            Mnemonic::CallZXX => self.call(|status| Flag::Zero.get(status)),
            Mnemonic::CallNCXX => self.call(|status| !Flag::Carry.get(status)),
            Mnemonic::CallCXX => self.call(|status| Flag::Carry.get(status)),
            Mnemonic::CallPOXX => self.call(|status| Flag::ParityOverflow.get(status)),
            Mnemonic::CallPEXX => self.call(|status| !Flag::ParityOverflow.get(status)),
            Mnemonic::CallPXX => self.call(|status| !Flag::Sign.get(status)),
            Mnemonic::CallMXX => self.call(|status| Flag::Sign.get(status)),

            Mnemonic::Ret => self.ret(),
            Mnemonic::RetNZ => self.ret_conditional(|status| !Flag::Zero.get(status)),
            Mnemonic::RetZ => self.ret_conditional(|status| Flag::Zero.get(status)),
            Mnemonic::RetNC => self.ret_conditional(|status| !Flag::Carry.get(status)),
            Mnemonic::RetC => self.ret_conditional(|status| Flag::Carry.get(status)),
            Mnemonic::RetPO => self.ret_conditional(|status| Flag::ParityOverflow.get(status)),
            Mnemonic::RetPE => self.ret_conditional(|status| !Flag::ParityOverflow.get(status)),
            Mnemonic::RetP => self.ret_conditional(|status| !Flag::Sign.get(status)),
            Mnemonic::RetM => self.ret_conditional(|status| Flag::Sign.get(status)),

            Mnemonic::LdBCXX => self.load_into_register_pair(Registers::into_bc()),
            Mnemonic::LdDEXX => self.load_into_register_pair(Registers::into_de()),
            Mnemonic::LdHLXX => self.load_into_register_pair(Registers::into_hl()),
            Mnemonic::LdSPXX => self.load_into_register_pair(Registers::into_sp()),

            Mnemonic::LdVBCA => self.load_into_memory(Registers::a(), Registers::address_in_bc()),
            Mnemonic::LdVDEA => self.load_into_memory(Registers::a(), Registers::address_in_de()),
            Mnemonic::LdAVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_a())
            }
            Mnemonic::LdBVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_b())
            }
            Mnemonic::LdCVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_c())
            }
            Mnemonic::LdDVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_d())
            }
            Mnemonic::LdEVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_e())
            }
            Mnemonic::LdHVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_h())
            }
            Mnemonic::LdLVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_l())
            }

            Mnemonic::LdBA => self.load_register_into_register(Registers::a(), Registers::into_b()),
            Mnemonic::LdBB => self.load_register_into_register(Registers::b(), Registers::into_b()),
            Mnemonic::LdBC => self.load_register_into_register(Registers::c(), Registers::into_b()),
            Mnemonic::LdBD => self.load_register_into_register(Registers::d(), Registers::into_b()),
            Mnemonic::LdBE => self.load_register_into_register(Registers::e(), Registers::into_b()),
            Mnemonic::LdBH => self.load_register_into_register(Registers::h(), Registers::into_b()),
            Mnemonic::LdBL => self.load_register_into_register(Registers::l(), Registers::into_b()),

            Mnemonic::LdCA => self.load_register_into_register(Registers::a(), Registers::into_c()),
            Mnemonic::LdCB => self.load_register_into_register(Registers::b(), Registers::into_c()),
            Mnemonic::LdCC => self.load_register_into_register(Registers::c(), Registers::into_c()),
            Mnemonic::LdCD => self.load_register_into_register(Registers::d(), Registers::into_c()),
            Mnemonic::LdCE => self.load_register_into_register(Registers::e(), Registers::into_c()),
            Mnemonic::LdCH => self.load_register_into_register(Registers::h(), Registers::into_c()),
            Mnemonic::LdCL => self.load_register_into_register(Registers::l(), Registers::into_c()),

            Mnemonic::LdDA => self.load_register_into_register(Registers::a(), Registers::into_d()),
            Mnemonic::LdDB => self.load_register_into_register(Registers::b(), Registers::into_d()),
            Mnemonic::LdDC => self.load_register_into_register(Registers::c(), Registers::into_d()),
            Mnemonic::LdDD => self.load_register_into_register(Registers::d(), Registers::into_d()),
            Mnemonic::LdDE => self.load_register_into_register(Registers::e(), Registers::into_d()),
            Mnemonic::LdDH => self.load_register_into_register(Registers::h(), Registers::into_d()),
            Mnemonic::LdDL => self.load_register_into_register(Registers::l(), Registers::into_d()),

            Mnemonic::LdEA => self.load_register_into_register(Registers::a(), Registers::into_e()),
            Mnemonic::LdEB => self.load_register_into_register(Registers::b(), Registers::into_e()),
            Mnemonic::LdEC => self.load_register_into_register(Registers::c(), Registers::into_e()),
            Mnemonic::LdED => self.load_register_into_register(Registers::d(), Registers::into_e()),
            Mnemonic::LdEE => self.load_register_into_register(Registers::e(), Registers::into_e()),
            Mnemonic::LdEH => self.load_register_into_register(Registers::h(), Registers::into_e()),
            Mnemonic::LdEL => self.load_register_into_register(Registers::l(), Registers::into_e()),

            Mnemonic::LdHA => self.load_register_into_register(Registers::a(), Registers::into_h()),
            Mnemonic::LdHB => self.load_register_into_register(Registers::b(), Registers::into_h()),
            Mnemonic::LdHC => self.load_register_into_register(Registers::c(), Registers::into_h()),
            Mnemonic::LdHD => self.load_register_into_register(Registers::d(), Registers::into_h()),
            Mnemonic::LdHE => self.load_register_into_register(Registers::e(), Registers::into_h()),
            Mnemonic::LdHH => self.load_register_into_register(Registers::h(), Registers::into_h()),
            Mnemonic::LdHL => self.load_register_into_register(Registers::l(), Registers::into_h()),

            Mnemonic::LdLA => self.load_register_into_register(Registers::a(), Registers::into_l()),
            Mnemonic::LdLB => self.load_register_into_register(Registers::b(), Registers::into_l()),
            Mnemonic::LdLC => self.load_register_into_register(Registers::c(), Registers::into_l()),
            Mnemonic::LdLD => self.load_register_into_register(Registers::d(), Registers::into_l()),
            Mnemonic::LdLE => self.load_register_into_register(Registers::e(), Registers::into_l()),
            Mnemonic::LdLH => self.load_register_into_register(Registers::h(), Registers::into_l()),
            Mnemonic::LdLL => self.load_register_into_register(Registers::l(), Registers::into_l()),

            Mnemonic::LdHLA => self.load_register_into_memory(Registers::a(), Registers::hl()),
            Mnemonic::LdHLB => self.load_register_into_memory(Registers::b(), Registers::hl()),
            Mnemonic::LdHLC => self.load_register_into_memory(Registers::c(), Registers::hl()),
            Mnemonic::LdHLD => self.load_register_into_memory(Registers::d(), Registers::hl()),
            Mnemonic::LdHLE => self.load_register_into_memory(Registers::e(), Registers::hl()),
            Mnemonic::LdHLH => self.load_register_into_memory(Registers::h(), Registers::hl()),
            Mnemonic::LdHLL => self.load_register_into_memory(Registers::l(), Registers::hl()),

            Mnemonic::LdAA => self.load_register_into_register(Registers::a(), Registers::into_a()),
            Mnemonic::LdAB => self.load_register_into_register(Registers::b(), Registers::into_a()),
            Mnemonic::LdAC => self.load_register_into_register(Registers::c(), Registers::into_a()),
            Mnemonic::LdAD => self.load_register_into_register(Registers::d(), Registers::into_a()),
            Mnemonic::LdAE => self.load_register_into_register(Registers::e(), Registers::into_a()),
            Mnemonic::LdAH => self.load_register_into_register(Registers::h(), Registers::into_a()),
            Mnemonic::LdAL => self.load_register_into_register(Registers::l(), Registers::into_a()),

            Mnemonic::LdAX => self.load_value_into_register(Registers::into_a()),
            Mnemonic::LdBX => self.load_value_into_register(Registers::into_b()),
            Mnemonic::LdCX => self.load_value_into_register(Registers::into_c()),
            Mnemonic::LdDX => self.load_value_into_register(Registers::into_d()),
            Mnemonic::LdEX => self.load_value_into_register(Registers::into_e()),
            Mnemonic::LdHX => self.load_value_into_register(Registers::into_h()),
            Mnemonic::LdLX => self.load_value_into_register(Registers::into_l()),

            Mnemonic::LdAVBC => {
                self.load_memory_into_register(Registers::address_in_bc(), Registers::into_a())
            }
            Mnemonic::LdAVDE => {
                self.load_memory_into_register(Registers::address_in_de(), Registers::into_a())
            }
            Mnemonic::LdVXXHL => self.load_wide_register_into_param_memory(Registers::hl()),
            Mnemonic::LdHLVXX => self.load_param_memory_into_wide_register(Registers::into_hl()),
            Mnemonic::LdVXXA => self.load_register_into_param_memory(Registers::a()),
            Mnemonic::LdAVXX => self.load_param_memory_into_register(Registers::into_a()),
            Mnemonic::LdVHLX => self.load_param_into_memory(Registers::hl()),

            Mnemonic::AndA => self.and_register(Registers::a()),
            Mnemonic::AndB => self.and_register(Registers::b()),
            Mnemonic::AndC => self.and_register(Registers::c()),
            Mnemonic::AndD => self.and_register(Registers::d()),
            Mnemonic::AndE => self.and_register(Registers::e()),
            Mnemonic::AndH => self.and_register(Registers::h()),
            Mnemonic::AndL => self.and_register(Registers::l()),
            Mnemonic::AndX => self.and_value(),

            Mnemonic::OrA => self.or_register(Registers::a()),
            Mnemonic::OrB => self.or_register(Registers::b()),
            Mnemonic::OrC => self.or_register(Registers::c()),
            Mnemonic::OrD => self.or_register(Registers::d()),
            Mnemonic::OrE => self.or_register(Registers::e()),
            Mnemonic::OrH => self.or_register(Registers::h()),
            Mnemonic::OrL => self.or_register(Registers::l()),
            Mnemonic::OrX => self.or_value(),

            Mnemonic::XorA => self.xor_register(Registers::a()),
            Mnemonic::XorB => self.xor_register(Registers::b()),
            Mnemonic::XorC => self.xor_register(Registers::c()),
            Mnemonic::XorD => self.xor_register(Registers::d()),
            Mnemonic::XorE => self.xor_register(Registers::e()),
            Mnemonic::XorH => self.xor_register(Registers::h()),
            Mnemonic::XorL => self.xor_register(Registers::l()),
            Mnemonic::XorX => self.xor_value(),

            Mnemonic::PushAF => self.push_to_stack(Registers::af()),
            Mnemonic::PushBC => self.push_to_stack(Registers::bc()),
            Mnemonic::PushDE => self.push_to_stack(Registers::de()),
            Mnemonic::PushHL => self.push_to_stack(Registers::hl()),

            Mnemonic::PopAF => self.pop_from_stack(Registers::into_af()),
            Mnemonic::PopBC => self.pop_from_stack(Registers::into_bc()),
            Mnemonic::PopDE => self.pop_from_stack(Registers::into_de()),
            Mnemonic::PopHL => self.pop_from_stack(Registers::into_hl()),

            Mnemonic::RLCA => self.rotate_accumulator_copy_left(),
            Mnemonic::RRCA => self.rotate_accumulator_copy_right(),
            Mnemonic::RLA => self.rotate_accumulator_left(),
            Mnemonic::RRA => self.rotate_accumulator_right(),
        }

        callbacks.do_after_instruction_exec(self, instruction);
        callbacks.do_after_instruction_exec_match(self, instruction);
    }

    fn next_byte(&mut self) -> u8 {
        let pc = alu::get_word(self.cpu.state.pc);
        let val = self.ram.read_u8(pc);
        let (result, overflow) = pc.overflowing_add(1);
        if overflow {
            self.stop();
        } else {
            self.cpu.state.pc = alu::get_octets(result);
        }
        val
    }

    fn next_byte_pair(&mut self) -> (u8, u8) {
        let low = self.next_byte();
        let high = self.next_byte();
        (high, low)
    }

    fn next_word(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;
        (high << 8) | low
    }

    pub fn clock(&mut self, _tstates: u8) {
        // TODO: Something.
    }
}
