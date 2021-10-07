mod chip;

fn main() {
    println!("Hello, world!");

    let mut memory = chip::CHIP8::new();

    memory.load_rom("/Users/keshav/chip8/c8games/PONG");


    
}
