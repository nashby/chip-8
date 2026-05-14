mod chip8;
use chip8::window::Window;
use chip8::font;
use chip8::rom;
use chip8::Chip8;
use std::collections::HashMap;
use minifb::{InputCallback, Key};
use std::{cell::RefCell, rc::Rc};

type KeyVec = Rc<RefCell<Vec<u32>>>;
struct Input {
    keys: KeyVec,
}

impl InputCallback for Input {
    /// Will be called every time a character key is pressed
    fn add_char(&mut self, uni_char: u32) {
        self.keys.borrow_mut().push(uni_char);
    }
}

fn main() {
    let mut window = Window::new("Chip8");
    let mut chip8 = Chip8::new();
    let rom = rom::Rom::load("stars.ch8");
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
            chip8.step(&mut window);
        };

        chip8.tick_timers();

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

        chip8.keypad.fill(false);
    }
}
