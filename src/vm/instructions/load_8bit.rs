use vm::cpu::alu;
use vm::cpu::state::State;
use vm::machine::Machine;
use vm::AddressSelector;

impl Machine {
    pub(crate) fn load_register_into_register(
        &mut self,
        source_selector: fn(&State) -> u8,
        dest_selector: fn(&mut State) -> &mut u8,
    ) {
        {
            let source = source_selector(&self.cpu.state);
            let dest = dest_selector(&mut self.cpu.state);
            *dest = source;
        }
        self.clock(4);
    }

    pub(crate) fn load_memory_into_register(
        &mut self,
        pointer: AddressSelector,
        selector: fn(&mut State) -> &mut u8,
    ) {
        {
            let address = pointer(&self.cpu.state);
            let value = self.ram.read_u8(address);
            let dest = selector(&mut self.cpu.state);
            *dest = value;
        }
        self.clock(7);
    }

    pub(crate) fn load_register_into_memory(
        &mut self,
        selector: fn(&State) -> u8,
        pointer: fn(&State) -> (u8, u8),
    ) {
        {
            let address = alu::get_word(pointer(&self.cpu.state));
            let value = selector(&self.cpu.state);
            self.ram.write_u8(address, value);
        }
        self.clock(7);
    }

    pub(crate) fn load_register_into_param_memory(&mut self, selector: fn(&State) -> u8) {
        let address = self.next_word();
        let value = selector(&self.cpu.state);
        self.ram.write_u8(address, value);
        self.clock(13);
    }

    pub(crate) fn load_param_memory_into_register(&mut self, selector: fn(&mut State) -> &mut u8) {
        {
            let address = self.next_word();
            let value = self.ram.read_u8(address);
            let dest = selector(&mut self.cpu.state);
            *dest = value;
        }
        self.clock(13);
    }

    pub(crate) fn load_into_register(&mut self, selector: fn(&mut State) -> &mut u8) {
        let value = self.next_byte();
        *selector(&mut self.cpu.state) = value;
        self.clock(7);
    }

    pub(crate) fn load_into_memory(&mut self, source: fn(&State) -> u8, pointer: AddressSelector) {
        {
            let value = source(&self.cpu.state);
            let address = pointer(&self.cpu.state);
            self.ram.write_u8(address, value);
        }
        self.clock(7);
    }
}
