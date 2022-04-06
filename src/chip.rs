extern crate sdl2; 

mod cpu;
mod op;
mod mem;
mod screen;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use std::time::Duration;

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
        let mut screen = screen::Screen::new();
        mem.load_rom("/Users/keshav/chip8/c8games/PONG");

        // for the time being, will not handle errors. Will do after actual implementation.
        let scale_factor = 15;
        // let fps = 30;
        
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo", 64*scale_factor, 32*scale_factor)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_logical_size(64,32);
    
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            cpu.cycle(&mut mem, &mut canvas, &mut screen);

            // canvas.set_draw_color(Color::WHITE);
            // for i in 0.. 10 {
            //     canvas.draw_point(Point::new(i,i));
            // }
            // canvas.present();
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }
}