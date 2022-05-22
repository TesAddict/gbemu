mod gameboy;
mod cpu;
mod bus;
mod cartridge;
use gameboy::*;
use cpu::*;
use bus::*;
use cartridge::*;

fn main() {
    let mut gb = GameBoy::power_on();
    gb.load_game("roms/hello.gb".to_string());
    gb.run();
}
