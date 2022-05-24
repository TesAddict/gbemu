mod bus;
mod cartridge;
mod sharp8080;
mod gameboy;
use bus::*;
use cartridge::*;
use sharp8080::*;
use gameboy::*;

fn main() {
    let mut gb = GameBoy::power_on();
    gb.load_game("roms/hello.gb".to_string());
    gb.run();
}
