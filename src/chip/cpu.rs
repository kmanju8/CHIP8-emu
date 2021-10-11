use super::op::Op;
use super::mem::Memory;

pub struct CPU {
    v: [u8; 16], // the V registers
    i: u16,      // the I register
    pc: u16,     // the program counter
    sp: u8,      // the stack pointer
    dt: u8,      // delay timer
    st: u8,      // sound timer
}

impl CPU {
    pub fn new() -> Self {
        // initializes values for the various counters
        Self {
            v: [0; 16],
            i: 0,
            // Set program counter to the beginning of the ROM
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory) {
        // read opcode in program counter
        let opcode = memory.read(self.pc);
        println!("{}", opcode);
        // increment program counter
        self.pc += 2;
        // decode instruction; probably bulk of work?
        let op = Op::decode(opcode);

        // some parameters pulled out for readability; these correspond to the instructional hex
        let x = Op::x(opcode);
        let y = Op::y(opcode);
        let kk = Op::kk(opcode);

        // executes instruction
        match op {
            Op::LD => self.v[x] = kk,
            Op::ADD => self.v[x] = self.v[x] + kk,
            Op::LDR => self.v[x] = self.v[y],
        }
    }
}
