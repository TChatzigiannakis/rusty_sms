use std::collections::HashMap;
use vm::cpu::state::State;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Flag {
    Carry = 1,
    AddSubtract = 2,
    ParityOverflow = 4,
    Unused1 = 8,
    HalfCarry = 16,
    Unused2 = 32,
    Zero = 64,
    Sign = 128,
}

impl Flag {
    pub fn set(self, state: &mut State, value: bool) {
        let mask = self as u8;
        let reg = &mut state.registers.af.1;
        if value {
            *reg |= mask;
        } else {
            *reg &= !mask;
        };
    }

    pub fn get(self, state: &State) -> bool {
        self.get_bit(state) > 0
    }

    pub fn get_bit(self, state: &State) -> u8 {
        let mask = self as u8;
        let reg = &state.registers.af.1;
        *reg & mask
    }

    pub(crate) fn set_values(state: &mut State, affected: &[Flag], values: &[(Flag, bool)]) {
        let map: HashMap<Flag, bool> = values.iter().cloned().collect();
        for flag in affected {
            match map.get(&flag) {
                Some(value) => flag.set(state, *value),
                None => {}
            }
        }
    }
}
