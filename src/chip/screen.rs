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

    // will implement drawing of sprites to coords here
    // implement collision detection in here as well I guess
    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: usize, y: usize, sprite: Vec<u8>){

        let mut height: usize = 0;
        for row in sprite {

            self.screen[height] = self.screen[height]^((row as u64) << (56 - x));

            height += 1;
        }

        // XOR is ^
        // self.screen[]^sprite
        // will need to throw in modulo for screen wrap
        // need to make sure data at array addresses is changed

        //at x,y, draw sprite. sprite can be array of 5 numbers.

    }

    
}
