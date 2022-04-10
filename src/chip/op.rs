pub enum Op {
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE,
    SNE,
    #[allow(non_camel_case_types)]
    SE_Y,
    LD,
    ADD,
    LDR,
    #[allow(non_camel_case_types)]
    LD_I,
    DRW,
    #[allow(non_camel_case_types)]
    LD_VDT,
    #[allow(non_camel_case_types)]
    LD_K,
    #[allow(non_camel_case_types)]
    LD_DTV,
    #[allow(non_camel_case_types)]
    LD_ST,
    #[allow(non_camel_case_types)]
    ADD_I,
    #[allow(non_camel_case_types)]
    LD_F,
    #[allow(non_camel_case_types)]
    LD_B,
    #[allow(non_camel_case_types)]
    LD_IV,
    #[allow(non_camel_case_types)]
    LD_VI, 
}


impl Op {
    pub fn decode(opcode: u16) -> Self {
        // if opcode ==  0x121A {
        //     panic!("Invalid opcode: {:04X}", opcode);
        // }
        println!("{:04X}",opcode);
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

            0xA000 => Self::LD_I,

            0xD000 => Self::DRW,
            
            0xF000 => match opcode & 0x00FF{
                0x0007 => Self::LD_VDT,
                0x000A => Self::LD_K,
                0x0015 => Self::LD_DTV,
                0x0018 => Self::LD_ST,
                0x001E => Self::ADD_I,
                0x0029 => Self::LD_F,
                0x0033 => Self::LD_B,
                0x0055 => Self::LD_IV,
                0x0065 => Self::LD_VI,
                _ => panic!("Invalid opcode: {:04X}", opcode),             
            },

            _ => panic!("Invalid opcode: {:04X}", opcode),
        }
    }

    pub fn x(opcode: u16) -> usize {
        ((opcode & 0x0F00) >> 8) as usize
    }

    pub fn y(opcode: u16) -> usize {
        ((opcode & 0x00F0) >> 4) as usize
    }

    pub fn n(opcode: u16) -> usize {
        ((opcode & 0x000F) >> 4) as usize
    }

    pub fn kk(opcode: u16) -> u8 {
        (opcode & 0x00FF) as u8
    }

    pub fn nnn(opcode: u16) -> u16 {
        (opcode & 0x0FFF) as u16
    }
}
