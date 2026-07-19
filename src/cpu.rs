pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    flag_z: bool,
    flag_n: bool,
    flag_h: bool,
    flag_c: bool,
    pc: u16,
    sp: u16,
    cycles: u64,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flag_z: false,
            flag_n: false,
            flag_h: false,
            flag_c: false,
            pc: 0x0100,
            sp: 0xFFFE,
            cycles: 0,
        }
    }
}
