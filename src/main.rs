mod chip8;
use chip8::window::Window;
use chip8::font;
use chip8::rom;
use chip8::Chip8;

fn main() {
    let mut window = Window::new("Chip8");
    let mut chip8 = Chip8::new();
    let mut rom = rom::Rom::load("flags.ch8");
    chip8.memory[font::FONT_START..font::FONT_START + font::FONT.len()].copy_from_slice(&font::FONT);
    chip8.memory[rom::ROM_START..rom::ROM_START + rom.data.len()].copy_from_slice(&rom.data);

    while window.is_open() {
        window.clear(0);

        chip8.step(&mut window);

        let buffer: Vec<u32> = chip8
            .display_buffer
            .iter()
            .map(|&pixel| {
                if pixel == 0 {
                    0x00000000 // black
                } else {
                    0xFFFFFFFF // white
                }
            })
            .collect();

        window.buffer = buffer;
        window.update();
    }
}
