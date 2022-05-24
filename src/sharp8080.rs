use crate::Bus;

macro_rules! cb_helper {
        ($self: expr, $opcode: expr, $op: tt, $lit: literal) => {
            {
                match $opcode % 0x8 {
                    0x0 => $self.b $op $lit,
                    0x1 => $self.c $op $lit,
                    0x2 => $self.d $op $lit,
                    0x3 => $self.e $op $lit,
                    0x4 => $self.h $op $lit,
                    0x5 => $self.l $op $lit,
                    0x6 => $self.l $op $lit,
                    0x7 => $self.a $op $lit,
                    _   => () 
                }
            }
        }
}
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
}

impl Sharp8080 {
    pub fn new() -> Sharp8080 {
        Sharp8080 { a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, sp: 0, pc: 0x0100 }
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
            (&INSTRUCTION_TABLE[((opcode & 0x00FF) as usize) * 2], self.pc+2)
        } else {
            (&INSTRUCTION_TABLE[(opcode & 0x00FF) as usize], self.pc+1)
        };
        match instruction.encoding {
            Type::N => {
                match opcode {
                    0x0000 => (),
                    0x0002 => self.ld_bc_a(),
                    0x0003 => self.inc_bc(),
                    _      => self.undefined_instruction(),
                }
            }
            Type::D16 => {
                let b0 = bus.read(pc_base);
                let b1 = bus.read(pc_base+1);
                match opcode {
                    0x0001 => self.ld_bc_d16(b0, b1),
                    _      => self.undefined_instruction(), 
                }
            }
            Type::CB => {
                match opcode {
                    // Set bit 0.
                    0xCBC0..=0xCBC7 => cb_helper!(self, opcode, |=, 0b00000001),
                    // Set bit 1
                    0xCBC8..=0xCBCF => cb_helper!(self, opcode, |=, 0b00000010),
                    _      => ()
                }
                self.pc += instruction.length;
            }
            _ => {
                self.undefined_type();
            }
        }
        self.decode(instruction);
        self.wait(instruction.cycles);
    }

    fn decode(&self, instruction: &Instruction) {
        println!("Instruction {:?}\n", instruction);
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
/* 0x50 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x51 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x52 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x53 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x54 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x55 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x56 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
/* 0x57 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
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
/* 0xc3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
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
/* 0xf3 */ Instruction{encoding:Type::N, mnemonic:"NOP",cycles:4,length:1},
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