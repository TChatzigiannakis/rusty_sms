#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0x00,

    LdBCXX = 0x01,
    LdVBCA = 0x02,

    IncBC = 0x03,
    IncB = 0x04,
    DecB = 0x05,

    LdBX = 0x06,

    DecBC = 0x0B,
    IncC = 0x0C,
    DecC = 0x0D,

    LdDEXX = 0x11,
    LdVDEA = 0x12,

    IncDE = 0x13,
    IncD = 0x14,
    DecD = 0x15,
    DecDE = 0x1B,
    IncE = 0x1C,
    DecE = 0x1D,

    LdHLXX = 0x21,
    LdVXXHL = 0x22,

    IncHL = 0x23,
    IncH = 0x24,
    DecH = 0x25,
    DecHL = 0x2B,
    IncL = 0x2C,
    DecL = 0x2D,

    LdSPXX = 0x31,
    LdVXXA = 0x32,

    IncSP = 0x33,
    DecSP = 0x3B,
    IncA = 0x3C,
    DecA = 0x3D,

    Halt = 0x76,

    AddB = 0x80,
    AddC = 0x81,
    AddD = 0x82,
    AddE = 0x83,
    AddH = 0x84,
    AddL = 0x85,
    AddA = 0x87,

    AdcB = 0x88,
    AdcC = 0x89,
    AdcD = 0x8A,
    AdcE = 0x8B,
    AdcH = 0x8C,
    AdcL = 0x8D,
    AdcA = 0x8F,

    SubB = 0x90,
    SubC = 0x91,
    SubD = 0x92,
    SubE = 0x93,
    SubH = 0x94,
    SubL = 0x95,
    SubA = 0x97,

    SbcB = 0x98,
    SbcC = 0x99,
    SbcD = 0x9A,
    SbcE = 0x9B,
    SbcH = 0x9C,
    SbcL = 0x9D,
    SbcA = 0x9F,

    AndB = 0xA0,
    AndC = 0xA1,
    AndD = 0xA2,
    AndE = 0xA3,
    AndH = 0xA4,
    AndL = 0xA5,
    AndA = 0xA7,

    XorB = 0xA8,
    XorC = 0xA9,
    XorD = 0xAA,
    XorE = 0xAB,
    XorH = 0xAC,
    XorL = 0xAD,
    XorA = 0xAF,

    OrB = 0xB0,
    OrC = 0xB1,
    OrD = 0xB2,
    OrE = 0xB3,
    OrH = 0xB4,
    OrL = 0xB5,
    OrHL = 0xB6,
    OrA = 0xB7,

    CpB = 0xB8,
    CpC = 0xB9,
    CpD = 0xBA,
    CpE = 0xBB,
    CpH = 0xBC,
    CpL = 0xBD,
    CpHL = 0xBE,
    CpA = 0xBF,

    RetNZ = 0xC0,
    PopBC = 0xC1,
    JpNZXX = 0xC2,
    JpXX = 0xC3,
    CallNZXX = 0xC4,
    PushDE = 0xC5,
    AddAX = 0xC6,
    Rst00h = 0xC7,
    RetZ = 0xC8,
    Ret = 0xC9,
    JpZXX = 0xCA,
    BITS = 0xCB,
    CallZXX = 0xCC,
    CallXX = 0xCD,
    AdcAX = 0xCE,
    Rst08h = 0xCF,
    RetNC = 0xD0,
    PopDE = 0xD1,
    JpNCXX = 0xD2,
    OutXA = 0xD3,
    CallNcXX = 0xD4,
    PushXX = 0xD5,
    SubX = 0xD6,
    Rst10h = 0xD7,
    RetC = 0xD8,
    Exx = 0xD9,
    JpCXX = 0xDA,
    InAX = 0xDB,
    CallCXX = 0xDC,
    IX = 0xDD,
    SbcAX = 0xDE,
    Rst18h = 0xDF,
    RetPO = 0xE0,
    PopHL = 0xE1,
    JpPOXX = 0xE2,
    ExSPHL = 0xE3,
    CallPoXX = 0xE4,
    PushHL = 0xE5,
    AndX = 0xE6,
    Rst20h = 0xE7,
    RetPE = 0xE8,
    JpHL = 0xE9,
    JpPEXX = 0xEA,
    ExDeHl = 0xEB,
    CallPeXX = 0xEC,
    EXTD = 0xED,
    XorX = 0xEE,
    Rst28h = 0xEF,
    RetP = 0xF0,
    PopAF = 0xF1,
    JpPXX = 0xF2,
    Di = 0xF3,
    CallPXX = 0xF4,
    PushAF = 0xF5,
    OrX = 0xF6,
    Rst30h = 0xF7,
    RetM = 0xF8,
    LdSpHl = 0xF9,
    JpMXX = 0xFA,
    Ei = 0xFB,
    CallMXX = 0xFC,
    IY = 0xFD,
    CpX = 0xFE,
    Rst38h = 0xFF,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        unsafe { ::std::mem::transmute_copy::<u8, Opcode>(&value) }
    }
}
