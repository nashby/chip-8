pub mod window;
pub mod font;
pub mod rom;

pub struct Chip8 {
  pub memory: [u8; 4096],

  //registers
  v_registers: [u8; 16],
  index_register: u16,

  pub program_counter: u16,

  //stack
  stack: [u16; 16],
  stack_pointer: u8,

  //timers
  delay_timer: u8,
  sound_timer: u8,

  pub display_buffer: [u8; 64 * 32],
  keypad: [bool; 16]
}

impl Chip8 {
  pub fn new() -> Self {
    Self {
      memory: [0; 4096],
      v_registers: [0; 16],
      index_register: 0,
      program_counter: 0x200,
      stack: [0; 16],
      stack_pointer: 0,
      delay_timer: 0,
      sound_timer: 0,
      display_buffer: [0; 64 * 32],
      keypad: [false; 16],
    }
  }

  pub fn step(&mut self, window: &mut window::Window) {
    // fetch
    let pc = self.program_counter as usize;
    let opcode = ((self.memory[pc] as u16) << 8) | (self.memory[pc + 1] as u16);
    self.program_counter += 2;

    //decode
    let family_nibble = (opcode & 0xF000) >> 12;
    let x = ((opcode & 0x0F00) >> 8) as usize;
    let y = ((opcode & 0x00F0) >> 4) as usize;
    let n = (opcode & 0x000F) as u8;
    let nn = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;
    let nibbles = (
        family_nibble,
        x,
        y,
        n
    );

    println!("opcode: {:04X}", opcode);
    println!("program_counter: {}", self.program_counter);


    match nibbles {
        (0x0, 0x0, 0xE, 0x0) => self.op_cls(window),
        (0x1, _, _, _) => self.op_jp(nnn),
        (0x6, _, _, _) => self.op_ld_imm(x, nn),
        (0x7, _, _, _) => self.op_add_imm(x, nn),
        (0xA, _, _, _) => self.op_ld_i(nnn),
        (0xD, _, _, _) => self.op_drw(x, y, n),
        _ => panic!("unknown opcode: {:04X}", opcode),
    }
  }

  fn op_cls(&mut self, window: &mut window::Window) {
      window.clear(0);
  }

  fn op_jp(&mut self, nnn: u16) {
    self.program_counter = nnn;
  }

  fn op_ld_imm(&mut self, x: usize, nn: u8) {
    self.v_registers[x] = nn;
  }

  fn op_add_imm(&mut self, x: usize, nn: u8) {
    self.v_registers[x] += nn;
  }

  fn op_ld_i(&mut self, nnn: u16) {
    self.index_register = nnn;
  }

  fn op_drw(&mut self, x: usize, y: usize, n: u8) {
    let start_x = (self.v_registers[x] as usize) % 64;
    let start_y = (self.v_registers[y] as usize) % 32;

    self.v_registers[0xF] = 0;

    for row in 0..n as usize {
      let sprite_byte = self.memory[self.index_register as usize + row];

      for col in 0..8 as usize {
        let px = start_x + col;
        let py = start_y + row;

        if px >= 64 || py >= 32 {
          continue;
        }

        let sprite_pixel = (sprite_byte >> (7 - col)) & 0x1;

        if sprite_pixel == 1 {
            let screen_index = py * 64 + px;

            if self.display_buffer[screen_index] == 1 {
                self.v_registers[0xF] = 1;
            }
            self.display_buffer[screen_index] ^= 1;
        }
      }
    }
  }
}
