use std::time::Duration;

use bitvec::prelude::*;
use sdl2::rect::Point;

static FPS: u32 = 30;
// might keep track of pixels on screen? 64x32
// if this can be kept persistant that'd be nice. But may make sense to move one layer
// higher into cpu.rs or something.
pub struct Screen{
    screen: [u64;32]
}

impl Screen{
    pub fn new() -> Self {
        Self{
            screen: [0;32],
        }
    }

    // TODO: Still have to account for screen wrap
    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: usize, y: usize, sprite: Vec<u8>){

        for (height,row) in sprite.into_iter().enumerate() {

            self.screen[y+height] ^= (row as u64) << (56 - x);

            let mut row: u64 = self.screen[y+height];
            let row_height: i32 = (y + height) as i32;
            
            // will have screen clears in between frames, so okay to draw only ON pixels and ignore OFF
            for (i,lit) in row.view_bits_mut::<Msb0>().into_iter().enumerate() {
                if *lit {
                    match canvas.draw_point(Point::new((x + i) as i32,row_height)) {
                        Ok(()) => (),
                        Err(e) => println!("Failed to draw point to cavas: {}", e)
                    }
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));


        // canvas.set_draw_color(Color::WHITE);
            // for i in 0.. 10 {
            //     canvas.draw_point(Point::new(i,i));
            // }
            // canvas.present();
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));

        // --------------------------Bit above shows drawing some pixels to screen
        // should I use textures instead? 

        // XOR is ^
        // self.screen[]^sprite
        // will need to throw in modulo for screen wrap
        // need to make sure data at array addresses is changed

        //at x,y, draw sprite. sprite can be array of 5 numbers.

    }

    
}
