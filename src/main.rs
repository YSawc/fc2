extern crate sdl2;
use fc2::emulator::configure::{
    PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, PPU_DRAW_LINE_CYCLE, SPRITE_SIZE, SQUARE_SIZE, TOTAL_LINE,
    VBLANK_LINE, VISIBLE_LINES,
};

use fc2::emulator::*;
use fc2::nes::*;
use std::env;

fn main() -> Result<(), String> {
    let nes = Nes::new();
    let mut emulator: Emulator<
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        SQUARE_SIZE,
        SPRITE_SIZE,
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
