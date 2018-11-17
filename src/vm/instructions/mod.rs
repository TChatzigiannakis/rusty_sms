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
pub mod opcodes;
mod rotate_shift;
mod stack;

use vm::callbacks::Callbacks;
use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::cpu::registers::Registers;
use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

impl Machine {
    pub fn execute(&mut self) {
        self.execute_with(&mut Callbacks::new());
    }

    pub fn execute_with(&mut self, callbacks: &mut Callbacks) {
        callbacks.do_before_instruction_fetch(self);
        let opcode = Opcode::from(self.next_byte());

        callbacks.do_before_instruction_exec(self, opcode);
        match opcode {
            Opcode::BITS => self.execute_bits(),

            Opcode::Nop => self.nop(),
            Opcode::SCF => self.set_carry_flag(),
            Opcode::CCF => self.complement_carry_flag(),
            Opcode::CPL => self.complement_registers(Registers::into_a()),
            Opcode::Halt => self.halt(),

            Opcode::Exx => self.shadow_exchange_bc_de_hl(),
            Opcode::ExAFAF => self.shadow_exchange_af(),
            Opcode::ExDEHL => self.exhange_de_with_hl(),
            Opcode::ExVSPHL => self.exchage_memory_from_sp_with_hl(),

            Opcode::IncA => self.increment_register(Registers::into_a()),
            Opcode::IncB => self.increment_register(Registers::into_b()),
            Opcode::IncC => self.increment_register(Registers::into_c()),
            Opcode::IncD => self.increment_register(Registers::into_d()),
            Opcode::IncE => self.increment_register(Registers::into_e()),
            Opcode::IncH => self.increment_register(Registers::into_h()),
            Opcode::IncL => self.increment_register(Registers::into_l()),

            Opcode::DecA => self.decrement_register(Registers::into_a()),
            Opcode::DecB => self.decrement_register(Registers::into_b()),
            Opcode::DecC => self.decrement_register(Registers::into_c()),
            Opcode::DecD => self.decrement_register(Registers::into_d()),
            Opcode::DecE => self.decrement_register(Registers::into_e()),
            Opcode::DecH => self.decrement_register(Registers::into_h()),
            Opcode::DecL => self.decrement_register(Registers::into_l()),

            Opcode::IncBC => self.increment_register_pair(Registers::into_bc()),
            Opcode::IncDE => self.increment_register_pair(Registers::into_de()),
            Opcode::IncHL => self.increment_register_pair(Registers::into_hl()),
            Opcode::IncSP => self.increment_register_pair(Registers::into_sp()),
            Opcode::IncVHL => self.increment_memory(),

            Opcode::DecBC => self.decrement_register_pair(Registers::into_bc()),
            Opcode::DecDE => self.decrement_register_pair(Registers::into_de()),
            Opcode::DecHL => self.decrement_register_pair(Registers::into_hl()),
            Opcode::DecSP => self.decrement_register_pair(Registers::into_sp()),
            Opcode::DecVHL => self.decrement_memory(),

            Opcode::AddA => self.add_register(Registers::a()),
            Opcode::AddB => self.add_register(Registers::b()),
            Opcode::AddC => self.add_register(Registers::c()),
            Opcode::AddD => self.add_register(Registers::d()),
            Opcode::AddE => self.add_register(Registers::e()),
            Opcode::AddH => self.add_register(Registers::h()),
            Opcode::AddL => self.add_register(Registers::l()),
            Opcode::AddVHL => self.add_memory(),

            Opcode::SubA => self.subtract_register(Registers::a()),
            Opcode::SubB => self.subtract_register(Registers::b()),
            Opcode::SubC => self.subtract_register(Registers::c()),
            Opcode::SubD => self.subtract_register(Registers::d()),
            Opcode::SubE => self.subtract_register(Registers::e()),
            Opcode::SubH => self.subtract_register(Registers::h()),
            Opcode::SubL => self.subtract_register(Registers::l()),
            Opcode::SubVHL => self.sub_memory(),

            Opcode::AddHLBC => self.add_register_pair_to_hl(Registers::bc()),
            Opcode::AddHLDE => self.add_register_pair_to_hl(Registers::de()),
            Opcode::AddHLHL => self.add_register_pair_to_hl(Registers::hl()),
            Opcode::AddHLSP => self.add_register_pair_to_hl(Registers::sp()),

            Opcode::AdcA => self.add_carry_register(Registers::a()),
            Opcode::AdcB => self.add_carry_register(Registers::b()),
            Opcode::AdcC => self.add_carry_register(Registers::c()),
            Opcode::AdcD => self.add_carry_register(Registers::d()),
            Opcode::AdcE => self.add_carry_register(Registers::e()),
            Opcode::AdcH => self.add_carry_register(Registers::h()),
            Opcode::AdcL => self.add_carry_register(Registers::l()),
            Opcode::AdcAVHL => self.add_carry_memory(),

            Opcode::SbcA => self.subtract_carry_register(Registers::a()),
            Opcode::SbcB => self.subtract_carry_register(Registers::b()),
            Opcode::SbcC => self.subtract_carry_register(Registers::c()),
            Opcode::SbcD => self.subtract_carry_register(Registers::d()),
            Opcode::SbcE => self.subtract_carry_register(Registers::e()),
            Opcode::SbcH => self.subtract_carry_register(Registers::h()),
            Opcode::SbcL => self.subtract_carry_register(Registers::l()),

            Opcode::JpXX => self.jump(|_| true),
            Opcode::JpNZXX => self.jump(|status| !Flag::Zero.get(status)),
            Opcode::JpZXX => self.jump(|status| Flag::Zero.get(status)),
            Opcode::JpNCXX => self.jump(|status| !Flag::Carry.get(status)),
            Opcode::JpCXX => self.jump(|status| Flag::Carry.get(status)),
            Opcode::JpPOXX => self.jump(|status| Flag::ParityOverflow.get(status)),
            Opcode::JpPEXX => self.jump(|status| !Flag::ParityOverflow.get(status)),
            Opcode::JpPXX => self.jump(|status| !Flag::Sign.get(status)),
            Opcode::JpMXX => self.jump(|status| Flag::Sign.get(status)),

            Opcode::JrX => self.jump_relative(|_| true),
            Opcode::JrCX => self.jump_relative(|status| Flag::Carry.get(status)),
            Opcode::JrNCX => self.jump_relative(|status| !Flag::Carry.get(status)),
            Opcode::JrZX => self.jump_relative(|status| Flag::Zero.get(status)),
            Opcode::JrNZX => self.jump_relative(|status| !Flag::Zero.get(status)),

            Opcode::DjNZX => self.decrement_and_jump_on_non_zero(),

            Opcode::CallXX => self.call(|_| true),
            Opcode::CallNZXX => self.call(|status| !Flag::Zero.get(status)),
            Opcode::CallZXX => self.call(|status| Flag::Zero.get(status)),
            Opcode::CallNCXX => self.call(|status| !Flag::Carry.get(status)),
            Opcode::CallCXX => self.call(|status| Flag::Carry.get(status)),
            Opcode::CallPOXX => self.call(|status| Flag::ParityOverflow.get(status)),
            Opcode::CallPEXX => self.call(|status| !Flag::ParityOverflow.get(status)),
            Opcode::CallPXX => self.call(|status| !Flag::Sign.get(status)),
            Opcode::CallMXX => self.call(|status| Flag::Sign.get(status)),

            Opcode::Ret => self.ret(),
            Opcode::RetNZ => self.ret_conditional(|status| !Flag::Zero.get(status)),
            Opcode::RetZ => self.ret_conditional(|status| Flag::Zero.get(status)),
            Opcode::RetNC => self.ret_conditional(|status| !Flag::Carry.get(status)),
            Opcode::RetC => self.ret_conditional(|status| Flag::Carry.get(status)),
            Opcode::RetPO => self.ret_conditional(|status| Flag::ParityOverflow.get(status)),
            Opcode::RetPE => self.ret_conditional(|status| !Flag::ParityOverflow.get(status)),
            Opcode::RetP => self.ret_conditional(|status| !Flag::Sign.get(status)),
            Opcode::RetM => self.ret_conditional(|status| Flag::Sign.get(status)),

            Opcode::LdBCXX => self.load_into_register_pair(Registers::into_bc()),
            Opcode::LdDEXX => self.load_into_register_pair(Registers::into_de()),
            Opcode::LdHLXX => self.load_into_register_pair(Registers::into_hl()),
            Opcode::LdSPXX => self.load_into_register_pair(Registers::into_sp()),

            Opcode::LdVBCA => self.load_into_memory(Registers::a(), Registers::address_in_bc()),
            Opcode::LdVDEA => self.load_into_memory(Registers::a(), Registers::address_in_de()),
            Opcode::LdAVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_a())
            }
            Opcode::LdBVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_b())
            }
            Opcode::LdCVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_c())
            }
            Opcode::LdDVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_d())
            }
            Opcode::LdEVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_e())
            }
            Opcode::LdHVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_h())
            }
            Opcode::LdLVHL => {
                self.load_memory_into_register(Registers::address_in_hl(), Registers::into_l())
            }

            Opcode::LdBA => self.load_register_into_register(Registers::a(), Registers::into_b()),
            Opcode::LdBB => self.load_register_into_register(Registers::b(), Registers::into_b()),
            Opcode::LdBC => self.load_register_into_register(Registers::c(), Registers::into_b()),
            Opcode::LdBD => self.load_register_into_register(Registers::d(), Registers::into_b()),
            Opcode::LdBE => self.load_register_into_register(Registers::e(), Registers::into_b()),
            Opcode::LdBH => self.load_register_into_register(Registers::h(), Registers::into_b()),
            Opcode::LdBL => self.load_register_into_register(Registers::l(), Registers::into_b()),

            Opcode::LdCA => self.load_register_into_register(Registers::a(), Registers::into_c()),
            Opcode::LdCB => self.load_register_into_register(Registers::b(), Registers::into_c()),
            Opcode::LdCC => self.load_register_into_register(Registers::c(), Registers::into_c()),
            Opcode::LdCD => self.load_register_into_register(Registers::d(), Registers::into_c()),
            Opcode::LdCE => self.load_register_into_register(Registers::e(), Registers::into_c()),
            Opcode::LdCH => self.load_register_into_register(Registers::h(), Registers::into_c()),
            Opcode::LdCL => self.load_register_into_register(Registers::l(), Registers::into_c()),

            Opcode::LdDA => self.load_register_into_register(Registers::a(), Registers::into_d()),
            Opcode::LdDB => self.load_register_into_register(Registers::b(), Registers::into_d()),
            Opcode::LdDC => self.load_register_into_register(Registers::c(), Registers::into_d()),
            Opcode::LdDD => self.load_register_into_register(Registers::d(), Registers::into_d()),
            Opcode::LdDE => self.load_register_into_register(Registers::e(), Registers::into_d()),
            Opcode::LdDH => self.load_register_into_register(Registers::h(), Registers::into_d()),
            Opcode::LdDL => self.load_register_into_register(Registers::l(), Registers::into_d()),

            Opcode::LdEA => self.load_register_into_register(Registers::a(), Registers::into_e()),
            Opcode::LdEB => self.load_register_into_register(Registers::b(), Registers::into_e()),
            Opcode::LdEC => self.load_register_into_register(Registers::c(), Registers::into_e()),
            Opcode::LdED => self.load_register_into_register(Registers::d(), Registers::into_e()),
            Opcode::LdEE => self.load_register_into_register(Registers::e(), Registers::into_e()),
            Opcode::LdEH => self.load_register_into_register(Registers::h(), Registers::into_e()),
            Opcode::LdEL => self.load_register_into_register(Registers::l(), Registers::into_e()),

            Opcode::LdHA => self.load_register_into_register(Registers::a(), Registers::into_h()),
            Opcode::LdHB => self.load_register_into_register(Registers::b(), Registers::into_h()),
            Opcode::LdHC => self.load_register_into_register(Registers::c(), Registers::into_h()),
            Opcode::LdHD => self.load_register_into_register(Registers::d(), Registers::into_h()),
            Opcode::LdHE => self.load_register_into_register(Registers::e(), Registers::into_h()),
            Opcode::LdHH => self.load_register_into_register(Registers::h(), Registers::into_h()),
            Opcode::LdHL => self.load_register_into_register(Registers::l(), Registers::into_h()),

            Opcode::LdLA => self.load_register_into_register(Registers::a(), Registers::into_l()),
            Opcode::LdLB => self.load_register_into_register(Registers::b(), Registers::into_l()),
            Opcode::LdLC => self.load_register_into_register(Registers::c(), Registers::into_l()),
            Opcode::LdLD => self.load_register_into_register(Registers::d(), Registers::into_l()),
            Opcode::LdLE => self.load_register_into_register(Registers::e(), Registers::into_l()),
            Opcode::LdLH => self.load_register_into_register(Registers::h(), Registers::into_l()),
            Opcode::LdLL => self.load_register_into_register(Registers::l(), Registers::into_l()),

            Opcode::LdHLA => self.load_register_into_memory(Registers::a(), Registers::hl()),
            Opcode::LdHLB => self.load_register_into_memory(Registers::b(), Registers::hl()),
            Opcode::LdHLC => self.load_register_into_memory(Registers::c(), Registers::hl()),
            Opcode::LdHLD => self.load_register_into_memory(Registers::d(), Registers::hl()),
            Opcode::LdHLE => self.load_register_into_memory(Registers::e(), Registers::hl()),
            Opcode::LdHLH => self.load_register_into_memory(Registers::h(), Registers::hl()),
            Opcode::LdHLL => self.load_register_into_memory(Registers::l(), Registers::hl()),

            Opcode::LdAA => self.load_register_into_register(Registers::a(), Registers::into_a()),
            Opcode::LdAB => self.load_register_into_register(Registers::b(), Registers::into_a()),
            Opcode::LdAC => self.load_register_into_register(Registers::c(), Registers::into_a()),
            Opcode::LdAD => self.load_register_into_register(Registers::d(), Registers::into_a()),
            Opcode::LdAE => self.load_register_into_register(Registers::e(), Registers::into_a()),
            Opcode::LdAH => self.load_register_into_register(Registers::h(), Registers::into_a()),
            Opcode::LdAL => self.load_register_into_register(Registers::l(), Registers::into_a()),

            Opcode::LdAX => self.load_value_into_register(Registers::into_a()),
            Opcode::LdBX => self.load_value_into_register(Registers::into_b()),
            Opcode::LdCX => self.load_value_into_register(Registers::into_c()),
            Opcode::LdDX => self.load_value_into_register(Registers::into_d()),
            Opcode::LdEX => self.load_value_into_register(Registers::into_e()),
            Opcode::LdHX => self.load_value_into_register(Registers::into_h()),
            Opcode::LdLX => self.load_value_into_register(Registers::into_l()),

            Opcode::LdAVBC => {
                self.load_memory_into_register(Registers::address_in_bc(), Registers::into_a())
            }
            Opcode::LdAVDE => {
                self.load_memory_into_register(Registers::address_in_de(), Registers::into_a())
            }
            Opcode::LdVXXHL => self.load_wide_register_into_param_memory(Registers::hl()),
            Opcode::LdHLVXX => self.load_param_memory_into_wide_register(Registers::into_hl()),
            Opcode::LdVXXA => self.load_register_into_param_memory(Registers::a()),
            Opcode::LdAVXX => self.load_param_memory_into_register(Registers::into_a()),
            Opcode::LdVHLX => self.load_param_into_memory(Registers::hl()),

            Opcode::AndA => self.and_register(Registers::a()),
            Opcode::AndB => self.and_register(Registers::b()),
            Opcode::AndC => self.and_register(Registers::c()),
            Opcode::AndD => self.and_register(Registers::d()),
            Opcode::AndE => self.and_register(Registers::e()),
            Opcode::AndH => self.and_register(Registers::h()),
            Opcode::AndL => self.and_register(Registers::l()),
            Opcode::AndX => self.and_value(),

            Opcode::OrA => self.or_register(Registers::a()),
            Opcode::OrB => self.or_register(Registers::b()),
            Opcode::OrC => self.or_register(Registers::c()),
            Opcode::OrD => self.or_register(Registers::d()),
            Opcode::OrE => self.or_register(Registers::e()),
            Opcode::OrH => self.or_register(Registers::h()),
            Opcode::OrL => self.or_register(Registers::l()),
            Opcode::OrX => self.or_value(),

            Opcode::XorA => self.xor_register(Registers::a()),
            Opcode::XorB => self.xor_register(Registers::b()),
            Opcode::XorC => self.xor_register(Registers::c()),
            Opcode::XorD => self.xor_register(Registers::d()),
            Opcode::XorE => self.xor_register(Registers::e()),
            Opcode::XorH => self.xor_register(Registers::h()),
            Opcode::XorL => self.xor_register(Registers::l()),
            Opcode::XorX => self.xor_value(),

            Opcode::PushAF => self.push_to_stack(Registers::af()),
            Opcode::PushBC => self.push_to_stack(Registers::bc()),
            Opcode::PushDE => self.push_to_stack(Registers::de()),
            Opcode::PushHL => self.push_to_stack(Registers::hl()),

            Opcode::PopAF => self.pop_from_stack(Registers::into_af()),
            Opcode::PopBC => self.pop_from_stack(Registers::into_bc()),
            Opcode::PopDE => self.pop_from_stack(Registers::into_de()),
            Opcode::PopHL => self.pop_from_stack(Registers::into_hl()),

            Opcode::RLCA => self.rotate_accumulator_copy_left(),
            Opcode::RRCA => self.rotate_accumulator_copy_right(),
            Opcode::RLA => self.rotate_accumulator_left(),
            Opcode::RRA => self.rotate_accumulator_right(),
        }
        callbacks.do_after_instruction_exec(self, opcode);
    }

    fn next_byte(&mut self) -> u8 {
        let pc = alu::get_word(self.cpu.state.pc);
        let val = self.ram.read_u8(pc);
        let (result, overflow) = pc.overflowing_add(1);
        if overflow {
            self.cpu.halt();
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
