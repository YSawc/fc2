pub mod mapper;
pub mod oam;

use crate::nes::*;
use mapper::*;
use oam::*;

#[derive(Debug, Clone)]
pub struct PPU {
    pub map: Map,
    pub primary_oam: PrimaryOAM,
    pub secondary_oam: SecondaryOAM,
    pub oam_buf: Vec<u8>,
}

impl PPU {
    pub fn new(nes: &Nes) -> Self {
        Self {
            map: Map::new(nes),
            primary_oam: PrimaryOAM::default(),
            secondary_oam: SecondaryOAM::default(),
            oam_buf: vec![],
        }
    }

    pub fn set_secondary_oam(&mut self, y: u8) {
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
    }
}
