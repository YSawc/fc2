use crate::bus::Mapper;
use crate::nes::*;
use serde::{Deserialize, Serialize};
use serde_with::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    #[serde_as(as = "[_; 0x1000]")]
    pattern_table_00: [u8; 0x1000],
    #[serde_as(as = "[_; 0x1000]")]
    pattern_table_01: [u8; 0x1000],
    #[serde_as(as = "[_; 0x03C0]")]
    name_table_00: [u8; 0x03C0],
    #[serde_as(as = "[_; 0x0040]")]
    attr_table_00: [u8; 0x0040],
    #[serde_as(as = "[_; 0x03C0]")]
    name_table_01: [u8; 0x03C0],
    #[serde_as(as = "[_; 0x0040]")]
    attr_table_01: [u8; 0x0040],
    #[serde_as(as = "[_; 0x03C0]")]
    name_table_02: [u8; 0x03C0],
    #[serde_as(as = "[_; 0x0040]")]
    attr_table_02: [u8; 0x0040],
    #[serde_as(as = "[_; 0x03C0]")]
    name_table_03: [u8; 0x03C0],
    #[serde_as(as = "[_; 0x0040]")]
    attr_table_03: [u8; 0x0040],
    #[serde_as(as = "[_; 0x0F00]")]
    name_and_attr_table_mirror: [u8; 0x0F00],
    background_table: [u8; 0x0010],
    pub sprite_pallet: [u8; 0x0010],
    #[serde_as(as = "[_; 0x00E0]")]
    background_and_sprite_pallet_mirror: [u8; 0x00E0],
    type_of_mirroring: TypeOfMirroring,
}

impl Map {
    pub fn new(nes: &Nes) -> Self {
        let type_of_mirroring = nes.header.flags6.get_type_of_mirroring();
        Self {
            pattern_table_00: [0; 0x1000],
            pattern_table_01: [0; 0x1000],
            name_table_00: [0; 0x03C0],
            attr_table_00: [0; 0x0040],
            name_table_01: [0; 0x03C0],
            attr_table_01: [0; 0x0040],
            name_table_02: [0; 0x03C0],
            attr_table_02: [0; 0x0040],
            name_table_03: [0; 0x03C0],
            attr_table_03: [0; 0x0040],
            name_and_attr_table_mirror: [0; 0x0F00],
            background_table: [0; 0x0010],
            sprite_pallet: [0; 0x0010],
            background_and_sprite_pallet_mirror: [0; 0x00E0],
            type_of_mirroring,
        }
    }
}

