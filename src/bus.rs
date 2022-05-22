use crate::Cartridge;

pub struct Bus {
    pub cartridge: Cartridge,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { cartridge: Cartridge::new() }
    }

    pub fn write(&mut self, addr: u16, data: u8) {

    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => {
                return self.cartridge.read(addr);
            },
            _ => return 0
        }
    }
}