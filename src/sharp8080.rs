use crate::Bus;

macro_rules! reg_map_get {
    ($self: expr, $opcode: expr) => {{
        match $opcode % 0x8 {
            0x0 => $self.b,
            0x1 => $self.c,
            0x2 => $self.d,
            0x3 => $self.e,
            0x4 => $self.h,
            0x5 => $self.l,
            0x6 => $self.l,
            0x7 => $self.a,
            _   => 0
        }
    }}
}

macro_rules! reg_map_set {
    ($self: expr, $opcode: expr, $value:expr) => {{
        match $opcode % 0x8 {
            0x0 => $self.b = $value,
            0x1 => $self.c = $value,
            0x2 => $self.d = $value,
            0x3 => $self.e = $value,
            0x4 => $self.h = $value,
            0x5 => $self.l = $value,
            0x6 => $self.l = $value,
            0x7 => $self.a = $value,
            _   => ()
        }
    }}
}

macro_rules! ld_reg {
    ($self: expr, $reg: expr, $op: expr) => {{
        $reg = reg_map_get!($self, $op)
    }}
}

macro_rules! add_reg {
    ($self: expr, $op: expr) => {{
        let reg = reg_map_get!($self, $op);
        $self.hf = if ($self.a & 0x0f) + (reg & 0x0f) > 0x0f { 1 } else { 0 };
        $self.cf = if ($self.a + reg) as u16 > 0xff { 1 } else { 0 };
        $self.a += reg;
        $self.zf = if $self.a == 0 { 1 } else { 0 };
        $self.nf = 0;
        $self.apply_flags();
    }}
}

macro_rules! cb_res_bit {
    ($self: expr, $op: expr, $bit: literal) => {{
        reg_map_set!($self, $op, reg_map_get!($self, $op) & !(0b1 << $bit));
    }}
}

macro_rules! cb_set_bit {
    ($self: expr, $op: expr, $bit: literal) => {{
        reg_map_set!($self, $op, reg_map_get!($self, $op) | (0b1 << $bit));
    }}
}

