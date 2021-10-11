use std::io;
use std::io::Read;
use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};


const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE]
}

impl Memory {

    pub fn new() -> Self {
        let mut bytes = [0; MEMORY_SIZE];

        Self { bytes }
    }

    pub fn load_rom(&mut self, path: &str) -> io::Result<()> {

        // tries to open rom
        let mut rom_file = File::open(path)?;
        // creates vector to store rom data
        let mut rom_data = Vec::new();

        // finds size of rom; but I think it also drops
        // the rom data stored in rom_file into rom_data.
        // rom_data is the buffer here, in which 'self' is loaded to
        let rom_size = rom_file.read_to_end(&mut rom_data)?;

        // loads rom into memory from 0x200
        for i in 0.. rom_size {
            self.bytes[0x200 + i] = rom_data[i]
        }

        Ok(())
    }

    //should read opcode PC is currently pointing to.
    pub fn read(&mut self, pc: u16) -> u16 {

        return 1 as u16;
    }
}