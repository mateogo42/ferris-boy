#![allow(dead_code)]
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

#[allow(non_camel_case_types)]
pub enum Flag {
    z,
    n,
    h,
    c,
}

pub struct CPU {
    registers: Registers,
}

impl CPU {
    fn new() -> Self {
        let registers = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x100,
        };

        Self { registers }
    }
    fn flag(&self, f: Flag) -> u8 {
        let offset = match f {
            Flag::z => 7,
            Flag::n => 6,
            Flag::h => 5,
            Flag::c => 4,
        };

        self.registers.f >> offset
    }
}
