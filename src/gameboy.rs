use crate::Cartridge;
use crate::Bus;
use crate::Cpu;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
}

impl GameBoy {
    pub fn power_on() -> GameBoy {
        GameBoy { cpu: Cpu::new(), bus: Bus::new() }
    }

    pub fn load_game(&mut self, path: String) {
       self.bus.cartridge.load_cartridge(&path); 
    }

    pub fn load_buffer(&mut self, buffer: &Vec<u8>) {
        self.bus.cartridge.load_cartridge_w_buffer(buffer);
    }

    pub fn run(&mut self) {
        let instr = self.cpu.fetch(&mut self.bus);
        self.cpu.execute(&mut self.bus, instr);
    }
}