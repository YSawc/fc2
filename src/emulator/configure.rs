pub const SQUARE_SIZE: u32 = 8;
pub const PLAYGROUND_WIDTH: u32 = 32;
pub const PLAYGROUND_HEIGHT: u32 = 30;
pub const NES_FILE: &str = "hello-world.nes";
const CPU_CLOCK: f32 = 1.79;
const PPU_CLOCK: f32 = CPU_CLOCK * 3.0;
pub const PPU_CLOCK_RATE_FOR_CPU: u8 = (PPU_CLOCK / CPU_CLOCK) as u8;
