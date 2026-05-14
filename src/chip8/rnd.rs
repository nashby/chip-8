use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand() -> u16 {
    static mut STATE: u16 = 0;
    unsafe {
        if STATE == 0 {
            STATE = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u16 | 1;
        }
        STATE ^= STATE << 7;
        STATE ^= STATE >> 9;
        STATE ^= STATE << 8;
        STATE
    }
}
