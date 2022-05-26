use crate::Cartridge;
use crate::Bus;
use crate::Sharp8080;

pub struct GameBoy {
    cpu: Sharp8080,
    bus: Bus,
}

impl GameBoy {
    pub fn power_on() -> GameBoy {
        GameBoy { cpu: Sharp8080::new(), bus: Bus::new() }
    }

    pub fn load_game(&mut self, path: String) {
       self.bus.cartridge.load_cartridge(&path); 
    }

    pub fn load_buffer(&mut self, buffer: &Vec<u8>) {
        self.bus.cartridge.load_cartridge_w_buffer(buffer);
    }

    pub fn run(&mut self) {
        while true {
            let opcode = self.cpu.fetch_opcode(&self.bus);
            self.cpu.execute(&mut self.bus, opcode);
        }
    }
}