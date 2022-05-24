use crate::Cartridge;

pub struct Bus {
    pub cartridge: Cartridge,
    memory : [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Bus {
        Bus { cartridge: Cartridge::new(), memory: [0; 0x10000] }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => {

            },
            0x8000..=0xFFFF => {
                self.memory[usize::from(addr)] = data;
            }
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => {
                return self.cartridge.read(addr);
            },
            0x8000..=0xFFFF => {
                return self.memory[usize::from(addr)];
            }
        }
    }
}