use crate::BusTrait;
mod instructions;
#[cfg(test)]
mod test;
pub use instructions::INSTRUCTION_TABLE;
pub use instructions::INSTRUCTION_TABLE_CB;
pub use instructions::Instruction;
pub use instructions::Type;

macro_rules! reg_map_get {
    ($self: expr, $bus: expr, $op: expr) => {{
        match $op % 0x8 {
            0x0 => $self.b,
            0x1 => $self.c,
            0x2 => $self.d,
            0x3 => $self.e,
            0x4 => $self.h,
            0x5 => $self.l,
            0x6 => $bus.read(($self.h as u16) << 8 | $self.l as u16),
            0x7 => $self.a,
            _   => 0
        }
    }}
}

macro_rules! reg_map_set {
    ($self: expr, $bus: expr, $op: expr, $value:expr) => {{
        match $op % 0x8 {
            0x0 => $self.b = $value,
            0x1 => $self.c = $value,
            0x2 => $self.d = $value,
            0x3 => $self.e = $value,
            0x4 => $self.h = $value,
            0x5 => $self.l = $value,
            0x6 => $bus.write(($self.h as u16) << 8 | $self.l as u16, $value),
            0x7 => $self.a = $value,
            _   => ()
        }
    }}
}

macro_rules! ld_reg {
    ($self: expr, $bus: expr, $reg: expr, $op: expr) => {{
        $reg = reg_map_get!($self, $bus, $op)
    }}
}

macro_rules! add_reg {
    ($self: expr, $bus: expr, $op: expr) => {{
        let reg = reg_map_get!($self, $bus, $op);
        $self.hf = if ($self.a & 0x0f) + (reg & 0x0f) > 0x0f { 1 } else { 0 };
        $self.cf = if ($self.a + reg) as u16 > 0xff { 1 } else { 0 };
        $self.a += reg;
        $self.zf = if $self.a == 0 { 1 } else { 0 };
        $self.nf = 0;
    }}
}

macro_rules! cb_rlc {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        cb_rl!($self, $bus, $op);
        $self.cf = if val & 0x80 != 0 { 1 } else { 0 };
    }}
}

macro_rules! cb_rrc {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        cb_rr!($self, $bus, $op);
        $self.cf = if val & 0x01 != 0 { 1 } else { 0 };
    }}
}

macro_rules! cb_rl {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, (val << 1) | ((val & 0x80) >> 7));
        $self.zf = if (val << 1) | ((val & 0x80) >> 7) == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
    }}
}

macro_rules! cb_rr {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, (val >> 1) | ((val & 0x01) << 7));
        $self.zf = if (val >> 1) | ((val & 0x01) << 7) == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
    }}
}

macro_rules! cb_sla {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, val << 1 | val & 0x01);
        $self.zf = if val << 1 | val & 0x01 == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
        $self.cf = if val & 0x80 != 0 { 1 } else { 0 };
    }}
}

macro_rules! cb_sra {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, val >> 1 | val & 0x80);
        $self.zf = if val >> 1 | val & 0x80 == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
        $self.cf = if val & 0x01 == 1 { 1 } else { 0 };
    }}
}

macro_rules! cb_swap {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, (val << 4 | val >> 4));
        $self.zf = if val << 4 | val >> 4 == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
        $self.cf = 0;
    }}
}

macro_rules! cb_srl {
    ($self: expr, $bus: expr, $op: expr) => {{
        let val = reg_map_get!($self, $bus, $op);
        reg_map_set!($self, $bus, $op, val >> 1);
        $self.zf = if val >> 1 == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.hf = 0;
        $self.cf = if val & 0x01 != 0 { 1 } else { 0 };
    }}
}

macro_rules! cb_res_bit {
    ($self: expr, $bus: expr, $op: expr, $bit: literal) => {{
        reg_map_set!($self, $bus, $op, 
            reg_map_get!($self, $bus, $op) & !(0b1 << $bit));
    }}
}

macro_rules! cb_set_bit {
    ($self: expr, $bus: expr, $op: expr, $bit: literal) => {{
        reg_map_set!($self, $bus, $op, 
            reg_map_get!($self, $bus, $op) | (0b1 << $bit));
    }}
}

macro_rules! cb_bit {
    ($self: expr, $bus: expr, $op: expr, $bit: literal) => {{
        $self.zf = reg_map_get!($self, $bus, $op) & (0b1 << $bit);
        $self.nf = 0;
        $self.hf = 1;
    }}
}

#[derive(Debug)]
pub struct Sharp8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    zf: u8,
    nf: u8,
    hf: u8,
    cf: u8,
    ime: bool,
}

impl Sharp8080 {
    pub fn new(pc: u16) -> Sharp8080 {
        Sharp8080 { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, 
            pc: pc, zf: 0, nf: 0, hf: 0, cf: 0, ime: true }
    }

    fn apply_flags(&self) {

    }

    pub fn fetch_opcode(&self, bus: &dyn BusTrait) -> u16 {
        match bus.read(self.pc) {
            0xCB => {
                (0xCB as u16) << 8 |
                  bus.read(self.pc+1) as u16
            } 
            _ => { bus.read(self.pc) as u16}
        }
    }