impl Mapper for Map {
    fn addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x0FFF => self.pattern_table_00[addr as usize],
            0x1000..=0x1FFF => self.pattern_table_01[(addr - 0x1000) as usize],
            0x2000..=0x23BF => self.name_table_00[(addr - 0x2000) as usize],
            0x23C0..=0x23FF => self.attr_table_00[(addr - 0x23C0) as usize],
            0x2400..=0x27BF => self.name_table_01[(addr - 0x2400) as usize],
            0x27C0..=0x27FF => self.attr_table_01[(addr - 0x27C0) as usize],
            0x2800..=0x2BBF => self.name_table_02[(addr - 0x2800) as usize],
            0x2BC0..=0x2BFF => self.attr_table_02[(addr - 0x2BC0) as usize],
            0x2C00..=0x2FBF => self.name_table_03[(addr - 0x2C00) as usize],
            0x2FC0..=0x2FFF => self.attr_table_03[(addr - 0x2FC0) as usize],
            0x3000..=0x3EFF => self.name_and_attr_table_mirror[(addr - 0x3000) as usize],
            0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C => self.background_table[0 as usize],
            0x3F01..=0x3F03 | 0x3F05..=0x3F07 | 0x3F09..=0x3F0B | 0x03F0D..=0x03F0F => {
                self.background_table[(addr - 0x3F00) as usize]
            }
            0x3F10..=0x3F1F => self.sprite_pallet[(addr - 0x3F10) as usize],
            0x3F20..=0x3FFF => self.background_and_sprite_pallet_mirror[(addr - 0x3F20) as usize],
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x0FFF => self.pattern_table_00[addr as usize] = data,
            0x1000..=0x1FFF => self.pattern_table_01[(addr - 0x1000) as usize] = data,
            0x2000..=0x23BF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.name_table_01[(addr - 0x2000) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.name_table_02[(addr - 0x2000) as usize] = data
                    }
                    _ => (),
                }
                self.name_table_00[(addr - 0x2000) as usize] = data;
            }
            0x23C0..=0x23FF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.attr_table_01[(addr - 0x23C0) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.attr_table_02[(addr - 0x23C0) as usize] = data
                    }
                    _ => (),
                }
                self.attr_table_00[(addr - 0x23C0) as usize] = data;
            }
            0x2400..=0x27BF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.name_table_00[(addr - 0x2400) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.name_table_03[(addr - 0x2400) as usize] = data
                    }
                    _ => (),
                }
                self.name_table_01[(addr - 0x2400) as usize] = data;
            }
            0x27C0..=0x27FF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.attr_table_00[(addr - 0x27C0) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.attr_table_03[(addr - 0x27C0) as usize] = data
                    }
                    _ => (),
                }
                self.attr_table_01[(addr - 0x27C0) as usize] = data;
            }
            0x2800..=0x2BBF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.name_table_03[(addr - 0x2800) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.name_table_00[(addr - 0x2800) as usize] = data
                    }
                    _ => (),
                }
                self.name_table_02[(addr - 0x2800) as usize] = data;
            }
            0x2BC0..=0x2BFF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.attr_table_03[(addr - 0x2BC0) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.attr_table_00[(addr - 0x2BC0) as usize] = data
                    }
                    _ => (),
                }
                self.attr_table_02[(addr - 0x2BC0) as usize] = data;
            }
            0x2C00..=0x2FBF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.name_table_02[(addr - 0x2C00) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.name_table_01[(addr - 0x2C00) as usize] = data
                    }
                    _ => (),
                }
                self.name_table_03[(addr - 0x2C00) as usize] = data;
            }
            0x2FC0..=0x2FFF => {
                match self.type_of_mirroring {
                    TypeOfMirroring::HORIZONTAL => {
                        self.attr_table_02[(addr - 0x2FC0) as usize] = data
                    }
                    TypeOfMirroring::VERTICAL => {
                        self.attr_table_01[(addr - 0x2FC0) as usize] = data
                    }
                    _ => (),
                }
                self.attr_table_03[(addr - 0x2FC0) as usize] = data;
            }
            0x3000..=0x3EFF => self.name_and_attr_table_mirror[(addr - 0x3000) as usize] = data,
            0x3F00 | 0x3F04 | 0x3F08 | 0x3F0C => {
                self.sprite_pallet[(addr - 0x3F00) as usize] = data;
                self.background_table[(addr - 0x3F00) as usize] = data;
            }
            0x3F01..=0x3F03 | 0x3F05..=0x3F07 | 0x3F09..=0x3F0B | 0x03F0D..=0x03F0F => {
                self.background_table[(addr - 0x3F00) as usize] = data
            }
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                self.background_table[(addr - 0x3F10) as usize] = data;
                self.sprite_pallet[(addr - 0x3F10) as usize] = data;
            }
            0x3F11..=0x3F13 | 0x3F15..=0x3F17 | 0x3F19..=0x3F1B | 0x03F1D..=0x03F1F => {
                self.sprite_pallet[(addr - 0x3F10) as usize] = data
            }
            0x3F20..=0x3FFF => {
                self.background_and_sprite_pallet_mirror[(addr - 0x3F20) as usize] = data
            }
            _ => (),
        };
    }
}