macro_rules! cb_bit {
    ($self: expr, $op: expr, $bit: literal) => {{
        $self.zf = reg_map_get!($self, $op) & (0b1 << $bit);
        $self.nf = 0;
        $self.hf = 1;
        $self.apply_flags();
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
    pub fn new() -> Sharp8080 {
        Sharp8080 { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, 
            pc: 0x0100, zf: 0, nf: 0, hf: 0, cf: 0, ime: true }
    }

    fn apply_flags(&self) {

    }

    pub fn step(&self) {
    }

    pub fn fetch_opcode(&self, bus: &Bus) -> u16 {
        match bus.read(self.pc) {
            0xCB => {
                (0xCB as u16) << 8 &
                  bus.read(self.pc+1) as u16
            } 
            _ => { bus.read(self.pc) as u16}
        }
    }

    pub fn execute(&mut self, bus: &mut Bus, opcode: u16) {
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

                    0x0040..=0x0047 => ld_reg!(self, self.b, opcode),
                    0x0048..=0x004f => ld_reg!(self, self.c, opcode),
                    0x0050..=0x0057 => ld_reg!(self, self.d, opcode),
                    0x0058..=0x005f => ld_reg!(self, self.e, opcode),
                    0x0060..=0x0067 => ld_reg!(self, self.h, opcode),
                    0x0068..=0x006f => ld_reg!(self, self.l, opcode),

                    0x0078..=0x007f => ld_reg!(self, self.a, opcode),

                    0x0080..=0x0087 => add_reg!(self, opcode),

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
                    0xCB40..=0xCB47 => cb_bit!(self, opcode, 0),
                    0xCB48..=0xCB4F => cb_bit!(self, opcode, 1),
                    0xCB50..=0xCB57 => cb_bit!(self, opcode, 2),
                    0xCB58..=0xCB5F => cb_bit!(self, opcode, 3),
                    0xCB60..=0xCB67 => cb_bit!(self, opcode, 4),
                    0xCB68..=0xCB6F => cb_bit!(self, opcode, 5),
                    0xCB70..=0xCB77 => cb_bit!(self, opcode, 6),
                    0xCB78..=0xCB7F => cb_bit!(self, opcode, 7),
                    // Restore bit opcodes.
                    0xCB80..=0xCB87 => cb_res_bit!(self, opcode, 0),
                    0xCB88..=0xCB8F => cb_res_bit!(self, opcode, 1),
                    0xCB90..=0xCB97 => cb_res_bit!(self, opcode, 2),
                    0xCB98..=0xCB9F => cb_res_bit!(self, opcode, 3),
                    0xCBA0..=0xCBA7 => cb_res_bit!(self, opcode, 4),
                    0xCBA8..=0xCBAF => cb_res_bit!(self, opcode, 5),
                    0xCBB0..=0xCBB7 => cb_res_bit!(self, opcode, 6),
                    0xCBB8..=0xCBBF => cb_res_bit!(self, opcode, 7),
                    // Set bit opcodes.
                    0xCBC0..=0xCBC7 => cb_set_bit!(self, opcode, 0),
                    0xCBC8..=0xCBCF => cb_set_bit!(self, opcode, 1),
                    0xCBD0..=0xCBD7 => cb_set_bit!(self, opcode, 2),
                    0xCBD8..=0xCBDF => cb_set_bit!(self, opcode, 3),
                    0xCBE0..=0xCBE7 => cb_set_bit!(self, opcode, 4),
                    0xCBE8..=0xCBEF => cb_set_bit!(self, opcode, 5),
                    0xCBF0..=0xCBF7 => cb_set_bit!(self, opcode, 6),
                    0xCBF8..=0xCBFF => cb_set_bit!(self, opcode, 7),
                    _      => self.undefined_instruction()
                }
                self.pc += instruction.length;
            }
            _ => {
                self.undefined_type();
            }
        }
        self.wait(instruction.cycles);
    }

    fn decode_type_n(&self, instruction: &Instruction) {
        println!("{}", instruction.mnemonic);
    }

    fn decode_type_a16(&self, instruction: &Instruction, address: u16) {
        println!("{} - Address: {:#06x}", instruction.mnemonic, address)
    }

    fn wait(&self, cycles: u8) {

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

#[derive(Debug)]
enum Type {
    Unknown,
    N,
    D8,
    D16,
    A8,
    A16,
    R8,
    CB
}

#[derive(Debug)]
struct Instruction {
    encoding: Type,
    mnemonic: &'static str,
    cycles: u8,
    length: u16
}
/*  */
const INSTRUCTION_TABLE: [Instruction; 256] = [
/* 0x00 */ Instruction{encoding:Type::N,mnemonic: "NOP",cycles:4,length:1},
/* 0x01 */ Instruction{encoding:Type::D16,mnemonic:"LDBCD16",cycles:12,length:3},
/* 0x02 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x03 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x04 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x05 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x06 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x07 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x08 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x09 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x0f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x10 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x11 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x12 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x13 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x14 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x15 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x16 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x17 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x18 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x19 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x1f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x20 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x21 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x22 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x23 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x24 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x25 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x26 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x27 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x28 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x29 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x2f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x30 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x31 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x32 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x33 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x34 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x35 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x36 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x37 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x38 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x39 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x3f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x40 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x41 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x42 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x43 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x44 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x45 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x46 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x47 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x48 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x49 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x4f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x50 */ Instruction{encoding:Type::N, mnemonic:"LD_D_B",cycles:4,length:1},
/* 0x51 */ Instruction{encoding:Type::N, mnemonic:"LD_D_C",cycles:4,length:1},
/* 0x52 */ Instruction{encoding:Type::N, mnemonic:"LD_D_D",cycles:4,length:1},
/* 0x53 */ Instruction{encoding:Type::N, mnemonic:"LD_D_E",cycles:4,length:1},
/* 0x54 */ Instruction{encoding:Type::N, mnemonic:"LD_D_H",cycles:4,length:1},
/* 0x55 */ Instruction{encoding:Type::N, mnemonic:"LD_D_L",cycles:4,length:1},
/* 0x56 */ Instruction{encoding:Type::N, mnemonic:"LD_D_HL",cycles:4,length:1},
/* 0x57 */ Instruction{encoding:Type::N, mnemonic:"LD_D_A",cycles:4,length:1},
/* 0x58 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x59 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x5f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x60 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x61 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x62 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x63 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x64 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x65 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x66 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x67 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x68 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x69 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x6f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x70 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x71 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x72 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x73 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x74 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x75 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x76 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x77 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x78 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x79 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x7f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x80 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x81 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x82 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x83 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x84 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x85 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x86 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x87 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x88 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x89 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x8f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x90 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x91 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x92 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x93 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x94 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x95 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x96 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x97 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x98 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x99 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9a */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9b */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9c */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9d */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9e */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x9f */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xa9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xaa */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xab */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xac */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xad */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xae */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xaf */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xb9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xba */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xbb */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xbc */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xbd */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xbe */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xbf */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc3 */ Instruction{encoding:Type::A16, mnemonic:"JP",cycles:16,length:3},
/* 0xc4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xc9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xca */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xcb */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xcc */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xcd */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xce */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xcf */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xd9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xda */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xdb */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xdc */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xdd */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xde */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xdf */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xe9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xea */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xeb */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xec */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xed */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xee */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xef */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf0 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf1 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf2 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf3 */ Instruction{encoding:Type::N, mnemonic:"DI",cycles:4,length:1},
/* 0xf4 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf5 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf6 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf7 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf8 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xf9 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xfa */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xfb */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xfc */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xfd */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xfe */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0xff */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
];

const INSTRUCTION_TABLE_CB: [Instruction; 256] = [
/*0x0*/ Instruction{encoding:Type::CB,mnemonic:"RLC_B",cycles:8,length:2},
/*0x1*/ Instruction{encoding:Type::CB,mnemonic:"RLC_C",cycles:8,length:2},
/*0x2*/ Instruction{encoding:Type::CB,mnemonic:"RLC_D",cycles:8,length:2},
/*0x3*/ Instruction{encoding:Type::CB,mnemonic:"RLC_E",cycles:8,length:2},
/*0x4*/ Instruction{encoding:Type::CB,mnemonic:"RLC_H",cycles:8,length:2},
/*0x5*/ Instruction{encoding:Type::CB,mnemonic:"RLC_L",cycles:8,length:2},
/*0x6*/ Instruction{encoding:Type::CB,mnemonic:"RLC_HL",cycles:16,length:2},
/*0x7*/ Instruction{encoding:Type::CB,mnemonic:"RLC_A",cycles:8,length:2},
/*0x8*/ Instruction{encoding:Type::CB,mnemonic:"RRC_B",cycles:8,length:2},
/*0x9*/ Instruction{encoding:Type::CB,mnemonic:"RRC_C",cycles:8,length:2},
/*0xa*/ Instruction{encoding:Type::CB,mnemonic:"RRC_D",cycles:8,length:2},
/*0xb*/ Instruction{encoding:Type::CB,mnemonic:"RRC_E",cycles:8,length:2},
/*0xc*/ Instruction{encoding:Type::CB,mnemonic:"RRC_H",cycles:8,length:2},
/*0xd*/ Instruction{encoding:Type::CB,mnemonic:"RRC_L",cycles:8,length:2},
/*0xe*/ Instruction{encoding:Type::CB,mnemonic:"RRC_HL",cycles:16,length:2},
/*0xf*/ Instruction{encoding:Type::CB,mnemonic:"RRC_A",cycles:8,length:2},
/*0x10*/ Instruction{encoding:Type::CB,mnemonic:"RL_B",cycles:8,length:2},
/*0x11*/ Instruction{encoding:Type::CB,mnemonic:"RL_C",cycles:8,length:2},
/*0x12*/ Instruction{encoding:Type::CB,mnemonic:"RL_D",cycles:8,length:2},
/*0x13*/ Instruction{encoding:Type::CB,mnemonic:"RL_E",cycles:8,length:2},
/*0x14*/ Instruction{encoding:Type::CB,mnemonic:"RL_H",cycles:8,length:2},
/*0x15*/ Instruction{encoding:Type::CB,mnemonic:"RL_L",cycles:8,length:2},
/*0x16*/ Instruction{encoding:Type::CB,mnemonic:"RL_HL",cycles:16,length:2},
/*0x17*/ Instruction{encoding:Type::CB,mnemonic:"RL_A",cycles:8,length:2},
/*0x18*/ Instruction{encoding:Type::CB,mnemonic:"RR_B",cycles:8,length:2},
/*0x19*/ Instruction{encoding:Type::CB,mnemonic:"RR_C",cycles:8,length:2},
/*0x1a*/ Instruction{encoding:Type::CB,mnemonic:"RR_D",cycles:8,length:2},
/*0x1b*/ Instruction{encoding:Type::CB,mnemonic:"RR_E",cycles:8,length:2},
/*0x1c*/ Instruction{encoding:Type::CB,mnemonic:"RR_H",cycles:8,length:2},
/*0x1d*/ Instruction{encoding:Type::CB,mnemonic:"RR_L",cycles:8,length:2},
/*0x1e*/ Instruction{encoding:Type::CB,mnemonic:"RR_HL",cycles:16,length:2},
/*0x1f*/ Instruction{encoding:Type::CB,mnemonic:"RR_A",cycles:8,length:2},
/*0x20*/ Instruction{encoding:Type::CB,mnemonic:"SLA_B",cycles:8,length:2},
/*0x21*/ Instruction{encoding:Type::CB,mnemonic:"SLA_C",cycles:8,length:2},
/*0x22*/ Instruction{encoding:Type::CB,mnemonic:"SLA_D",cycles:8,length:2},
/*0x23*/ Instruction{encoding:Type::CB,mnemonic:"SLA_E",cycles:8,length:2},
/*0x24*/ Instruction{encoding:Type::CB,mnemonic:"SLA_H",cycles:8,length:2},
/*0x25*/ Instruction{encoding:Type::CB,mnemonic:"SLA_L",cycles:8,length:2},
/*0x26*/ Instruction{encoding:Type::CB,mnemonic:"SLA_HL",cycles:16,length:2},
/*0x27*/ Instruction{encoding:Type::CB,mnemonic:"SLA_A",cycles:8,length:2},
/*0x28*/ Instruction{encoding:Type::CB,mnemonic:"SRA_B",cycles:8,length:2},
/*0x29*/ Instruction{encoding:Type::CB,mnemonic:"SRA_C",cycles:8,length:2},
/*0x2a*/ Instruction{encoding:Type::CB,mnemonic:"SRA_D",cycles:8,length:2},
/*0x2b*/ Instruction{encoding:Type::CB,mnemonic:"SRA_E",cycles:8,length:2},
/*0x2c*/ Instruction{encoding:Type::CB,mnemonic:"SRA_H",cycles:8,length:2},
/*0x2d*/ Instruction{encoding:Type::CB,mnemonic:"SRA_L",cycles:8,length:2},
/*0x2e*/ Instruction{encoding:Type::CB,mnemonic:"SRA_HL",cycles:16,length:2},
/*0x2f*/ Instruction{encoding:Type::CB,mnemonic:"SRA_A",cycles:8,length:2},
/*0x30*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_B",cycles:8,length:2},
/*0x31*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_C",cycles:8,length:2},
/*0x32*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_D",cycles:8,length:2},
/*0x33*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_E",cycles:8,length:2},
/*0x34*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_H",cycles:8,length:2},
/*0x35*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_L",cycles:8,length:2},
/*0x36*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_HL",cycles:16,length:2},
/*0x37*/ Instruction{encoding:Type::CB,mnemonic:"SWAP_A",cycles:8,length:2},
/*0x38*/ Instruction{encoding:Type::CB,mnemonic:"SRL_B",cycles:8,length:2},
/*0x39*/ Instruction{encoding:Type::CB,mnemonic:"SRL_C",cycles:8,length:2},
/*0x3a*/ Instruction{encoding:Type::CB,mnemonic:"SRL_D",cycles:8,length:2},
/*0x3b*/ Instruction{encoding:Type::CB,mnemonic:"SRL_E",cycles:8,length:2},
/*0x3c*/ Instruction{encoding:Type::CB,mnemonic:"SRL_H",cycles:8,length:2},
/*0x3d*/ Instruction{encoding:Type::CB,mnemonic:"SRL_L",cycles:8,length:2},
/*0x3e*/ Instruction{encoding:Type::CB,mnemonic:"SRL_HL",cycles:16,length:2},
/*0x3f*/ Instruction{encoding:Type::CB,mnemonic:"SRL_A",cycles:8,length:2},
/*0x40*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_B",cycles:8,length:2},
/*0x41*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_C",cycles:8,length:2},
/*0x42*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_D",cycles:8,length:2},
/*0x43*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_E",cycles:8,length:2},
/*0x44*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_H",cycles:8,length:2},
/*0x45*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_L",cycles:8,length:2},
/*0x46*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_HL",cycles:16,length:2},
/*0x47*/ Instruction{encoding:Type::CB,mnemonic:"BIT0_A",cycles:8,length:2},
/*0x48*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_B",cycles:8,length:2},
/*0x49*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_C",cycles:8,length:2},
/*0x4a*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_D",cycles:8,length:2},
/*0x4b*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_E",cycles:8,length:2},
/*0x4c*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_H",cycles:8,length:2},
/*0x4d*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_L",cycles:8,length:2},
/*0x4e*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_HL",cycles:16,length:2},
/*0x4f*/ Instruction{encoding:Type::CB,mnemonic:"BIT1_A",cycles:8,length:2},
/*0x50*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_B",cycles:8,length:2},
/*0x51*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_C",cycles:8,length:2},
/*0x52*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_D",cycles:8,length:2},
/*0x53*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_E",cycles:8,length:2},
/*0x54*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_H",cycles:8,length:2},
/*0x55*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_L",cycles:8,length:2},
/*0x56*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_HL",cycles:16,length:2},
/*0x57*/ Instruction{encoding:Type::CB,mnemonic:"BIT2_A",cycles:8,length:2},
/*0x58*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_B",cycles:8,length:2},
/*0x59*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_C",cycles:8,length:2},
/*0x5a*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_D",cycles:8,length:2},
/*0x5b*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_E",cycles:8,length:2},
/*0x5c*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_H",cycles:8,length:2},
/*0x5d*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_L",cycles:8,length:2},
/*0x5e*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_HL",cycles:16,length:2},
/*0x5f*/ Instruction{encoding:Type::CB,mnemonic:"BIT3_A",cycles:8,length:2},
/*0x60*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_B",cycles:8,length:2},
/*0x61*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_C",cycles:8,length:2},
/*0x62*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_D",cycles:8,length:2},
/*0x63*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_E",cycles:8,length:2},
/*0x64*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_H",cycles:8,length:2},
/*0x65*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_L",cycles:8,length:2},
/*0x66*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_HL",cycles:16,length:2},
/*0x67*/ Instruction{encoding:Type::CB,mnemonic:"BIT4_A",cycles:8,length:2},
/*0x68*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_B",cycles:8,length:2},
/*0x69*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_C",cycles:8,length:2},
/*0x6a*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_D",cycles:8,length:2},
/*0x6b*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_E",cycles:8,length:2},
/*0x6c*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_H",cycles:8,length:2},
/*0x6d*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_L",cycles:8,length:2},
/*0x6e*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_HL",cycles:16,length:2},
/*0x6f*/ Instruction{encoding:Type::CB,mnemonic:"BIT5_A",cycles:8,length:2},
/*0x70*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_B",cycles:8,length:2},
/*0x71*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_C",cycles:8,length:2},
/*0x72*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_D",cycles:8,length:2},
/*0x73*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_E",cycles:8,length:2},
/*0x74*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_H",cycles:8,length:2},
/*0x75*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_L",cycles:8,length:2},
/*0x76*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_HL",cycles:16,length:2},
/*0x77*/ Instruction{encoding:Type::CB,mnemonic:"BIT6_A",cycles:8,length:2},
/*0x78*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_B",cycles:8,length:2},
/*0x79*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_C",cycles:8,length:2},
/*0x7a*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_D",cycles:8,length:2},
/*0x7b*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_E",cycles:8,length:2},
/*0x7c*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_H",cycles:8,length:2},
/*0x7d*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_L",cycles:8,length:2},
/*0x7e*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_HL",cycles:16,length:2},
/*0x7f*/ Instruction{encoding:Type::CB,mnemonic:"BIT7_A",cycles:8,length:2},
/*0x80*/ Instruction{encoding:Type::CB,mnemonic:"RES0_B",cycles:8,length:2},
/*0x81*/ Instruction{encoding:Type::CB,mnemonic:"RES0_C",cycles:8,length:2},
/*0x82*/ Instruction{encoding:Type::CB,mnemonic:"RES0_D",cycles:8,length:2},
/*0x83*/ Instruction{encoding:Type::CB,mnemonic:"RES0_E",cycles:8,length:2},
/*0x84*/ Instruction{encoding:Type::CB,mnemonic:"RES0_H",cycles:8,length:2},
/*0x85*/ Instruction{encoding:Type::CB,mnemonic:"RES0_L",cycles:8,length:2},
/*0x86*/ Instruction{encoding:Type::CB,mnemonic:"RES0_HL",cycles:16,length:2},
/*0x87*/ Instruction{encoding:Type::CB,mnemonic:"RES0_A",cycles:8,length:2},
/*0x88*/ Instruction{encoding:Type::CB,mnemonic:"RES1_B",cycles:8,length:2},
/*0x89*/ Instruction{encoding:Type::CB,mnemonic:"RES1_C",cycles:8,length:2},
/*0x8a*/ Instruction{encoding:Type::CB,mnemonic:"RES1_D",cycles:8,length:2},
/*0x8b*/ Instruction{encoding:Type::CB,mnemonic:"RES1_E",cycles:8,length:2},
/*0x8c*/ Instruction{encoding:Type::CB,mnemonic:"RES1_H",cycles:8,length:2},
/*0x8d*/ Instruction{encoding:Type::CB,mnemonic:"RES1_L",cycles:8,length:2},
/*0x8e*/ Instruction{encoding:Type::CB,mnemonic:"RES1_HL",cycles:16,length:2},
/*0x8f*/ Instruction{encoding:Type::CB,mnemonic:"RES1_A",cycles:8,length:2},
/*0x90*/ Instruction{encoding:Type::CB,mnemonic:"RES2_B",cycles:8,length:2},
/*0x91*/ Instruction{encoding:Type::CB,mnemonic:"RES2_C",cycles:8,length:2},
/*0x92*/ Instruction{encoding:Type::CB,mnemonic:"RES2_D",cycles:8,length:2},
/*0x93*/ Instruction{encoding:Type::CB,mnemonic:"RES2_E",cycles:8,length:2},
/*0x94*/ Instruction{encoding:Type::CB,mnemonic:"RES2_H",cycles:8,length:2},
/*0x95*/ Instruction{encoding:Type::CB,mnemonic:"RES2_L",cycles:8,length:2},
/*0x96*/ Instruction{encoding:Type::CB,mnemonic:"RES2_HL",cycles:16,length:2},
/*0x97*/ Instruction{encoding:Type::CB,mnemonic:"RES2_A",cycles:8,length:2},
/*0x98*/ Instruction{encoding:Type::CB,mnemonic:"RES3_B",cycles:8,length:2},
/*0x99*/ Instruction{encoding:Type::CB,mnemonic:"RES3_C",cycles:8,length:2},
/*0x9a*/ Instruction{encoding:Type::CB,mnemonic:"RES3_D",cycles:8,length:2},
/*0x9b*/ Instruction{encoding:Type::CB,mnemonic:"RES3_E",cycles:8,length:2},
/*0x9c*/ Instruction{encoding:Type::CB,mnemonic:"RES3_H",cycles:8,length:2},
/*0x9d*/ Instruction{encoding:Type::CB,mnemonic:"RES3_L",cycles:8,length:2},
/*0x9e*/ Instruction{encoding:Type::CB,mnemonic:"RES3_HL",cycles:16,length:2},
/*0x9f*/ Instruction{encoding:Type::CB,mnemonic:"RES3_A",cycles:8,length:2},
/*0xa0*/ Instruction{encoding:Type::CB,mnemonic:"RES4_B",cycles:8,length:2},
/*0xa1*/ Instruction{encoding:Type::CB,mnemonic:"RES4_C",cycles:8,length:2},
/*0xa2*/ Instruction{encoding:Type::CB,mnemonic:"RES4_D",cycles:8,length:2},
/*0xa3*/ Instruction{encoding:Type::CB,mnemonic:"RES4_E",cycles:8,length:2},
/*0xa4*/ Instruction{encoding:Type::CB,mnemonic:"RES4_H",cycles:8,length:2},
/*0xa5*/ Instruction{encoding:Type::CB,mnemonic:"RES4_L",cycles:8,length:2},
/*0xa6*/ Instruction{encoding:Type::CB,mnemonic:"RES4_HL",cycles:16,length:2},
/*0xa7*/ Instruction{encoding:Type::CB,mnemonic:"RES4_A",cycles:8,length:2},
/*0xa8*/ Instruction{encoding:Type::CB,mnemonic:"RES5_B",cycles:8,length:2},
/*0xa9*/ Instruction{encoding:Type::CB,mnemonic:"RES5_C",cycles:8,length:2},
/*0xaa*/ Instruction{encoding:Type::CB,mnemonic:"RES5_D",cycles:8,length:2},
/*0xab*/ Instruction{encoding:Type::CB,mnemonic:"RES5_E",cycles:8,length:2},
/*0xac*/ Instruction{encoding:Type::CB,mnemonic:"RES5_H",cycles:8,length:2},
/*0xad*/ Instruction{encoding:Type::CB,mnemonic:"RES5_L",cycles:8,length:2},
/*0xae*/ Instruction{encoding:Type::CB,mnemonic:"RES5_HL",cycles:16,length:2},
/*0xaf*/ Instruction{encoding:Type::CB,mnemonic:"RES5_A",cycles:8,length:2},
/*0xb0*/ Instruction{encoding:Type::CB,mnemonic:"RES6_B",cycles:8,length:2},
/*0xb1*/ Instruction{encoding:Type::CB,mnemonic:"RES6_C",cycles:8,length:2},
/*0xb2*/ Instruction{encoding:Type::CB,mnemonic:"RES6_D",cycles:8,length:2},
/*0xb3*/ Instruction{encoding:Type::CB,mnemonic:"RES6_E",cycles:8,length:2},
/*0xb4*/ Instruction{encoding:Type::CB,mnemonic:"RES6_H",cycles:8,length:2},
/*0xb5*/ Instruction{encoding:Type::CB,mnemonic:"RES6_L",cycles:8,length:2},
/*0xb6*/ Instruction{encoding:Type::CB,mnemonic:"RES6_HL",cycles:16,length:2},
/*0xb7*/ Instruction{encoding:Type::CB,mnemonic:"RES6_A",cycles:8,length:2},
/*0xb8*/ Instruction{encoding:Type::CB,mnemonic:"RES7_B",cycles:8,length:2},
/*0xb9*/ Instruction{encoding:Type::CB,mnemonic:"RES7_C",cycles:8,length:2},
/*0xba*/ Instruction{encoding:Type::CB,mnemonic:"RES7_D",cycles:8,length:2},
/*0xbb*/ Instruction{encoding:Type::CB,mnemonic:"RES7_E",cycles:8,length:2},
/*0xbc*/ Instruction{encoding:Type::CB,mnemonic:"RES7_H",cycles:8,length:2},
/*0xbd*/ Instruction{encoding:Type::CB,mnemonic:"RES7_L",cycles:8,length:2},
/*0xbe*/ Instruction{encoding:Type::CB,mnemonic:"RES7_HL",cycles:16,length:2},
/*0xbf*/ Instruction{encoding:Type::CB,mnemonic:"RES7_A",cycles:8,length:2},
/*0xc0*/ Instruction{encoding:Type::CB,mnemonic:"SET0_B",cycles:8,length:2},
/*0xc1*/ Instruction{encoding:Type::CB,mnemonic:"SET0_C",cycles:8,length:2},
/*0xc2*/ Instruction{encoding:Type::CB,mnemonic:"SET0_D",cycles:8,length:2},
/*0xc3*/ Instruction{encoding:Type::CB,mnemonic:"SET0_E",cycles:8,length:2},
/*0xc4*/ Instruction{encoding:Type::CB,mnemonic:"SET0_H",cycles:8,length:2},
/*0xc5*/ Instruction{encoding:Type::CB,mnemonic:"SET0_L",cycles:8,length:2},
/*0xc6*/ Instruction{encoding:Type::CB,mnemonic:"SET0_HL",cycles:16,length:2},
/*0xc7*/ Instruction{encoding:Type::CB,mnemonic:"SET0_A",cycles:8,length:2},
/*0xc8*/ Instruction{encoding:Type::CB,mnemonic:"SET1_B",cycles:8,length:2},
/*0xc9*/ Instruction{encoding:Type::CB,mnemonic:"SET1_C",cycles:8,length:2},
/*0xca*/ Instruction{encoding:Type::CB,mnemonic:"SET1_D",cycles:8,length:2},
/*0xcb*/ Instruction{encoding:Type::CB,mnemonic:"SET1_E",cycles:8,length:2},
/*0xcc*/ Instruction{encoding:Type::CB,mnemonic:"SET1_H",cycles:8,length:2},
/*0xcd*/ Instruction{encoding:Type::CB,mnemonic:"SET1_L",cycles:8,length:2},
/*0xce*/ Instruction{encoding:Type::CB,mnemonic:"SET1_HL",cycles:16,length:2},
/*0xcf*/ Instruction{encoding:Type::CB,mnemonic:"SET1_A",cycles:8,length:2},
/*0xd0*/ Instruction{encoding:Type::CB,mnemonic:"SET2_B",cycles:8,length:2},
/*0xd1*/ Instruction{encoding:Type::CB,mnemonic:"SET2_C",cycles:8,length:2},
/*0xd2*/ Instruction{encoding:Type::CB,mnemonic:"SET2_D",cycles:8,length:2},
/*0xd3*/ Instruction{encoding:Type::CB,mnemonic:"SET2_E",cycles:8,length:2},
/*0xd4*/ Instruction{encoding:Type::CB,mnemonic:"SET2_H",cycles:8,length:2},
/*0xd5*/ Instruction{encoding:Type::CB,mnemonic:"SET2_L",cycles:8,length:2},
/*0xd6*/ Instruction{encoding:Type::CB,mnemonic:"SET2_HL",cycles:16,length:2},
/*0xd7*/ Instruction{encoding:Type::CB,mnemonic:"SET2_A",cycles:8,length:2},
/*0xd8*/ Instruction{encoding:Type::CB,mnemonic:"SET3_B",cycles:8,length:2},
/*0xd9*/ Instruction{encoding:Type::CB,mnemonic:"SET3_C",cycles:8,length:2},
/*0xda*/ Instruction{encoding:Type::CB,mnemonic:"SET3_D",cycles:8,length:2},
/*0xdb*/ Instruction{encoding:Type::CB,mnemonic:"SET3_E",cycles:8,length:2},
/*0xdc*/ Instruction{encoding:Type::CB,mnemonic:"SET3_H",cycles:8,length:2},
/*0xdd*/ Instruction{encoding:Type::CB,mnemonic:"SET3_L",cycles:8,length:2},
/*0xde*/ Instruction{encoding:Type::CB,mnemonic:"SET3_HL",cycles:16,length:2},
/*0xdf*/ Instruction{encoding:Type::CB,mnemonic:"SET3_A",cycles:8,length:2},
/*0xe0*/ Instruction{encoding:Type::CB,mnemonic:"SET4_B",cycles:8,length:2},
/*0xe1*/ Instruction{encoding:Type::CB,mnemonic:"SET4_C",cycles:8,length:2},
/*0xe2*/ Instruction{encoding:Type::CB,mnemonic:"SET4_D",cycles:8,length:2},
/*0xe3*/ Instruction{encoding:Type::CB,mnemonic:"SET4_E",cycles:8,length:2},
/*0xe4*/ Instruction{encoding:Type::CB,mnemonic:"SET4_H",cycles:8,length:2},
/*0xe5*/ Instruction{encoding:Type::CB,mnemonic:"SET4_L",cycles:8,length:2},
/*0xe6*/ Instruction{encoding:Type::CB,mnemonic:"SET4_HL",cycles:16,length:2},
/*0xe7*/ Instruction{encoding:Type::CB,mnemonic:"SET4_A",cycles:8,length:2},
/*0xe8*/ Instruction{encoding:Type::CB,mnemonic:"SET5_B",cycles:8,length:2},
/*0xe9*/ Instruction{encoding:Type::CB,mnemonic:"SET5_C",cycles:8,length:2},
/*0xea*/ Instruction{encoding:Type::CB,mnemonic:"SET5_D",cycles:8,length:2},
/*0xeb*/ Instruction{encoding:Type::CB,mnemonic:"SET5_E",cycles:8,length:2},
/*0xec*/ Instruction{encoding:Type::CB,mnemonic:"SET5_H",cycles:8,length:2},
/*0xed*/ Instruction{encoding:Type::CB,mnemonic:"SET5_L",cycles:8,length:2},
/*0xee*/ Instruction{encoding:Type::CB,mnemonic:"SET5_HL",cycles:16,length:2},
/*0xef*/ Instruction{encoding:Type::CB,mnemonic:"SET5_A",cycles:8,length:2},
/*0xf0*/ Instruction{encoding:Type::CB,mnemonic:"SET6_B",cycles:8,length:2},
/*0xf1*/ Instruction{encoding:Type::CB,mnemonic:"SET6_C",cycles:8,length:2},
/*0xf2*/ Instruction{encoding:Type::CB,mnemonic:"SET6_D",cycles:8,length:2},
/*0xf3*/ Instruction{encoding:Type::CB,mnemonic:"SET6_E",cycles:8,length:2},
/*0xf4*/ Instruction{encoding:Type::CB,mnemonic:"SET6_H",cycles:8,length:2},
/*0xf5*/ Instruction{encoding:Type::CB,mnemonic:"SET6_L",cycles:8,length:2},
/*0xf6*/ Instruction{encoding:Type::CB,mnemonic:"SET6_HL",cycles:16,length:2},
/*0xf7*/ Instruction{encoding:Type::CB,mnemonic:"SET6_A",cycles:8,length:2},
/*0xf8*/ Instruction{encoding:Type::CB,mnemonic:"SET7_B",cycles:8,length:2},
/*0xf9*/ Instruction{encoding:Type::CB,mnemonic:"SET7_C",cycles:8,length:2},
/*0xfa*/ Instruction{encoding:Type::CB,mnemonic:"SET7_D",cycles:8,length:2},
/*0xfb*/ Instruction{encoding:Type::CB,mnemonic:"SET7_E",cycles:8,length:2},
/*0xfc*/ Instruction{encoding:Type::CB,mnemonic:"SET7_H",cycles:8,length:2},
/*0xfd*/ Instruction{encoding:Type::CB,mnemonic:"SET7_L",cycles:8,length:2},
/*0xfe*/ Instruction{encoding:Type::CB,mnemonic:"SET7_HL",cycles:16,length:2},
/*0xff*/ Instruction{encoding:Type::CB,mnemonic:"SET7_A",cycles:8,length:2}
];
