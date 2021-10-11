mod cpu;
mod op;
mod mem;

// Const representing size of memory, stored as unsigned int
// // of pointer-size (dependant on computer architecture)
// const MEMORY_SIZE: usize = 4096;

pub struct CHIP8 {}

// impl creates methods for a given struct
impl CHIP8 {

    //implement cycle, maybe let's loop it here as well
    pub fn run(&mut self){

        let mut cpu = cpu::CPU::new();
        let mut mem = mem::Memory::new();

        mem.load_rom("/Users/keshav/chip8/c8games/PONG");
        
        loop{
            //need to pass in some kind of memory
            cpu.cycle(&mut mem);
        }
    }
}