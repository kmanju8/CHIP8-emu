pub enum Op {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE,
    SNE,
    SE_Y,
    LD,
    ADD,
    LDR
}


impl Op {
    pub fn decode(opcode: u16) -> Self {
        match opcode & 0xF000 {
            // didn't match very well, need to recap
            0x0000 => match opcode & 0x00FF{
                0x00EE => Self::RET,
                0x00E0 => Self::CLS,
                _ => Self::SYS,
            },
            0x1000 => Self::JP,
            0x2000 => Self::CALL,
            0x3000 => Self::SE,
            0x4000 => Self::SNE,
            0x5000 => Self::SE_Y,
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

    pub fn nnn(opcode: u16) -> u16 {
        (opcode & 0x0FFF) as u16
    }
}
