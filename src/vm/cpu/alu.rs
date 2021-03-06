use crate::vm::cpu::nibble::Nibble;
use num::One;
use num::Zero;
use std::ops::Add;
use std::ops::Not;

#[derive(Copy, Clone)]
pub(crate) struct AdderResult<T: Copy + Clone> {
    pub value: T,
    pub half_carry: bool,
    pub carry: bool,
    pub overflow: bool,
}

pub(crate) fn get_octets(value: u16) -> (u8, u8) {
    let high = ((value & 0xFF00) >> 8) as u8;
    let low = (value & 0xFF) as u8;
    (high, low)
}

pub(crate) fn get_word(value: (u8, u8)) -> u16 {
    let (high, low) = value;
    ((high as u16) << 8) | low as u16
}

pub(crate) fn get_bit<T: Zero + One>(value: bool) -> T {
    if value {
        num::one()
    } else {
        num::zero()
    }
}

pub(crate) fn negate<T: Add<Output = T> + Not<Output = T> + One>(value: T) -> T {
    !value + num::one()
}

pub(crate) fn sign_extend(value: u8) -> u16 {
    if value > 0x7F {
        negate(value) as u16 | 1 << 15
    } else {
        value as u16
    }
}

pub(crate) fn add_octets(a: u8, b: u8) -> AdderResult<u8> {
    let (low_nibble, half_carry) = Nibble::from_u8(a).overflowing_add(Nibble::from_u8(b));
    let (high_nibble_temp, carry_temp_1) =
        Nibble::from_u8_high(a).overflowing_add(Nibble::from_u8_high(b));
    let (high_nibble, carry_temp_2) = high_nibble_temp.overflowing_add(get_bit(half_carry));
    let carry = carry_temp_1 | carry_temp_2;
    let result = Nibble::u8_from_nibbles(high_nibble, low_nibble);
    let overflow = if a < 0x80 && b < 0x80 {
        result > 0x7F
    } else if a > 0x7F && b > 0x7F {
        result < 0x80
    } else {
        false
    };
    AdderResult {
        value: result,
        half_carry: half_carry,
        carry: carry,
        overflow: overflow,
    }
}

pub(crate) fn add_words(a: u16, b: u16) -> AdderResult<u16> {
    let low = {
        let (_, op1) = get_octets(a);
        let (_, op2) = get_octets(b);
        add_octets(op1, op2)
    };
    let high = {
        let (op1, _) = get_octets(a);
        let (op2, _) = get_octets(b);
        let result_temp = add_octets(op1, op2);
        let mut result = add_octets(result_temp.value, get_bit(low.carry));
        result.half_carry |= result_temp.half_carry;
        result.carry |= result.carry;
        result
    };
    let result = get_word((high.value, low.value));
    let overflow = if a < 0x8000 && b < 0x8000 {
        result > 0x7FFF
    } else if a > 0x7FFF && b > 0x7FFF {
        result < 0x8000
    } else {
        false
    };
    AdderResult {
        value: result,
        half_carry: high.half_carry,
        carry: high.carry,
        overflow: overflow,
    }
}

#[test]
#[cfg(test)]
fn nibbles() {
    for iteration in 0..256 {
        let i = iteration as u8;
        let r = add_octets(i, 1);
        assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
        assert_eq!(i == 0xFF, r.carry, "At {}.", i);
        assert_eq!((i & 0x0F) == 0x0F, r.half_carry, "At {}.", i);
        assert_eq!(i == 0x7F, r.overflow, "At {}.", i);
    }
}

#[test]
#[cfg(test)]
fn words() {
    for iteration in 0..65536 {
        let i = iteration as u16;
        let r = add_words(i, 1);
        assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
        assert_eq!(i == 0xFFFF, r.carry, "At {}.", i);
        assert_eq!((i & 0x0FFF) == 0x0FFF, r.half_carry, "At {}.", i);
        assert_eq!(i == 0x7FFF, r.overflow, "At {}.", i);
    }
}
