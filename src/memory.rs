#![allow(dead_code)]

// 0000 - 3FFF
const ROM_0_SIZE: usize = 1024 * 16;
// 4000 - 7FFF
const SWITCHABLE_ROM_SIZE: usize = 1024 * 16;
// 8000 - 9FFF
const VRAM_SIZE: usize = 1024 * 8;
// A000 - BFFF
const EXTERNAL_RAM_SIZE: usize = 1024 * 8;
// C000 - DFFF
const WRAM_SIZE: usize = 1024 * 8;
// FE00 - FE9F
const OAM_SIZE: usize = 160;
// FF00 - FF7F
const IO_PORTS_SIZE: usize = 128;
// FF80 - 9FFF
const HRAM_SIZE: usize = 1024 * 8;

pub struct Memory {
    rom_0: [u8; ROM_0_SIZE],
    switchable_rom: [u8; SWITCHABLE_ROM_SIZE],
    vram: [u8; VRAM_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io_ports: [u8; IO_PORTS_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: bool,
}

impl Memory {
    fn new() -> Self {
        Self {
            rom_0: [0; ROM_0_SIZE],
            switchable_rom: [0; SWITCHABLE_ROM_SIZE],
            vram: [0; VRAM_SIZE],
            external_ram: [0; EXTERNAL_RAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_ports: [0; IO_PORTS_SIZE],
            hram: [0; HRAM_SIZE],
            ie: false,
        }
    }
    fn read_u8(&self, addr: u8) -> u8 {
        self.wram[addr as usize]
    }

    fn write_u8(&mut self, addr: u8, val: u8) {
        self.wram[addr as usize] = val;
    }

    fn read_u16(&self, addr: u16) -> u16 {
        let high_bytes = ((addr & 0xFF00) >> 8) as u8;
        let low_bytes = (addr & 0x00FF) as u8;

        (self.read_u8(high_bytes) << 8) as u16 | (self.read_u8(low_bytes) as u16)
    }

    fn write_u16(&mut self, addr: u16, val: u16) {
        let high_bytes_addr = ((addr & 0xFF00) >> 8) as u8;
        let low_bytes_addr = (addr & 0x00FF) as u8;

        let high_bytes_val = ((val & 0xFF00) >> 8) as u8;
        let low_bytes_val = (val & 0x00FF) as u8;
        self.write_u8(high_bytes_addr, high_bytes_val);
        self.write_u8(low_bytes_addr, low_bytes_val)
    }
}
