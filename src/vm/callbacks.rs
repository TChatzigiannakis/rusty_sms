use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

pub struct Callbacks {
    before_instruction_fetch: Vec<Box<Fn(&Machine, u16)>>,
    before_instruction_exec: Vec<Box<Fn(&Machine, Opcode)>>,
    after_instruction_exec: Vec<Box<Fn(&Machine, Opcode)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks {
            before_instruction_fetch: Vec::new(),
            before_instruction_exec: Vec::new(),
            after_instruction_exec: Vec::new(),
        }
    }

    pub fn on_before_instruction_fetch(&mut self, callback: Box<Fn(&Machine, u16)>) {
        self.before_instruction_fetch.push(callback);
    }

    pub(crate) fn do_before_instruction_fetch(&self, machine: &Machine, program_counter: u16) {
        for c in &self.before_instruction_fetch {
            c(machine, program_counter);
        }
    }

    pub fn on_before_instruction_exec(&mut self, callback: Box<Fn(&Machine, Opcode)>) {
        self.before_instruction_exec.push(callback);
    }

    pub(crate) fn do_before_instruction_exec(&self, machine: &Machine, instruction: Opcode) {
        for c in &self.before_instruction_exec {
            c(machine, instruction);
        }
    }

    pub fn on_after_instruction_exec(&mut self, callback: Box<Fn(&Machine, Opcode)>) {
        self.after_instruction_exec.push(callback);
    }

    pub(crate) fn do_after_instruction_exec(&self, machine: &Machine, instructions: Opcode) {
        for c in &self.after_instruction_exec {
            c(machine, instructions);
        }
    }
}
