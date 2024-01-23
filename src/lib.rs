pub mod client;
pub mod daemon;

pub const DISCONNECT: u8 = 0;
pub const NOP: u8 = 1;
pub const READ_FLOATING: u8 = 2;
pub const READ_PULLUP: u8 = 3;
pub const READ_PULLDN: u8 = 4;
pub const READ_ANALOG: u8 = 5;
pub const WRITE_FLOATING: u8 = 6;
pub const WRITE_DIGITAL: u8 = 7;
pub const WRITE_PWM: u8 = 8;
