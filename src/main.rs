extern crate sdl2;
use fc2::emulator::configure::{
    PPU_DRAW_LINE_CYCLE, TILE_COUNTS_ON_WIDTH, TOTAL_LINE, VBLANK_LINE, VISIBLE_LINES,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

use fc2::emulator::*;
use fc2::nes::*;
use std::env;

fn main() -> Result<(), String> {
    let nes = Nes::new();
    let mut emulator: Emulator<
        TILE_COUNTS_ON_WIDTH,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VISIBLE_LINES,
    > = Emulator::new(&nes);
    emulator.cpu.init(&nes);
    emulator.startup();
    emulator.set_sprites(&nes.header.info.chr_rom);

    let args: Vec<String> = env::args().collect();
    if &args[1] == "show_sprites" {
        emulator.render_all_sprites(nes.header.info.sprites_num)?;
        return Ok(());
    }
    emulator.main_loop()?;
    Ok(())
}
