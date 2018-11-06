mod arithmetic_16bit;
mod arithmetic_8bit;
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

use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

impl Machine {
    pub fn execute(&mut self) {
        let opcode = Opcode::from(self.next_byte());
        match opcode {
            Opcode::Nop => self.nop(),
            Opcode::SCF => self.set_carry_flag(),
            Opcode::CCF => self.complement_carry_flag(),
            Opcode::CPL => self.complement_registers(|cpu| &mut cpu.registers.af.0),
            Opcode::Halt => self.halt(),

            Opcode::Exx => self.shadow_exchange_bc_de_hl(),
            Opcode::ExAFAF => self.shadow_exchange_af(),
            Opcode::ExDEHL => self.exhange_de_with_hl(),
            Opcode::ExVSPHL => self.exchage_memory_from_sp_with_hl(),

            Opcode::IncA => self.increment_register(|cpu| &mut cpu.registers.af.0),
            Opcode::IncB => self.increment_register(|cpu| &mut cpu.registers.bc.0),
            Opcode::IncC => self.increment_register(|cpu| &mut cpu.registers.bc.1),
            Opcode::IncD => self.increment_register(|cpu| &mut cpu.registers.de.0),
            Opcode::IncE => self.increment_register(|cpu| &mut cpu.registers.de.1),
            Opcode::IncH => self.increment_register(|cpu| &mut cpu.registers.hl.0),
            Opcode::IncL => self.increment_register(|cpu| &mut cpu.registers.hl.1),

            Opcode::DecA => self.decrement_register(|cpu| &mut cpu.registers.af.0),
            Opcode::DecB => self.decrement_register(|cpu| &mut cpu.registers.bc.0),
            Opcode::DecC => self.decrement_register(|cpu| &mut cpu.registers.bc.1),
            Opcode::DecD => self.decrement_register(|cpu| &mut cpu.registers.de.0),
            Opcode::DecE => self.decrement_register(|cpu| &mut cpu.registers.de.1),
            Opcode::DecH => self.decrement_register(|cpu| &mut cpu.registers.hl.0),
            Opcode::DecL => self.decrement_register(|cpu| &mut cpu.registers.hl.1),

            Opcode::IncBC => self.increment_register_pair(|cpu| &mut cpu.registers.bc),
            Opcode::IncDE => self.increment_register_pair(|cpu| &mut cpu.registers.de),
            Opcode::IncHL => self.increment_register_pair(|cpu| &mut cpu.registers.hl),
            Opcode::IncSP => self.increment_register_pair(|cpu| &mut cpu.sp),

            Opcode::DecBC => self.decrement_register_pair(|cpu| &mut cpu.registers.bc),
            Opcode::DecDE => self.decrement_register_pair(|cpu| &mut cpu.registers.de),
            Opcode::DecHL => self.decrement_register_pair(|cpu| &mut cpu.registers.hl),
            Opcode::DecSP => self.decrement_register_pair(|cpu| &mut cpu.sp),

            Opcode::AddA => self.add_register(|cpu| cpu.registers.af.0),
            Opcode::AddB => self.add_register(|cpu| cpu.registers.bc.0),
            Opcode::AddC => self.add_register(|cpu| cpu.registers.bc.1),
            Opcode::AddD => self.add_register(|cpu| cpu.registers.de.0),
            Opcode::AddE => self.add_register(|cpu| cpu.registers.de.1),
            Opcode::AddH => self.add_register(|cpu| cpu.registers.hl.0),
            Opcode::AddL => self.add_register(|cpu| cpu.registers.hl.1),

            Opcode::SubA => self.subtract_register(|cpu| cpu.registers.af.0),
            Opcode::SubB => self.subtract_register(|cpu| cpu.registers.bc.0),
            Opcode::SubC => self.subtract_register(|cpu| cpu.registers.bc.1),
            Opcode::SubD => self.subtract_register(|cpu| cpu.registers.de.0),
            Opcode::SubE => self.subtract_register(|cpu| cpu.registers.de.1),
            Opcode::SubH => self.subtract_register(|cpu| cpu.registers.hl.0),
            Opcode::SubL => self.subtract_register(|cpu| cpu.registers.hl.1),

            Opcode::AddHLBC => self.add_register_pair_to_hl(|cpu| cpu.registers.bc),
            Opcode::AddHLDE => self.add_register_pair_to_hl(|cpu| cpu.registers.de),
            Opcode::AddHLHL => self.add_register_pair_to_hl(|cpu| cpu.registers.hl),
            Opcode::AddHLSP => self.add_register_pair_to_hl(|cpu| cpu.sp),

            Opcode::AdcA => self.add_carry_register(|cpu| cpu.registers.af.0),
            Opcode::AdcB => self.add_carry_register(|cpu| cpu.registers.bc.0),
            Opcode::AdcC => self.add_carry_register(|cpu| cpu.registers.bc.1),
            Opcode::AdcD => self.add_carry_register(|cpu| cpu.registers.de.0),
            Opcode::AdcE => self.add_carry_register(|cpu| cpu.registers.de.1),
            Opcode::AdcH => self.add_carry_register(|cpu| cpu.registers.hl.0),
            Opcode::AdcL => self.add_carry_register(|cpu| cpu.registers.hl.1),

            Opcode::SbcA => self.subtract_carry_register(|cpu| cpu.registers.af.0),
            Opcode::SbcB => self.subtract_carry_register(|cpu| cpu.registers.bc.0),
            Opcode::SbcC => self.subtract_carry_register(|cpu| cpu.registers.bc.1),
            Opcode::SbcD => self.subtract_carry_register(|cpu| cpu.registers.de.0),
            Opcode::SbcE => self.subtract_carry_register(|cpu| cpu.registers.de.1),
            Opcode::SbcH => self.subtract_carry_register(|cpu| cpu.registers.hl.0),
            Opcode::SbcL => self.subtract_carry_register(|cpu| cpu.registers.hl.1),

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

            Opcode::LdBCXX => self.load_into_register_pair(|cpu| &mut cpu.registers.bc),
            Opcode::LdDEXX => self.load_into_register_pair(|cpu| &mut cpu.registers.de),
            Opcode::LdHLXX => self.load_into_register_pair(|cpu| &mut cpu.registers.hl),
            Opcode::LdSPXX => self.load_into_register_pair(|cpu| &mut cpu.sp),

            Opcode::LdVBCA => {
                self.load_into_memory(|cpu| cpu.registers.af.0, |cpu| cpu.registers.bc)
            }
            Opcode::LdVDEA => {
                self.load_into_memory(|cpu| cpu.registers.af.0, |cpu| cpu.registers.de)
            }
            Opcode::LdAVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.af.0),
            Opcode::LdBVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.bc.0),
            Opcode::LdCVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.bc.1),
            Opcode::LdDVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.de.0),
            Opcode::LdEVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.de.1),
            Opcode::LdHVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.hl.0),
            Opcode::LdLVHL => self
                .load_memory_into_register(|cpu| cpu.registers.hl, |cpu| &mut cpu.registers.hl.1),

            Opcode::LdBA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.bc.0,
            ),
            Opcode::LdBL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.bc.0,
            ),

            Opcode::LdCA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.bc.1,
            ),
            Opcode::LdCL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.bc.1,
            ),

            Opcode::LdDA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.de.0,
            ),
            Opcode::LdDL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.de.0,
            ),

            Opcode::LdEA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdEB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdEC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdED => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdEE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdEH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.de.1,
            ),
            Opcode::LdEL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.de.1,
            ),

            Opcode::LdHA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.hl.0,
            ),
            Opcode::LdHL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.hl.0,
            ),

            Opcode::LdLA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.hl.1,
            ),
            Opcode::LdLL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.hl.1,
            ),

            Opcode::LdHLA => {
                self.load_register_into_memory(|cpu| cpu.registers.af.0, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLB => {
                self.load_register_into_memory(|cpu| cpu.registers.bc.0, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLC => {
                self.load_register_into_memory(|cpu| cpu.registers.bc.1, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLD => {
                self.load_register_into_memory(|cpu| cpu.registers.de.0, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLE => {
                self.load_register_into_memory(|cpu| cpu.registers.de.1, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLH => {
                self.load_register_into_memory(|cpu| cpu.registers.hl.0, |cpu| cpu.registers.hl)
            }
            Opcode::LdHLL => {
                self.load_register_into_memory(|cpu| cpu.registers.hl.1, |cpu| cpu.registers.hl)
            }

            Opcode::LdAA => self.load_register_into_register(
                |cpu| cpu.registers.af.0,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAB => self.load_register_into_register(
                |cpu| cpu.registers.bc.0,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAC => self.load_register_into_register(
                |cpu| cpu.registers.bc.1,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAD => self.load_register_into_register(
                |cpu| cpu.registers.de.0,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAE => self.load_register_into_register(
                |cpu| cpu.registers.de.1,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAH => self.load_register_into_register(
                |cpu| cpu.registers.hl.0,
                |cpu| &mut cpu.registers.af.0,
            ),
            Opcode::LdAL => self.load_register_into_register(
                |cpu| cpu.registers.hl.1,
                |cpu| &mut cpu.registers.af.0,
            ),

            Opcode::LdAX => self.load_into_register(|cpu| &mut cpu.registers.af.0),
            Opcode::LdBX => self.load_into_register(|cpu| &mut cpu.registers.bc.0),
            Opcode::LdCX => self.load_into_register(|cpu| &mut cpu.registers.bc.1),
            Opcode::LdDX => self.load_into_register(|cpu| &mut cpu.registers.de.0),
            Opcode::LdEX => self.load_into_register(|cpu| &mut cpu.registers.de.1),
            Opcode::LdHX => self.load_into_register(|cpu| &mut cpu.registers.hl.0),
            Opcode::LdLX => self.load_into_register(|cpu| &mut cpu.registers.hl.1),

            Opcode::LdAVBC => self
                .load_memory_into_register(|cpu| cpu.registers.bc, |cpu| &mut cpu.registers.af.0),
            Opcode::LdAVDE => self
                .load_memory_into_register(|cpu| cpu.registers.de, |cpu| &mut cpu.registers.af.0),
            Opcode::LdVXXHL => self.load_wide_register_into_param_memory(|cpu| cpu.registers.hl),
            Opcode::LdHLVXX => {
                self.load_param_memory_into_wide_register(|cpu| &mut cpu.registers.hl)
            }
            Opcode::LdVXXA => self.load_register_into_param_memory(|cpu| cpu.registers.af.0),
            Opcode::LdAVXX => self.load_param_memory_into_register(|cpu| &mut cpu.registers.af.0),
            Opcode::LdVHLX => self.load_param_into_memory(|cpu| cpu.registers.hl),

            Opcode::AndA => self.and_register(|cpu| cpu.registers.af.0),
            Opcode::AndB => self.and_register(|cpu| cpu.registers.bc.0),
            Opcode::AndC => self.and_register(|cpu| cpu.registers.bc.1),
            Opcode::AndD => self.and_register(|cpu| cpu.registers.de.0),
            Opcode::AndE => self.and_register(|cpu| cpu.registers.de.1),
            Opcode::AndH => self.and_register(|cpu| cpu.registers.hl.0),
            Opcode::AndL => self.and_register(|cpu| cpu.registers.hl.1),
            Opcode::AndX => self.and_value(),

            Opcode::OrA => self.or_register(|cpu| cpu.registers.af.0),
            Opcode::OrB => self.or_register(|cpu| cpu.registers.bc.0),
            Opcode::OrC => self.or_register(|cpu| cpu.registers.bc.1),
            Opcode::OrD => self.or_register(|cpu| cpu.registers.de.0),
            Opcode::OrE => self.or_register(|cpu| cpu.registers.de.1),
            Opcode::OrH => self.or_register(|cpu| cpu.registers.hl.0),
            Opcode::OrL => self.or_register(|cpu| cpu.registers.hl.1),
            Opcode::OrX => self.or_value(),

            Opcode::XorA => self.xor_register(|cpu| cpu.registers.af.0),
            Opcode::XorB => self.xor_register(|cpu| cpu.registers.bc.0),
            Opcode::XorC => self.xor_register(|cpu| cpu.registers.bc.1),
            Opcode::XorD => self.xor_register(|cpu| cpu.registers.de.0),
            Opcode::XorE => self.xor_register(|cpu| cpu.registers.de.1),
            Opcode::XorH => self.xor_register(|cpu| cpu.registers.hl.0),
            Opcode::XorL => self.xor_register(|cpu| cpu.registers.hl.1),
            Opcode::XorX => self.xor_value(),

            Opcode::PushAF => self.push_to_stack(|cpu| cpu.registers.af),
            Opcode::PushBC => self.push_to_stack(|cpu| cpu.registers.bc),
            Opcode::PushDE => self.push_to_stack(|cpu| cpu.registers.de),
            Opcode::PushHL => self.push_to_stack(|cpu| cpu.registers.hl),

            Opcode::PopAF => self.pop_from_stack(|cpu| &mut cpu.registers.af),
            Opcode::PopBC => self.pop_from_stack(|cpu| &mut cpu.registers.bc),
            Opcode::PopDE => self.pop_from_stack(|cpu| &mut cpu.registers.de),
            Opcode::PopHL => self.pop_from_stack(|cpu| &mut cpu.registers.hl),

            Opcode::RLCA => self.rotate_accumulator_copy_left(),
            Opcode::RRCA => self.rotate_accumulator_copy_right(),
            Opcode::RLA => self.rotate_accumulator_left(),
            Opcode::RRA => self.rotate_accumulator_right(),
        }
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
