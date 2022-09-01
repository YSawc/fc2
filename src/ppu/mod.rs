pub mod mapper;
pub mod oam;
pub mod register;

use mapper::*;
use oam::*;
use register::*;

#[derive(Debug, Clone)]
pub struct PPU {
    pub map: Map,
    pub register: Register,
    pub primary_oam: PrimaryOAM,
    pub secondary_oam: SecondaryOAM,
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

impl PPU {
    pub fn new() -> Self {
        Self {
            map: Map::default(),
            register: Register::default(),
            primary_oam: PrimaryOAM::default(),
            secondary_oam: SecondaryOAM::default(),
        }
    }

    pub fn set_secondary_oam_in_line(&mut self, y: u8) {
        let mut sprite_infos: SpriteInfos = vec![];
        for sprite_info in &self.primary_oam.sprite_infos {
            if sprite_info.in_drawing_range(y) {
                sprite_infos.push(sprite_info.to_owned());
                if sprite_infos.len() >= 8 {
                    break;
                }
            }
        }
        self.secondary_oam.sprite_infos = sprite_infos;
        // println!(
        //     "secondary_oams: {:0x?}, y: {:0x?}",
        //     self.secondary_oam.sprite_infos, y
        // );
    }
}
