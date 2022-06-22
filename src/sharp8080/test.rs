use super::*;

pub struct BusTest {
   memory : [u8; 0x10000]
}

impl BusTrait for BusTest {
    fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn read(&self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
}

impl BusTest {
    fn new() -> BusTest {
        BusTest { memory: [0; 0x10000] }
    }
}

#[test]
fn test_cb_rlc() {
    let mut cpu = Sharp8080::new(0x0000);
    let mut bus = BusTest::new();
    for i in 0x00..0x07 {
        bus.write(2 * i as u16, 0xCB);
        bus.write(((2 * i) + 1) as u16, i);
    }
    cpu.b = 0b11000000;
    cpu.c = 0b00100000;
    cpu.d = 0b00010000;
    cpu.e = 0b00001000;
    cpu.h = 0b00000100;
    cpu.l = 0b00000010;
    cpu.a = 0b00000001;
    loop {
        let opcode = cpu.fetch_opcode(&bus);
        cpu.execute(&mut bus, opcode);
        if opcode == 0 { break; }
    }
    assert!(cpu.b == 0b10000001);
    assert!(cpu.c == 0b01000000);
    assert!(cpu.d == 0b00100000);
    assert!(cpu.e == 0b00010000);
    assert!(cpu.h == 0b00001000);
    assert!(cpu.l == 0b00000100);
    assert!(cpu.a == 0b00000010);   
}
