use crate::vm::instructions::opcodes::Opcode;
use crate::vm::machine::Machine;
use std::collections::HashMap;

pub struct Callbacks {
    before_instruction_fetch: Vec<Box<FnMut(&mut Machine)>>,
    before_instruction_exec: Vec<Box<FnMut(&mut Machine, Opcode)>>,
    after_instruction_exec: Vec<Box<FnMut(&mut Machine, Opcode)>>,

    before_instruction_exec_match: HashMap<Opcode, Vec<Box<FnMut(&mut Machine)>>>,
    after_instruction_exec_match: HashMap<Opcode, Vec<Box<FnMut(&mut Machine)>>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks {
            before_instruction_fetch: Vec::new(),
            before_instruction_exec: Vec::new(),
            after_instruction_exec: Vec::new(),

            before_instruction_exec_match: HashMap::new(),
            after_instruction_exec_match: HashMap::new(),
        }
    }

    pub fn on_before_instruction_fetch(&mut self, callback: Box<FnMut(&mut Machine)>) {
        self.before_instruction_fetch.push(callback);
    }

    pub(crate) fn do_before_instruction_fetch(&mut self, machine: &mut Machine) {
        for c in &mut self.before_instruction_fetch {
            c(machine);
        }
    }

    pub fn on_before_instruction_exec(&mut self, callback: Box<FnMut(&mut Machine, Opcode)>) {
        self.before_instruction_exec.push(callback);
    }

    pub(crate) fn do_before_instruction_exec(
        &mut self,
        machine: &mut Machine,
        instruction: Opcode,
    ) {
        for c in &mut self.before_instruction_exec {
            c(machine, instruction);
        }
    }

    pub fn on_after_instruction_exec(&mut self, callback: Box<FnMut(&mut Machine, Opcode)>) {
        self.after_instruction_exec.push(callback);
    }

    pub(crate) fn do_after_instruction_exec(&mut self, machine: &mut Machine, instruction: Opcode) {
        for c in &mut self.after_instruction_exec {
            c(machine, instruction);
        }
    }

    pub fn on_before_instruction_exec_match(
        &mut self,
        instruction: Opcode,
        callback: Box<FnMut(&mut Machine)>,
    ) {
        if !self
            .before_instruction_exec_match
            .contains_key(&instruction)
        {
            self.before_instruction_exec_match
                .insert(instruction, Vec::new());
        }
        let list = self.before_instruction_exec_match.get_mut(&instruction);
        list.unwrap().push(callback);
    }

    pub(crate) fn do_before_instruction_exec_match(
        &mut self,
        machine: &mut Machine,
        instruction: Opcode,
    ) {
        if self
            .before_instruction_exec_match
            .contains_key(&instruction)
        {
            for c in self
                .before_instruction_exec_match
                .get_mut(&instruction)
                .unwrap()
            {
                c(machine);
            }
        }
    }

    pub fn on_after_instruction_exec_match(
        &mut self,
        instruction: Opcode,
        callback: Box<FnMut(&mut Machine)>,
    ) {
        if !self.after_instruction_exec_match.contains_key(&instruction) {
            self.after_instruction_exec_match
                .insert(instruction, Vec::new());
        }
        let list = self.after_instruction_exec_match.get_mut(&instruction);
        list.unwrap().push(callback);
    }

    pub(crate) fn do_after_instruction_exec_match(
        &mut self,
        machine: &mut Machine,
        instruction: Opcode,
    ) {
        if self.after_instruction_exec_match.contains_key(&instruction) {
            for c in self
                .after_instruction_exec_match
                .get_mut(&instruction)
                .unwrap()
            {
                c(machine);
            }
        }
    }
}
