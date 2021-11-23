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

        // XOR is ^
        self.screen[]^sprite
        // will need to throw in modulo for screen wrap
        // need to make sure data at array addresses is changed

        //at x,y, draw sprite. sprite can be array of 5 numbers.

    }

    // method to pull sprite out of memory. should this be here?
    pub fn getsprite(i: u8, n: u8){
        //i is starting mem location of sprite, n is no of bytes long

    }
}
asdfada