    pub fn execute(&mut self, bus: &mut dyn BusTrait, opcode: u16) {
        // pc_base points to the first param or next opcode.
        let (instruction, pc_base) = if opcode & 0xFF00 == 0xCB00 {
            (&INSTRUCTION_TABLE_CB[((opcode & 0x00FF) as usize)], self.pc+2)
        } else {
            (&INSTRUCTION_TABLE[(opcode & 0x00FF) as usize], self.pc+1)
        };
        match instruction.encoding {
            Type::N => {
                match opcode {
                    0x0000          => (),
                    0x0002          => self.ld_bc_a(),
                    0x0003          => self.inc_bc(),

                    0x0040..=0x0047 => ld_reg!(self, bus, self.b, opcode),
                    0x0048..=0x004f => ld_reg!(self, bus, self.c, opcode),
                    0x0050..=0x0057 => ld_reg!(self, bus, self.d, opcode),
                    0x0058..=0x005f => ld_reg!(self, bus, self.e, opcode),
                    0x0060..=0x0067 => ld_reg!(self, bus, self.h, opcode),
                    0x0068..=0x006f => ld_reg!(self, bus, self.l, opcode),

                    0x0078..=0x007f => ld_reg!(self, bus, self.a, opcode),

                    0x0080..=0x0087 => add_reg!(self, bus, opcode),

                    0x00F3          => self.ime = false,
                    _               => self.undefined_instruction(),
                }
                self.pc += instruction.length;
                self.decode_type_n(instruction);
            }
            Type::D16 => {
                let b0 = bus.read(pc_base);
                let b1 = bus.read(pc_base+1);
                match opcode {
                    0x0001 => self.ld_bc_d16(b0, b1),
                    _      => self.undefined_instruction(), 
                }
            }
            Type::A16 => {
                let address = (bus.read(pc_base+1) as u16) << 8 | 
                               bus.read(pc_base) as u16;
                match opcode {
                    0x00C3 => self.pc = address,
                    _      => self.undefined_instruction()
                }
                self.decode_type_a16(instruction, address);
            }
            Type::CB => {
                match opcode {
                    0xCB00..=0xCB07 => cb_rlc!(self, bus, opcode),
                    0xCB08..=0xCB0F => cb_rrc!(self, bus, opcode),
                    0xCB10..=0xCB17 => cb_rl!(self, bus, opcode),
                    0xCB18..=0xCB1F => cb_rr!(self, bus, opcode),
                    0xCB20..=0xCB27 => cb_sla!(self, bus, opcode),
                    0xCB28..=0xCB2F => cb_sra!(self, bus, opcode),
                    0xCB30..=0xCB37 => cb_swap!(self, bus, opcode),
                    0xCB38..=0xCB3F => cb_srl!(self, bus, opcode),
                    0xCB40..=0xCB47 => cb_bit!(self, bus, opcode, 0),
                    0xCB48..=0xCB4F => cb_bit!(self, bus, opcode, 1),
                    0xCB50..=0xCB57 => cb_bit!(self, bus, opcode, 2),
                    0xCB58..=0xCB5F => cb_bit!(self, bus, opcode, 3),
                    0xCB60..=0xCB67 => cb_bit!(self, bus, opcode, 4),
                    0xCB68..=0xCB6F => cb_bit!(self, bus, opcode, 5),
                    0xCB70..=0xCB77 => cb_bit!(self, bus, opcode, 6),
                    0xCB78..=0xCB7F => cb_bit!(self, bus, opcode, 7),
                    0xCB80..=0xCB87 => cb_res_bit!(self, bus, opcode, 0),
                    0xCB88..=0xCB8F => cb_res_bit!(self, bus, opcode, 1),
                    0xCB90..=0xCB97 => cb_res_bit!(self, bus, opcode, 2),
                    0xCB98..=0xCB9F => cb_res_bit!(self, bus, opcode, 3),
                    0xCBA0..=0xCBA7 => cb_res_bit!(self, bus, opcode, 4),
                    0xCBA8..=0xCBAF => cb_res_bit!(self, bus, opcode, 5),
                    0xCBB0..=0xCBB7 => cb_res_bit!(self, bus, opcode, 6),
                    0xCBB8..=0xCBBF => cb_res_bit!(self, bus, opcode, 7),
                    0xCBC0..=0xCBC7 => cb_set_bit!(self, bus, opcode, 0),
                    0xCBC8..=0xCBCF => cb_set_bit!(self, bus, opcode, 1),
                    0xCBD0..=0xCBD7 => cb_set_bit!(self, bus, opcode, 2),
                    0xCBD8..=0xCBDF => cb_set_bit!(self, bus, opcode, 3),
                    0xCBE0..=0xCBE7 => cb_set_bit!(self, bus, opcode, 4),
                    0xCBE8..=0xCBEF => cb_set_bit!(self, bus, opcode, 5),
                    0xCBF0..=0xCBF7 => cb_set_bit!(self, bus, opcode, 6),
                    0xCBF8..=0xCBFF => cb_set_bit!(self, bus, opcode, 7),
                    _      => self.undefined_instruction()
                }
                self.pc += instruction.length;
            }
            _ => {
                self.undefined_type();
            }
        }
        self.apply_flags();
        self.wait(instruction.cycles);
    }

    fn wait(&self, cycles: u8) {

    }

    fn decode_type_n(&self, instruction: &Instruction) {
        println!("{}", instruction.mnemonic);
    }

    fn decode_type_a16(&self, instruction: &Instruction, address: u16) {
        println!("{} - Address: {:#06x}", instruction.mnemonic, address)
    }

    fn ld_bc_a(&self) {

    }

    fn inc_bc(&self) {

    }

    fn ld_bc_d16(&self, b0: u8, b1: u8) {

    }

    fn undefined_instruction(&self) {
        println!("{:?}", self);
        panic!("Undefined Instruction!\n");
    }

    fn undefined_type(&self) {

    }
}
