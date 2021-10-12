use super::op::Op;
use super::mem::Memory;

pub struct CPU {
    v: [u8; 16], // the V registers
    i: u16,      // the I register
    pc: u16,     // the program counter
    sp: u8,      // the stack pointer
    dt: u8,      // delay timer
    st: u8,      // sound timer

    stack: [u16; 16], // the stack
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
            stack: [0; 16],
        }
    }

    pub fn cycle(&mut self, memory: &mut Memory) {
        // read opcode in program counter
        let opcode = memory.read(self.pc);
        // increment program counter
        self.pc += 2;
        // decode instruction; probably bulk of work?
        let op = Op::decode(opcode);

        // some parameters pulled out for readability; these correspond to the instructional hex
        let x = Op::x(opcode);
        let y = Op::y(opcode);
        let kk = Op::kk(opcode);
        let nnn = Op::nnn(opcode);

        // executes instruction
        match op {

            Op::CLS => {}, //clear display
            Op::RET => {
                self.pc=self.stack[(self.st-1) as usize];
                self.st-=1;                
            },
            Op::SYS => {},


            Op::JP => self.pc = nnn,
            Op::CALL => {
                self.st+=1;
                self.stack[(self.st-1) as usize]=self.pc;
                self.pc=nnn;
            },
            Op::SE => if self.v[x]==kk {self.pc+=2},
            Op::SNE => if self.v[x]!=kk {self.pc+=2},
            Op::SE_Y => if self.v[x]==self.v[y] {self.pc+=2},
            Op::LD => self.v[x] = kk,
            Op::ADD => self.v[x] = self.v[x] + kk,
            Op::LDR => self.v[x] = self.v[y],
        }
    }
}
