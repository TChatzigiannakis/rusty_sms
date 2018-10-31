use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn load_wide_register_into_param_memory(
        &mut self,
        selector: fn(&Registers) -> (u8, u8),
    ) {
        let address = self.next_word();
        let (high_val, low_val) = selector(&self.cpu.state.registers);
        let value = Registers::u8s_to_u16(high_val, low_val);
        self.ram.write_u16(address, value);
        self.clock(16);
    }

    pub(crate) fn load_param_memory_into_wide_register(
        &mut self,
        selector: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        {
            let address = self.next_word();
            let (high_addr, low_addr) = selector(&mut self.cpu.state.registers);
            let value = self.ram.read_u16(address);
            let (high_val, low_val) = Registers::u16_to_u8s(value);
            *high_addr = high_val;
            *low_addr = low_val;
        }
        self.clock(16);
    }

    pub(crate) fn load_param_into_memory(&mut self, selector: fn(&Registers) -> (u8, u8)) {
        let (high_addr, low_addr) = selector(&self.cpu.state.registers);
        let address = Registers::u8s_to_u16(high_addr, low_addr);
        let value = self.next_byte();
        self.ram.write_u8(address, value);
        self.clock(10);
    }

    pub(crate) fn load_into_register_pair(
        &mut self,
        selector: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        {
            let (high_val, low_val) = self.next_byte_pair();
            let (high_reg, low_reg) = selector(&mut self.cpu.state.registers);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        self.clock(10);
    }
}
