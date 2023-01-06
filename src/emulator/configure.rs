pub const TILE_COUNTS_ON_WIDTH: u32 = 32;
pub const WINDOW_WIDTH: u32 = 512;
pub const WINDOW_HEIGHT: u32 = 480;
pub const PPU_DRAW_LINE_CYCLE: u16 = 341;
pub const VBLANK_LINE: u16 = 241;
const VBLANK_LINES: u16 = 20;
pub const VISIBLE_LINES: u16 = 240;
pub const TOTAL_LINE: u16 = VBLANK_LINES + VISIBLE_LINES + 2;
