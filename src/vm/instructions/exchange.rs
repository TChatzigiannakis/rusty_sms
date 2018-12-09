use crate::vm::cpu::alu;
use crate::vm::cpu::registers::Registers;
use crate::vm::cpu::state::State;
use crate::vm::machine::Machine;
use std::mem;

impl Machine {
    pub(crate) fn shadow_exchange_af(&mut self) {
        self.exchange_with_shadow(&[|regs| &mut regs.af]);
        self.clock(4);
    }

    pub(crate) fn shadow_exchange_bc_de_hl(&mut self) {
        self.exchange_with_shadow(&[
            |regs| &mut regs.bc,
            |regs| &mut regs.de,
            |regs| &mut regs.hl,
        ]);
        self.clock(4);
    }

    pub(crate) fn exhange_de_with_hl(&mut self) {
        self.exchange(&[|cpu| (&mut cpu.registers.de, &mut cpu.registers.hl)]);
        self.clock(4);
    }

    pub(crate) fn exchage_memory_from_sp_with_hl(&mut self) {
        {
            let sp = alu::get_word(self.cpu.state.sp);
            let reg_value = alu::get_word(self.cpu.state.registers.hl);
            let mem_value = self.ram.read_u16(sp);
            self.cpu.state.registers.hl = alu::get_octets(mem_value);
            self.ram.write_u16(sp, reg_value);
        }
        self.clock(19);
    }

    fn exchange(&mut self, selectors: &[fn(&mut State) -> (&mut (u8, u8), &mut (u8, u8))]) {
        for s in selectors {
            let (r1, r2) = s(&mut self.cpu.state);
            mem::swap(r1, r2);
        }
    }

    fn exchange_with_shadow(&mut self, selectors: &[fn(&mut Registers) -> &mut (u8, u8)]) {
        let reg = &mut self.cpu.state.registers;
        let alt = &mut self.cpu.state.alt_registers;
        for s in selectors {
            mem::swap(s(reg), s(alt));
        }
    }
}
