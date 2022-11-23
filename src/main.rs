extern crate sdl2;
use fc2::emulator::configure::{
    PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, PPU_DRAW_LINE_CYCLE, SPRITE_SIZE, SQUARE_SIZE, TOTAL_LINE,
    VBLANK_LINE, VERTICAL_PIXEL,
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
        VERTICAL_PIXEL,
    > = Emulator::default();
    emulator.cpu.init(&nes);
    // println!("{:#?}", nes);
    // println!("{:#?}", emulator.cpu);
    emulator.reset();
    emulator.set_sprites(&nes.header.info.chr_rom);

    let args: Vec<String> = env::args().collect();
    if &args[1] == "show_sprites" {
        emulator.render_all_sprites(nes.header.info.sprites_num)?;
        return Ok(());
    }
    emulator.main_loop()?;
    Ok(())
}
