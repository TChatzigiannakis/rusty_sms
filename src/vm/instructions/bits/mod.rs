mod opcodes;

use vm::machine::Machine;

impl Machine {
    pub(crate) fn execute_bits(&mut self) {
        let _opcode = self.next_byte();
    }
}
