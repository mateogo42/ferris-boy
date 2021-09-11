mod cpu;
mod memory;
use std::fs;

fn main() {
    let rom = fs::read("roms/asteroids.gba").unwrap();
    println!("{:#04X?}", rom);
}
