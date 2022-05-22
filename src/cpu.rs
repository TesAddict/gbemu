use crate::Bus;

pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x0100 }
    }

    fn debug_wait(&self, bus: &mut Bus, instr: u16, mnem: String, len: u8, cycles: u8) {
        match len {
            1 => println!("{:#6} - {:#04x}\n", mnem, instr),
            2 => println!("{:#6} - {:#04x} {:#04x}\n", mnem, instr, bus.read(self.pc+1)),
            3 => println!("{:#6} - {:#04x} {:#04x} {:#04x}\n", mnem, instr, bus.read(self.pc+1), bus.read(self.pc+2)),
            _ => println!("Instruction length error.\n")
        }
    }

    fn nop(&mut self, bus: &mut Bus) {
        self.pc += 1;
        self.debug_wait(bus, 0x0000, "NOP".to_string(), 1, 4);
    }

    fn ld_bc_d16(&mut self, bus: &mut Bus) {
        let lo = bus.read(self.pc+1);
        let hi = bus.read(self.pc+2);
        self.debug_wait(bus, 0x0001, "LDBCD16".to_string(), 3, 12);
        self.pc += 3;
        self.b = hi;
        self.c = lo;
    }

    fn jp_a16(&mut self, bus: &mut Bus) {
        let lo = bus.read(self.pc+1);
        let hi = bus.read(self.pc+2);
        self.debug_wait(bus, 0x00C3, "JPA16".to_string(), 3, 16);
        self.pc = u16::from(hi) << 8 | u16::from(lo);
    }

    pub fn execute(&mut self, bus: &mut Bus, instr: u16) {
        match instr {
            0x0000 => self.nop(bus),
            0x0001 => self.ld_bc_d16(bus),
            0x00C3 => self.jp_a16(bus),
            _ => {
                println!("Cpu:execute - Instruction {:#04x} not implemented.\n", instr);
            }
        }
    }

    pub fn fetch(&self, bus: &mut Bus) -> u16 {
        let instr_pre = bus.read(self.pc);
        if instr_pre == 0xCB {
            let instr_post = bus.read(self.pc+1);
            return u16::from(instr_pre) << 8 | u16::from(instr_post);
        }
        return u16::from(instr_pre);
    }
}