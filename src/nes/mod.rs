use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nes {
    pub header: Header,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub info: Info,
    pub flags6: Flags6,
    flags7: Flags7,
    flags8: Flags8,
    flags9: Flags9,
    flags10: Flags10,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    nes_header_size: u32,
    chr_rom_per_size: u32,
    prg_rom_per_size: u32,
    default_canvas_width: u32,
    pub sprites_num: u32,
    chr_rom_start: u32,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Info {
    fn new(buffer: &[u8]) -> Self {
        use std::str;
        if str::from_utf8(&buffer[0..3]) != Ok("NES") {
            panic!("File format is not nes!");
        }

        let prm_rom_size = buffer[4] as u32;
        let chr_rom_size = buffer[5] as u32;
        let nes_header_size = 0x0010;
        let prg_rom_per_size = 0x4000;
        let chr_rom_per_size = 0x2000;
        let chr_rom_start = nes_header_size + prm_rom_size * prg_rom_per_size;
        let chr_rom_end = chr_rom_start + chr_rom_size * chr_rom_per_size;
        let default_canvas_width = 800;
        let sprites_num = chr_rom_per_size * chr_rom_size / 16;
        let prg_rom = buffer[(nes_header_size as usize)..(chr_rom_start as usize)].to_vec();
        let chr_rom = buffer[(chr_rom_start as usize)..(chr_rom_end as usize)].to_vec();

        Self {
            nes_header_size,
            chr_rom_per_size,
            prg_rom_per_size,
            default_canvas_width,
            sprites_num,
            chr_rom_start,
            prg_rom,
            chr_rom,
        }
    }
}

impl Header {
    pub fn new(buffer: &[u8]) -> Self {
        let info = Info::new(buffer);
        let flags6 = Flags6::parse_buf(buffer[6]);
        let flags7 = Flags7::parse_buf(buffer[7]);
        let flags8 = Flags8::parse_buf(buffer[8]);
        let flags9 = Flags9::parse_buf(buffer[9]);
        let flags10 = Flags10::parse_buf(buffer[10]);

        Self {
            info,
            flags6,
            flags7,
            flags8,
            flags9,
            flags10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeOfMirroring {
    HORIZONTAL,
    VERTICAL,
    IGNORING,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags6 {
    mirroring: bool,
    ram_or_memory: bool,
    trainer: bool,
    ignore_mirroring: bool,
    mapper: u8,
}

impl Flags6 {
    fn parse_buf(data: u8) -> Self {
        let mirroring = (data & 0b00000001) != 0;
        let ram_or_memory = (data & 0b00000010) != 0;
        let trainer = (data & 0b00000100) != 0;
        let ignore_mirroring = (data & 0b00001000) != 0;
        let mapper = (data & 0b11110000) >> 4;

        Self {
            mirroring,
            ram_or_memory,
            trainer,
            ignore_mirroring,
            mapper,
        }
    }

    pub fn get_type_of_mirroring(&self) -> TypeOfMirroring {
        match self.ignore_mirroring {
            false => match self.mirroring {
                true => TypeOfMirroring::VERTICAL,
                false => TypeOfMirroring::HORIZONTAL,
            },
            true => TypeOfMirroring::IGNORING,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Flags7 {
    vs_unisystem: bool,
    play_choice_10: bool,
    nes_20_format: u8,
    mapper: u8,
}

impl Flags7 {
    fn parse_buf(data: u8) -> Self {
        let vs_unisystem = (data & 0b00000001) != 0;
        let play_choice_10 = (data & 0b00000010) != 0;
        let nes_20_format = (data & 0b00001100) >> 2;
        let mapper = (data & 0b11110000) >> 4;

        Self {
            vs_unisystem,
            play_choice_10,
            nes_20_format,
            mapper,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Flags8 {
    prg_ram_size: u8,
}

impl Flags8 {
    fn parse_buf(data: u8) -> Self {
        let prg_ram_size = data;

        Self { prg_ram_size }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Flags9 {
    tv_system: bool,
    reserved: u8,
}

impl Flags9 {
    fn parse_buf(data: u8) -> Self {
        let tv_system = (data & 0b00000001) != 0;
        let reserved = 0;

        Self {
            tv_system,
            reserved,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Flags10 {
    tv_system: u8,
    prg_ram: bool,
    board_mode: bool,
}

impl Flags10 {
    fn parse_buf(data: u8) -> Self {
        let tv_system = data & 0b00000011;
        let prg_ram = (data & 0b00010000) != 0;
        let board_mode = (data & 0b00100000) != 0;

        Self {
            tv_system,
            prg_ram,
            board_mode,
        }
    }
}

pub type Sprites = Vec<Vec<Vec<u32>>>;

impl Default for Nes {
    fn default() -> Self {
        Self::new()
    }
}

impl Nes {
    fn new() -> Self {
        let file_path: Vec<String> = env::args().collect();
        let file_path = Path::new(&file_path[file_path.len() - 1]);
        let mut f = File::open(file_path).expect("File path need.");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        let header = Header::new(&buffer);
        Self { header }
    }
}
