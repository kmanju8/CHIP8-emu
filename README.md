## CHIP-8 emulator project

The purpose of this project is for me to break into understanding system emulation, with CHIP-8 well known as a simple system to start with.

Through this project, the main things I am trying to learn are:
- Rust
- low level system interfacting
- general systems architecture.

I used two resources for this project:
- https://blog.wjdevschool.com/blog/video-game-console-emulator/ kicks off with the basic skeleton of the project in Rust, thus extremely helpful for picking up a decent coding style in the language.
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#dispcoords is a technical reference guide for the CHIP-8. More or less necessary, lest the project focus shift to reverse engineering.

Everything else was researched primarily looking at documentation.


### Rust
Prior to this project, I was relatively unfamiliar with using Rust, asides reading some of the documentation and tutorials.

### Low level system interfacting
Used SDL2 with the inclusion of a Rust wrapper. SDL contains OpenGL as well as other low level accesses to hardware. More of a means to an end than to deep dive - especially as it is hardly the tidiest library to deal with. But also gave nice exposure to using C libraries in Rust, and some light exposure to unsafe Rust and error handling.
In general I was relatively unfamiliar with connecting my programs with other aspects of a computer such as triggered inputs, sound output etc. so this was a dive into this, as well as making use of a more complicated library which is not overly opinionated.

### General systems architecture
The project idea fundamentally stems from wondering how emulation is performed - and this is essentially just implementing the Fetch-Decode-Execute cycle. Interesting to implement some typical instructions for this type of low level programming, as well as enforce safety measures needed due to working in a higher level language.