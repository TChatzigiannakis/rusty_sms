use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

pub struct Callbacks {
    before_instruction_fetch: Vec<Box<FnMut(&Machine)>>,
    before_instruction_exec: Vec<Box<FnMut(&Machine, Opcode)>>,
    after_instruction_exec: Vec<Box<FnMut(&Machine, Opcode)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks {
            before_instruction_fetch: Vec::new(),
            before_instruction_exec: Vec::new(),
            after_instruction_exec: Vec::new(),
        }
    }

    pub fn on_before_instruction_fetch(&mut self, callback: Box<FnMut(&Machine)>) {
        self.before_instruction_fetch.push(callback);
    }

    pub(crate) fn do_before_instruction_fetch(&mut self, machine: &Machine) {
        for c in &mut self.before_instruction_fetch {
            c(machine);
        }
    }

    pub fn on_before_instruction_exec(&mut self, callback: Box<FnMut(&Machine, Opcode)>) {
        self.before_instruction_exec.push(callback);
    }

    pub(crate) fn do_before_instruction_exec(&mut self, machine: &Machine, instruction: Opcode) {
        for c in &mut self.before_instruction_exec {
            c(machine, instruction);
        }
    }

    pub fn on_after_instruction_exec(&mut self, callback: Box<FnMut(&Machine, Opcode)>) {
        self.after_instruction_exec.push(callback);
    }

    pub(crate) fn do_after_instruction_exec(&mut self, machine: &Machine, instructions: Opcode) {
        for c in &mut self.after_instruction_exec {
            c(machine, instructions);
        }
    }
}
