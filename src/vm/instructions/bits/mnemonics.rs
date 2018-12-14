#[derive(Copy, Clone)]
#[repr(u8)]
pub enum BitsMnemonic {
    RlcB = 0x00,
}

impl From<u8> for BitsMnemonic {
    fn from(value: u8) -> Self {
        unsafe { ::std::mem::transmute_copy::<u8, BitsMnemonic>(&value) }
    }
}
