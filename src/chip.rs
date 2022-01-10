extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut i = 0;
        
        'running: loop {

            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            // allows quiting with escape
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            //need to pass in some kind of memory
            cpu.cycle(&mut mem);

            //refresh screen?
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}