use minifb::{Key as MiniKey, Window as MiniWindow, WindowOptions as MiniWindowOptions, Scale as MiniScale};

pub struct Window {
  window: MiniWindow,
  pub buffer: Vec<u32>
}

impl Window {
  const WIDTH: usize = 256;
  const HEIGHT: usize = 128;

  pub fn new(title: &str) -> Self {
    let mut window = MiniWindow::new(
        title,
        Self::WIDTH,
        Self::HEIGHT,
        MiniWindowOptions {
          scale: MiniScale::X4,
          ..MiniWindowOptions::default()
        }

    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(120);

    Self {
      window,
      buffer: vec![0; Self::WIDTH * Self::HEIGHT],
    }
  }

  pub fn is_open(&self) -> bool {
      self.window.is_open() && !self.window.is_key_down(MiniKey::Escape)
  }

  pub fn clear(&mut self, color: u32) {
      for px in self.buffer.iter_mut() {
          *px = color;
      }
  }

  pub fn update(&mut self) {
      self.window
          .update_with_buffer(&self.buffer, 64, 32)
          .unwrap();
  }
}
