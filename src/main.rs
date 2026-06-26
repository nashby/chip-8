mod chip8;
use chip8::window::Window;
use chip8::font;
use chip8::rom;
use chip8::Chip8;
use std::collections::HashMap;
use minifb::Key;

fn main() {
    let mut window = Window::new("Chip8");
    let mut chip8 = Chip8::new();
    let rom = rom::Rom::load("flags.ch8");
    chip8.memory[font::FONT_START..font::FONT_START + font::FONT.len()].copy_from_slice(&font::FONT);
    chip8.memory[rom::ROM_START..rom::ROM_START + rom.data.len()].copy_from_slice(&rom.data);

    let keys_mapping = HashMap::from([
        (Key::Key0, 0),
        (Key::Key1, 1),
        (Key::Key2, 2),
        (Key::Key3, 3),
        (Key::Key4, 4),
        (Key::Key5, 5),
        (Key::Key6, 6),
        (Key::Key7, 7),
        (Key::Key8, 8),
        (Key::Key9, 9)
    ]);

    while window.is_open() {
        window.clear(0);

        for &key in window.get_keys().iter() {
            chip8.keypad[keys_mapping[&key]] = true;
        }

        for _ in 0..chip8::INSTRUCTIONS_PER_FRAME {
            chip8.step();
        };

        chip8.tick_timers();

        let buffer: Vec<u32> = chip8
            .display_buffer
            .iter()
            .map(|&pixel| {
                if pixel == 0 { 0x00000000 }
                else { 0xFFFFFFFF }
            })
            .collect();

        window.buffer = buffer;
        window.update();

        chip8.keypad.fill(false);
    }
}
