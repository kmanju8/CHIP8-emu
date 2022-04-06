mod chip;

static FPS: u32 = 30;

fn main() {

    let mut system = chip::CHIP8{};

    system.run();
}

