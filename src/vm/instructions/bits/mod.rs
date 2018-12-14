mod mnemonics;

use crate::vm::instructions::bits::mnemonics::BitsMnemonic;
use crate::vm::machine::Machine;

impl Machine {
    pub(crate) fn execute_bits(&mut self) {
        let _instruction = BitsMnemonic::from(self.next_byte());
    }
}
