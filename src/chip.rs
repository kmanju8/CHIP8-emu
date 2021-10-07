use std::io;
use std::io::Read;
use std::fs::File;

mod cpu;
mod op;

// Const representing size of memory, stored as unsigned int
// of pointer-size (dependant on computer architecture)
const MEMORY_SIZE: usize = 4096;

pub struct CHIP8 {
    // create 4kB of memory
    mem: [u8; MEMORY_SIZE]
}

// impl creates methods for a given struct
impl CHIP8 {
    
    // resets memory to blank I think?
    pub fn new() -> Self {
        Self {
            mem:[0; MEMORY_SIZE]
        }
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
            self.mem[0x200 + i] = rom_data[i]
        }

        Ok(())
    }

    // TODO
    // implement run method.
}