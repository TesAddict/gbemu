
mod cartridge;
use cartridge::*;

fn main() {
    let mut cart = Cartridge::new();
    let size = Cartridge::load_cartridge(&mut cart, &String::from("roms/hello.gb"));
    Cartridge::read(&cart, 0x100);
    println!("Loaded ROM of size: {}", size);
}
