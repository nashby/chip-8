const MAX_ROM_SIZE: usize = 3584;
pub const ROM_START: usize = 0x200;
pub struct Rom {
  pub data: [u8; MAX_ROM_SIZE],
}

impl Rom {
  pub fn load(name: &str) -> Rom  {
    let path = format!("roms/{name}");
    let bytes = std::fs::read(path).expect("filed to read ROM file");

    assert!(bytes.len() <= MAX_ROM_SIZE, "ROM too large: {} bytes", bytes.len());

    let mut data = [0u8; MAX_ROM_SIZE];
    data[..bytes.len()].copy_from_slice(&bytes);

    Rom { data }
  }
}
