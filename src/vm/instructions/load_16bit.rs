use vm::cpu::alu;
use vm::cpu::state::State;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn load_wide_register_into_param_memory(
        &mut self,
        selector: fn(&State) -> (u8, u8),
    ) {
        let address = self.next_word();
        let value = alu::get_word(selector(&self.cpu.state));
        self.ram.write_u16(address, value);
        self.clock(16);
    }

    pub(crate) fn load_param_memory_into_wide_register(
        &mut self,
        selector: fn(&mut State) -> &mut (u8, u8),
    ) {
        {
            let address = self.next_word();
            let (high_addr, low_addr) = selector(&mut self.cpu.state);
            let value = self.ram.read_u16(address);
            let (high_val, low_val) = alu::get_octets(value);
            *high_addr = high_val;
            *low_addr = low_val;
        }
        self.clock(16);
    }

    pub(crate) fn load_param_into_memory(&mut self, selector: fn(&State) -> (u8, u8)) {
        let address = alu::get_word(selector(&self.cpu.state));
        let value = self.next_byte();
        self.ram.write_u8(address, value);
        self.clock(10);
    }

    pub(crate) fn load_into_register_pair(&mut self, selector: fn(&mut State) -> &mut (u8, u8)) {
        {
            let (high_val, low_val) = self.next_byte_pair();
            let (high_reg, low_reg) = selector(&mut self.cpu.state);
            *high_reg = high_val;
            *low_reg = low_val;
        }
        self.clock(10);
    }
}
