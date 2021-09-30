enum Op {
    LD,
    ADD,
    LDR
}

impl Op {
    pub fn decode(opcode: u16) -> Self {
        match opcode & 0xF000 {
            0x6000 => Self::LD,
            0x7000 => Self::ADD,
            0x8000 => Self::LDR,
            _ => panic!("Invalid opcode: {:04X}", opcode),
        }
    }

    pub fn x(opcode: u16) -> usize {
        ((opcode & 0x0F00) >> 8) as usize
    }

    pub fn y(opcode: u16) -> usize {
        ((opcode & 0x00F0) >> 4) as usize
    }

    pub fn kk(opcode: u16) -> u8 {
        (opcode & 0x00FF) as u8
    }
}
