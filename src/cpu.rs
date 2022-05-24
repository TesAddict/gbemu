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

    fn wait(&self, cycles: u8) {

    }

    fn debug_zero(&self, mnem: String, code: u16) {
        println!("{:#6} - {:#04x}\n", mnem, code);
    }

    fn debug_one(&self, mnem:String, code: u16, one: u8) {
        println!("{:#6} - {:#04x} {:#04x}\n", mnem, code, one);
    }

    fn debug_two(&self, mnem: String, code: u16, one: u8, two: u8) {
        println!("{:#6} - {:#04x} {:#04x} {:#04x}\n", mnem, code, one, two);
    }

    fn nop(&mut self) {
        self.debug_zero("NOP".to_string(), 0x0000);
        self.wait(1);
        self.pc += 1;
    }

    fn ld_bc_d16(&mut self, bus: &mut Bus) {
        self.b = bus.read(self.pc+2);
        self.c = bus.read(self.pc+1);
        self.debug_two("LDBCD16".to_string(),0x0001, self.c, self.b);
        self.wait(12);
        self.pc += 3;
    }

    fn ld_da(&mut self) {
        self.debug_zero("LDDA".to_string(), 0x0057);
        self.d = self.a;
        self.wait(4);
        self.pc += 1; 
    }

    fn jp_a16(&mut self, bus: &mut Bus) {

        self.debug_wait(bus, 0x00C3, "JPA16".to_string(), 3, 16);
        self.pc = u16::from(bus.read(self.pc+2)) << 8 | u16::from(bus.read(self.pc+1));
    }

    fn di(&mut self, bus: &mut Bus) {
        self.debug_wait(bus, 0x00F3, "DI".to_string(), 1, 4);
        let data: u8 = bus.read(0xFFFF) & 0b11100000;
        bus.write(0xFFFF, data);
        self.pc += 1;
    }

    pub fn execute(&mut self, bus: &mut Bus, instr: u16) -> bool {
        match instr {
            0x0000 => self.nop(),
            0x0001 => self.ld_bc_d16(bus),
            0x0057 => self.ld_da(),
            0x00C3 => self.jp_a16(bus),
            0x00F3 => self.di(bus),
            _ => {
                println!("Cpu:execute - Instruction {:#04x} not implemented.\n", instr);
                return false;
            }
        }
        return true;
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