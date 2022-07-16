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

macro_rules! write_indirect {
    ($bus: expr, $high: expr, $low: expr, $value: expr) => {{
        $bus.write(($high as u16) << 8 | $low as u16, $value)
    }}
}

macro_rules! get_reg {
    ($cpu: expr, $bus: expr, $op: expr) => {{
        match $op % 0x8 {
            0x0 => $cpu.b,
            0x1 => $cpu.c,
            0x2 => $cpu.d,
            0x3 => $cpu.e,
            0x4 => $cpu.h,
            0x5 => $cpu.l,
            0x6 => $bus.read(($cpu.h as u16) << 8 | $cpu.l as u16),
            0x7 => $cpu.a,
            _   => 0
        }
    }}
}

macro_rules! assert_state {
    ($cpu: expr, $reg: expr, $state: expr) => {{
        assert!($reg == $state.reg_p);
        assert!($cpu.cf == $state.cf);
        assert!($cpu.hf == $state.hf);
        assert!($cpu.nf == $state.nf);
        assert!($cpu.zf == $state.zf);
    }}
}

macro_rules! set_reg_state {
    ($cpu: expr, $bus: expr, $state: expr) => {{
        $cpu.b = $state[0].reg;
        $cpu.c = $state[1].reg;
        $cpu.d = $state[2].reg;
        $cpu.e = $state[3].reg;
        $cpu.h = $state[4].reg;
        $cpu.l = $state[5].reg;
        write_indirect!($bus, $state[4].reg_p, $state[5].reg_p, $state[6].reg);
        $cpu.a = $state[7].reg;
    }}
}

macro_rules! check_reg_state {
    ($cpu: expr, $bus: expr, $state: expr) => {{
        loop {
            let opcode = $cpu.fetch_opcode(&$bus);
            if opcode == 0 { break; }
            $cpu.execute(&mut $bus, opcode);
            assert_state!($cpu, get_reg!($cpu, $bus, opcode), 
                $state[(opcode % 0x08) as usize]);
        }
    }}
}

macro_rules! run_test {
    ($cpu:expr, $bus: expr, $state: expr) => {{
        set_reg_state!($cpu, $bus, $state);
        check_reg_state!($cpu, $bus, $state);
    }}
}

struct State {
    reg: u8,
    reg_p: u8,
    zf: u8,
    nf: u8,
    hf: u8,
    cf: u8
}

#[test]
fn test_cb_rlc() {
    let mut cpu = Sharp8080::new(0x0000);
    let mut bus = BusTest::new();
    for i in 0x00..0x07 {
        bus.write(2 * i as u16, 0xCB);
        bus.write(((2 * i) + 1) as u16, i);
    }
    const STATE : [State; 8] = [ 
    { State { reg: 0xC0, reg_p: 0xC0 << 1 | 0x1, zf: 0, nf: 0, hf: 0, cf: 1 }},
    { State { reg: 0x20, reg_p: 0x20 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x10, reg_p: 0x10 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x08, reg_p: 0x08 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x04, reg_p: 0x04 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x02, reg_p: 0x02 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x01, reg_p: 0x01 << 1,       zf: 0, nf: 0, hf: 0, cf: 0 }},
    { State { reg: 0x00, reg_p: 0x00,            zf: 1, nf: 0, hf: 0, cf: 0 }}];
    run_test!(cpu, bus, STATE);
}
