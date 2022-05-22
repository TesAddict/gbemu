use std::fs::File;
use std::io::prelude::*;

pub enum CartridgeType {
    RomOnly = 0x00,
    Mbc1 = 0x01,
    Mbc1Ram = 0x02,
    Mbc1RamBattery = 0x03,
    Mbc2 = 0x05,
    Mbc2Battery = 0x06,
    RomRam = 0x08,
    RomRamBattery = 0x09,
    Mmm01 = 0x0B,
    Mmm01Ram = 0x0C,
    Mmm01RamBattery = 0x0D,
}

impl CartridgeType {
    fn from_u8(n: u8) -> Option<CartridgeType> {
        match n {
            0x00 => Some(CartridgeType::RomOnly),
            0x01 => Some(CartridgeType::Mbc1),
            0x02 => Some(CartridgeType::Mbc1Ram),
            0x03 => Some(CartridgeType::Mbc1RamBattery),
            0x05 => Some(CartridgeType::Mbc2),
            0x06 => Some(CartridgeType::Mbc2Battery),
            0x08 => Some(CartridgeType::RomRam),
            0x09 => Some(CartridgeType::RomRamBattery),
            0x0B => Some(CartridgeType::Mmm01),
            0x0C => Some(CartridgeType::Mmm01Ram),
            0x0D => Some(CartridgeType::Mmm01RamBattery),
            _    => None,
        }
    }
}
pub struct Cartridge {
    rom : Vec<u8>,
    rom_sz : usize,
    ctype: CartridgeType
}

static CTYPE_ADDR: usize = 0x0147;
static ROM_SIZE_ADDR: usize = 0x0148;

fn decode_cartridge_header(cart: &mut Cartridge) -> bool {
    // Get Cartridge Type.
    let ctype_opt = CartridgeType::from_u8(cart.rom[CTYPE_ADDR]);
    match ctype_opt {
        Some(ctype) => {
            println!("Cartridge::decode_header - Cartridge Type: {}", cart.rom[CTYPE_ADDR]);
            cart.ctype = ctype
        },
        None => {
            println!("Cartridge::decode_header - Unknown Cartridge Type: {}", cart.rom[CTYPE_ADDR]);
            return false;
        }
    }
    // Get ROM size.
    let rom_sz = cart.rom[ROM_SIZE_ADDR];
    if cart.rom.len() != (32*1024) << rom_sz {
        println!("Cartridge::decode_header - Cartridge size from header does not match ROM size.");
        return false;
    }
    cart.rom_sz = (32*1024) << rom_sz;
    println!("Cartridge::decode_header - Cartridge Size: {}", cart.rom_sz);
    return true;
}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge { rom: vec![], rom_sz: (32*1024), ctype: CartridgeType::RomOnly }
    }

    pub fn load_cartridge(cart: &mut Cartridge, rom_path: &String) -> usize {
        let mut file= match File::open(rom_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Cartridge::load_cartridge - ROM at path {} was not found. Error: {}\n", rom_path, err);
                return 0;
            }
        };
        Vec::resize(&mut cart.rom, file.metadata().unwrap().len() as usize, 0);
        match file.read(&mut cart.rom) {
            Ok(size) => {
                if decode_cartridge_header(cart) {
                    return size
                }
            },
            Err(err) => {
                println!("Cartridge::load_cartridge - No data has been loaded from ROM at path {}. Error: {}\n", rom_path, err);
                return 0;
            }
        };
        return 0;
    }

    // TODO: Implement memory mapper. 
    pub fn read(cart: &Cartridge, addr: u16) -> u8 {
        return cart.rom[usize::from(addr)];
    }
}