use program::Program;
use vm::cpu::alu;
use vm::cpu::processor::Processor;
use vm::cpu::state::State;
use vm::ram::memory::Memory;

pub struct Machine {
    pub cpu: Processor,
    pub ram: Memory,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Processor::new(),
            ram: Memory::new(),
        }
    }

    pub fn load_at(&mut self, program: &Program, start_address: u16) -> bool {
        let end = start_address as u32 + program.raw().len() as u32;
        let will_fit = end <= 65536;
        let mut address = start_address;
        if will_fit {
            for value in program.raw() {
                self.ram.write_u8(address, *value);
                address = address.wrapping_add(1);
            }
        }
        will_fit
    }

    pub fn load(&mut self, program: &Program) -> bool {
        self.load_at(program, 0)
    }

    pub fn start_at(&mut self, address: u16) {
        self.cpu.halt();
        self.cpu.goto(address);
        self.cpu.unhalt();
        while !self.cpu.is_halted() {
            self.execute();
        }
    }

    pub fn start(&mut self) {
        self.start_at(0);
    }

    pub fn get_register<T>(&self, selector: fn(&State) -> T) -> T {
        selector(&self.cpu.state)
    }

    pub fn get_register_mut<T: Copy + Clone>(&mut self, selector: fn(&mut State) -> &mut T) -> T {
        *selector(&mut self.cpu.state)
    }

    pub fn get_register_pair(&self, selector: fn(&State) -> (u8, u8)) -> u16 {
        alu::get_word(self.get_register(selector))
    }

    pub fn get_register_pair_mut(&mut self, selector: fn(&mut State) -> &mut (u8, u8)) -> u16 {
        alu::get_word((
            selector(&mut self.cpu.state).0,
            selector(&mut self.cpu.state).1,
        ))
    }

    pub fn set_register<T>(&mut self, selector: fn(&mut State) -> &mut T, value: T) {
        *selector(&mut self.cpu.state) = value;
    }

    pub fn set_register_pair(&mut self, selector: fn(&mut State) -> &mut (u8, u8), value: u16) {
        *selector(&mut self.cpu.state) = alu::get_octets(value);
    }
}
