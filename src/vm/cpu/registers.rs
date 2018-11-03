pub struct Registers {
    pub af: (u8, u8),
    pub bc: (u8, u8),
    pub de: (u8, u8),
    pub hl: (u8, u8),
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: (0x00, 0x00),
            bc: (0x00, 0x00),
            de: (0x00, 0x00),
            hl: (0x00, 0x00),
        }
    }
}
