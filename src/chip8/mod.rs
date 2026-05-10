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
  pub stack: [u16; 16],
  pub stack_pointer: usize,

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
      (0x0, 0x0, 0xE, 0xE) => self.op_rtn(),
      (0x1, _, _, _) => self.op_jp(nnn),
      (0x2, _, _, _) => self.op_call(nnn),
      (0x3, _, _, _) => self.op_skp_imm_if(x, nn),
      (0x4, _, _, _) => self.op_skp_imm_unless(x, nn),
      (0x5, _, _, 0x0) => self.op_skp_reg_if(x, y),
      (0x6, _, _, _) => self.op_ld_imm(x, nn),
      (0x7, _, _, _) => self.op_add_imm(x, nn),
      (0x8, _, _, 0x0) => self.op_reg_assign(x, y),
      (0x8, _, _, 0x1) => self.op_reg_or(x, y),
      (0x8, _, _, 0x2) => self.op_reg_and(x, y),
      (0x8, _, _, 0x3) => self.op_reg_xor(x, y),
      (0x8, _, _, 0x4) => self.op_reg_add(x, y),
      (0x8, _, _, 0x5) => self.op_reg_sub(x, y),
      (0x8, _, _, 0x6) => self.op_reg_right_move(x, y),
      (0x8, _, _, 0x7) => self.op_reg_sub_reverse(x, y),
      (0x8, _, _, 0xE) => self.op_reg_left_move(x, y),
      (0x9, _, _, 0x0) => self.op_skp_reg_unless(x, y),
      (0xA, _, _, _) => self.op_ld_i(nnn),
      (0xD, _, _, _) => self.op_drw(x, y, n),
      (0xF, _, 0x1, 0xE) => self.op_add_ir(x),
      (0xF, _, 0x3, 0x3) => self.op_bcd_reg(x),
      (0xF, _, 0x5, 0x5) => self.op_save_reg(x),
      (0xF, _, 0x6, 0x5) => self.op_ld_reg(x),
      _ => panic!("unknown opcode: {:04X}", opcode),
    }
  }

  fn op_cls(&mut self, window: &mut window::Window) {
      window.clear(0);
  }

  fn op_call(&mut self, nnn: u16) {
    self.stack[self.stack_pointer] = self.program_counter;
    self.stack_pointer += 1;

    self.program_counter = nnn;
  }

  fn op_add_ir(&mut self, x: usize) {
    let (result, _carry) = self.index_register.overflowing_add(self.v_registers[x] as u16);
    self.index_register = result;
  }

  fn op_bcd_reg(&mut self, x: usize) {
    let mut val = self.v_registers[x];
    let mut digits: [u8; 3] = [0; 3];
    let mut i: usize = 3;

    while val > 0 {
      i -= 1;
      digits[i] = val % 10;
      val /= 10;
    }

    println!("!!!!!!!");
    println!("{}", val);
    dbg!(self.v_registers[x]);
    dbg!(digits);

    for i in 0..=2 as u16 {
      self.memory[(self.index_register + i) as usize] = digits[i as usize];
    }
  }

  fn op_save_reg(&mut self, x: usize) {
    for i in 0..=x as u16 {
      self.memory[(self.index_register + i) as usize] = self.v_registers[i as usize]
    }
  }

  fn op_ld_reg(&mut self, x: usize) {
    for i in 0..=x as u16 {
      self.v_registers[i as usize] = self.memory[(self.index_register + i) as usize]
    }
  }

  fn op_reg_assign(&mut self, x: usize, y: usize) {
    self.v_registers[x] = self.v_registers[y];
  }

  fn op_reg_or(&mut self, x: usize, y: usize) {
    self.v_registers[x] |= self.v_registers[y];
  }

  fn op_reg_and(&mut self, x: usize, y: usize) {
    self.v_registers[x] &= self.v_registers[y];
  }

  fn op_reg_xor(&mut self, x: usize, y: usize) {
    self.v_registers[x] ^= self.v_registers[y];
  }

  fn op_reg_add(&mut self, x: usize, y: usize) {
    let (result, carry) = self.v_registers[x].overflowing_add(self.v_registers[y]);
    self.v_registers[x] = result;
    self.v_registers[0xF] = if carry { 1 } else { 0 };
  }

  fn op_reg_sub(&mut self, x: usize, y: usize) {
     let (result, borrow) = self.v_registers[x].overflowing_sub(self.v_registers[y]);
     self.v_registers[x] = result;
     self.v_registers[0xF] = if borrow { 0 } else { 1 };
  }

  fn op_reg_sub_reverse(&mut self, x: usize, y: usize) {
     let (result, borrow) = self.v_registers[y].overflowing_sub(self.v_registers[x]);
     self.v_registers[x] = result;
     self.v_registers[0xF] = if borrow { 0 } else { 1 };
  }

  fn op_reg_right_move(&mut self, x: usize, y: usize) {
     let old_sgb = self.v_registers[y] & 0x1;
     let (result, _borrow) = self.v_registers[y].overflowing_shr(1);
     self.v_registers[x] = result;
     self.v_registers[0xF] = old_sgb;
  }

  fn op_reg_left_move(&mut self, x: usize, y: usize) {
     let old_sgb = self.v_registers[y] & 0x0001;
     let (result, _borrow) = self.v_registers[y].overflowing_shl(1);
     self.v_registers[x] = result;
     self.v_registers[0xF] = old_sgb;
  }

  fn op_rtn(&mut self) {
    self.stack_pointer -= 1;
    self.program_counter = self.stack[self.stack_pointer];
  }

  fn op_jp(&mut self, nnn: u16) {
    self.program_counter = nnn;
  }

  fn op_skp_imm_if(&mut self, x: usize, nn: u8) {
    if self.v_registers[x] == nn {
      self.program_counter += 2;
    }
  }

  fn op_skp_imm_unless(&mut self, x: usize, nn: u8) {
    if self.v_registers[x] != nn {
      self.program_counter += 2;
    }
  }

  fn op_skp_reg_if(&mut self, x: usize, y: usize) {
    if self.v_registers[x] == self.v_registers[y] {
      self.program_counter += 2;
    }
  }

  fn op_skp_reg_unless(&mut self, x: usize, y: usize) {
    if self.v_registers[x] != self.v_registers[y] {
      self.program_counter += 2;
    }
  }

  fn op_ld_imm(&mut self, x: usize, nn: u8) {
    self.v_registers[x] = nn;
  }

  fn op_add_imm(&mut self, x: usize, nn: u8) {
    self.v_registers[x] = self.v_registers[x].wrapping_add(nn);
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
