extern crate sdl2;

use fc2::emulator::*;
use fc2::nes::*;
use std::env;

fn main() -> Result<(), String> {
    let nes = Nes::default();
    let mut emulator: Emulator = Emulator::new(&nes);
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
