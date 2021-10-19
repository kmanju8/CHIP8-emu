// might keep track of pixels on screen? 64x32
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
    pub fn draw(x: u8, y: u8, sprite: [u8;5]){

        //at x,y, draw sprite. sprite can be array of 5 numbers.

    }

    // method to pull sprite out of memory. should this be here?
    pub fn getsprite(){
        
    }
}
