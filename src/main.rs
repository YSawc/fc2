extern crate sdl2;
use fc2::emulator::*;
use fc2::nes::*;
use std::env;

pub fn main() -> Result<(), String> {
    let nes = Nes::new();
    let mut emulator = Emulator::default();
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
