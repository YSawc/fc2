extern crate sdl2;

use crate::emurator::{PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, SQUARE_SIZE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

mod util {
    pub fn char_to_bool(c: &char) -> bool {
        match c {
            '0' => false,
            '1' => true,
            _ => unimplemented!(),
        }
    }

    pub fn chars_to_u32(v: &Vec<char>) -> u32 {
        let mut r = 0;
        for n in v {
            r += n.to_digit(10).unwrap();
        }
        r
    }
}

mod nes {
    use crate::{emurator::NES_FILE, util};
    use std::fs::File;
    use std::io::Read;

    #[derive(Debug, Clone)]
    pub struct NES {
        pub header: Header,
    }

    #[derive(Debug, Clone)]
    pub struct Header {
        pub info: Info,
        pub flags6: Flags6,
        pub flags7: Flags7,
        pub flags8: Flags8,
        pub flags9: Flags9,
        pub flags10: Flags10,
    }

    #[derive(Debug, Clone)]
    pub struct Info {
        pub nes_header_size: u32,
        pub chr_rom_per_size: u32,
        pub prg_rom_per_size: u32,
        pub default_canvas_width: u32,
        pub sprites_num: u32,
        pub chr_rom_start: u32,
        pub prg_rom: Vec<u8>,
        pub chr_rom: Vec<u8>,
    }

    impl Info {
        pub fn new(buffer: &Vec<u8>) -> Self {
            use std::str;
            if str::from_utf8(&buffer[0..3]) != Ok("NES") {
                panic!("File format is not nes!");
            } else {
                println!("NES file read!");
            }

            let prm_rom_size = buffer[4] as u32;
            let chr_rom_size = buffer[5] as u32;

            if buffer[5] == 0 {
                println!("Info: The board uses chr RAM!");
            }

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
        pub fn new(buffer: &Vec<u8>) -> Self {
            let info = Info::new(&buffer);
            let flags6 = Flags6::parse_buf(&buffer[6]);
            let flags7 = Flags7::parse_buf(&buffer[7]);
            let flags8 = Flags8::parse_buf(&buffer[8]);
            let flags9 = Flags9::parse_buf(&buffer[9]);
            let flags10 = Flags10::parse_buf(&buffer[10]);

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

    #[derive(Debug, Clone)]
    pub struct Flags6 {
        pub mirroring: bool,
        pub ram_or_memory: bool,
        pub trainer: bool,
        pub ignore_mirroring: bool,
        pub mapper: u32,
    }

    impl Flags6 {
        pub fn parse_buf(num: &u8) -> Self {
            let s = format!("{:08b}", num);
            let v: Vec<char> = s.chars().collect();
            let mirroring = util::char_to_bool(&v[0]);
            let ram_or_memory = util::char_to_bool(&v[1]);
            let trainer = util::char_to_bool(&v[2]);
            let ignore_mirroring = util::char_to_bool(&v[3]);
            let mapper = util::chars_to_u32(&v[4..=7].to_vec());

            Self {
                mirroring,
                ram_or_memory,
                trainer,
                ignore_mirroring,
                mapper,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Flags7 {
        pub vs_unisystem: bool,
        pub play_choice_10: bool,
        pub nes_20_format: u32,
        pub mapper: u32,
    }

    impl Flags7 {
        pub fn parse_buf(num: &u8) -> Self {
            let s = format!("{:08b}", num);
            let v: Vec<char> = s.chars().collect();
            let vs_unisystem = util::char_to_bool(&v[0]);
            let play_choice_10 = util::char_to_bool(&v[1]);
            let nes_20_format = util::chars_to_u32(&v[2..=3].to_vec());
            let mapper = util::chars_to_u32(&v[4..=7].to_vec());

            Self {
                vs_unisystem,
                play_choice_10,
                nes_20_format,
                mapper,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Flags8 {
        pub prg_ram_size: u32,
    }

    impl Flags8 {
        pub fn parse_buf(num: &u8) -> Self {
            let s = format!("{:08b}", num);
            let v: Vec<char> = s.chars().collect();
            let prg_ram_size = util::chars_to_u32(&v[0..=7].to_vec());

            Self { prg_ram_size }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Flags9 {
        pub tv_system: bool,
        pub reserved: u32,
    }

    impl Flags9 {
        pub fn parse_buf(num: &u8) -> Self {
            let s = format!("{:08b}", num);
            let v: Vec<char> = s.chars().collect();
            let tv_system = util::char_to_bool(&v[0]);
            let reserved = util::chars_to_u32(&v[1..=7].to_vec());

            Self {
                tv_system,
                reserved,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Flags10 {
        pub tv_system: u32,
        pub prg_ram: bool,
        pub board_mode: bool,
    }

    impl Flags10 {
        pub fn parse_buf(num: &u8) -> Self {
            let s = format!("{:08b}", num);
            let v: Vec<char> = s.chars().collect();
            let tv_system = util::chars_to_u32(&v[0..=1].to_vec());
            let prg_ram = util::char_to_bool(&v[4]);
            let board_mode = util::char_to_bool(&v[5]);

            Self {
                tv_system,
                prg_ram,
                board_mode,
            }
        }
    }

    impl NES {
        pub fn new() -> Self {
            let mut f = File::open(NES_FILE).unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            let header = Header::new(&buffer);
            Self { header }
        }

        pub fn read_sprites(&self) -> Vec<Vec<Vec<u32>>> {
            let mut sprites = Vec::new();
            for n in 0..self.header.info.sprites_num {
                let mut sprite = vec![vec![0; 8]; 8];
                for i in 0..16 {
                    let val_str = self.header.info.chr_rom[(n * 16 + i) as usize];
                    sprite[(i % 8) as usize] = sprite[(i % 8) as usize]
                        .clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, x)| {
                            x + format!("{:08b}", val_str)
                                .chars()
                                .nth(idx)
                                .unwrap()
                                .to_digit(2)
                                .unwrap()
                        })
                        .collect::<Vec<_>>();
                }
                sprites.push(sprite);
            }
            sprites
        }
    }
}

mod cpu {
    #[derive(Debug, Clone)]
    pub struct CPU {
        pub map: Map,
        pub register: Register,
    }

    impl CPU {
        pub fn new() -> Self {
            let map = Map::new();
            let register = Register::new();
            Self { map, register }
        }

        pub fn handle_interrupt(&mut self, intr: Interrupt) {
            match intr {
                Interrupt::NMI => (),
                Interrupt::RESET => self.reset(),
                Interrupt::IRQ => (),
                Interrupt::BRK => (),
            }
        }

        pub fn reset(&mut self) {
            self.read_addr(0xFFFC, 0xFFFD)
        }

        pub fn set_nz(&mut self, n: u8) {
            if (n >> 7) == 1 {
                self.register.p.negative = true;
            }
            if n == 0 {
                self.register.p.zero = true;
            }
        }

        pub fn fetch_code(&mut self) -> u8 {
            let pc = self.register.pc;
            self.map.addr(pc)
        }

        pub fn fetch_register(&mut self) -> u8 {
            let pc = self.register.pc;
            self.map.addr(pc)
        }

        pub fn read_addr(&mut self, b: u16, u: u16) {
            self.register.x = self.map.addr(b);
            self.register.y = self.map.addr(u);
            self.register.set_pc();
        }

        pub fn undef(&mut self) {
            self.register.pc += 1;
        }

        pub fn nop(&mut self) {
            self.register.pc += 1;
        }

        pub fn ex_ope(&mut self, opekind: OpeKind, addr_mode: AddrMode) {
            self.register.pc += 1;
            match opekind {
                OpeKind::SEI => self.register.p.permit_irq = true,
                OpeKind::LDA => {
                    let r = self.fetch_register();
                    self.register.a = r;
                    self.set_nz(self.register.a);
                }
                OpeKind::LDX => {
                    let r = self.fetch_register();
                    self.register.x = r;
                    self.set_nz(self.register.x);
                }
                OpeKind::LDY => {
                    let r = self.fetch_register();
                    self.register.y = r;
                    self.set_nz(self.register.y);
                }
                OpeKind::TXS => {
                    self.register.s = self.register.x;
                    self.register.x = 0;
                }
                OpeKind::TAX => {
                    self.register.x = self.register.a;
                    self.register.a = 0;
                }
                OpeKind::STA => {
                    let b = self.fetch_register() as u16;
                    let r = b + (0 << 8);
                    self.map.set(r, self.register.a);
                }
                _ => {}
            }

            match addr_mode {
                AddrMode::AccA | AddrMode::Impl => (),
                AddrMode::ImmA
                | AddrMode::ZPA
                | AddrMode::ZPAX
                | AddrMode::ZPAY
                | AddrMode::RelA => self.register.pc += 1,
                AddrMode::AbsIA
                | AddrMode::AbsA
                | AddrMode::AbsAX
                | AddrMode::AbsAY
                | AddrMode::IAA
                | AddrMode::AIA
                | AddrMode::IZPA
                | AddrMode::IdxIA
                | AddrMode::IdrIA => self.register.pc += 2,
            }
        }

        pub fn read_ope(&mut self) {
            let c = self.fetch_code();
            println!("self.register.pc: {:0x?}", self.register.pc);
            println!("c: {:0x?}", c);
            match c {
                0x00 => self.ex_ope(OpeKind::BRK, AddrMode::Impl),
                0x01 => self.ex_ope(OpeKind::ORA, AddrMode::IdxIA),
                0x02 => self.undef(),
                0x03 => self.undef(),
                0x04 => self.undef(),
                0x05 => self.ex_ope(OpeKind::ORA, AddrMode::ZPA),
                0x06 => self.ex_ope(OpeKind::ASL, AddrMode::ZPA),
                0x07 => self.undef(),
                0x08 => self.ex_ope(OpeKind::PHP, AddrMode::Impl),
                0x09 => self.ex_ope(OpeKind::ORA, AddrMode::ImmA),
                0x0A => self.ex_ope(OpeKind::ASL, AddrMode::AccA),
                0x0B => self.undef(),
                0x0C => self.undef(),
                0x0D => self.ex_ope(OpeKind::ORA, AddrMode::AbsA),
                0x0E => self.ex_ope(OpeKind::ASL, AddrMode::AbsA),
                0x0F => self.undef(),

                0x10 => self.ex_ope(OpeKind::BPL, AddrMode::RelA),
                0x11 => self.ex_ope(OpeKind::ORA, AddrMode::IdrIA),
                0x12 => self.undef(),
                0x13 => self.undef(),
                0x14 => self.undef(),
                0x15 => self.ex_ope(OpeKind::ORA, AddrMode::ZPAX),
                0x16 => self.ex_ope(OpeKind::ASL, AddrMode::ZPAX),
                0x17 => self.undef(),
                0x18 => self.ex_ope(OpeKind::CLC, AddrMode::Impl),
                0x19 => self.ex_ope(OpeKind::BRK, AddrMode::AbsAY),
                0x1A => self.undef(),
                0x1B => self.undef(),
                0x1C => self.undef(),
                0x1D => self.ex_ope(OpeKind::ORA, AddrMode::AbsAX),
                0x1E => self.ex_ope(OpeKind::ASL, AddrMode::AbsAX),
                0x1F => self.undef(),

                0x20 => self.ex_ope(OpeKind::JSR, AddrMode::AbsA),
                0x21 => self.ex_ope(OpeKind::AND, AddrMode::IdxIA),
                0x22 => self.undef(),
                0x23 => self.undef(),
                0x24 => self.ex_ope(OpeKind::BIT, AddrMode::ZPA),
                0x25 => self.ex_ope(OpeKind::AND, AddrMode::ZPA),
                0x26 => self.ex_ope(OpeKind::ROL, AddrMode::ZPA),
                0x27 => self.undef(),
                0x28 => self.ex_ope(OpeKind::PLP, AddrMode::Impl),
                0x29 => self.ex_ope(OpeKind::AND, AddrMode::ImmA),
                0x2A => self.ex_ope(OpeKind::ROL, AddrMode::AccA),
                0x2B => self.undef(),
                0x2C => self.ex_ope(OpeKind::BIT, AddrMode::AbsA),
                0x2D => self.ex_ope(OpeKind::AND, AddrMode::AbsA),
                0x2E => self.ex_ope(OpeKind::ROL, AddrMode::AbsA),
                0x2F => self.undef(),

                0x30 => self.ex_ope(OpeKind::BMI, AddrMode::RelA),
                0x31 => self.ex_ope(OpeKind::AND, AddrMode::IdrIA),
                0x32 => self.undef(),
                0x33 => self.undef(),
                0x34 => self.undef(),
                0x35 => self.ex_ope(OpeKind::EOR, AddrMode::ZPAX),
                0x36 => self.ex_ope(OpeKind::ROL, AddrMode::ZPAX),
                0x37 => self.undef(),
                0x38 => self.ex_ope(OpeKind::SEC, AddrMode::Impl),
                0x39 => self.ex_ope(OpeKind::AND, AddrMode::AbsAY),
                0x3A => self.undef(),
                0x3B => self.undef(),
                0x3C => self.undef(),
                0x3D => self.ex_ope(OpeKind::AND, AddrMode::AbsAX),
                0x3E => self.ex_ope(OpeKind::ROL, AddrMode::AbsAX),
                0x3F => self.undef(),

                0x40 => self.ex_ope(OpeKind::RTI, AddrMode::Impl),
                0x41 => self.ex_ope(OpeKind::EOR, AddrMode::IdxIA),
                0x42 => self.undef(),
                0x43 => self.undef(),
                0x44 => self.undef(),
                0x45 => self.ex_ope(OpeKind::EOR, AddrMode::ZPA),
                0x46 => self.ex_ope(OpeKind::LSR, AddrMode::ZPA),
                0x47 => self.undef(),
                0x48 => self.ex_ope(OpeKind::PHA, AddrMode::IdxIA),
                0x49 => self.ex_ope(OpeKind::EOR, AddrMode::ImmA),
                0x4A => self.ex_ope(OpeKind::LSR, AddrMode::AccA),
                0x4B => self.undef(),
                0x4C => self.ex_ope(OpeKind::JMP, AddrMode::AbsA),
                0x4D => self.ex_ope(OpeKind::EOR, AddrMode::AbsA),
                0x4E => self.ex_ope(OpeKind::LSR, AddrMode::AbsA),
                0x4F => self.undef(),

                0x50 => self.ex_ope(OpeKind::BVC, AddrMode::RelA),
                0x51 => self.ex_ope(OpeKind::EOR, AddrMode::IdrIA),
                0x52 => self.undef(),
                0x53 => self.undef(),
                0x54 => self.undef(),
                0x55 => self.ex_ope(OpeKind::EOR, AddrMode::ZPAX),
                0x56 => self.ex_ope(OpeKind::LSR, AddrMode::ZPAX),
                0x57 => self.undef(),
                0x58 => self.ex_ope(OpeKind::CLI, AddrMode::Impl),
                0x59 => self.ex_ope(OpeKind::EOR, AddrMode::AbsAY),
                0x5A => self.undef(),
                0x5B => self.undef(),
                0x5C => self.undef(),
                0x5D => self.ex_ope(OpeKind::EOR, AddrMode::AbsAX),
                0x5E => self.ex_ope(OpeKind::LSR, AddrMode::AbsAX),
                0x5F => self.undef(),

                0x60 => self.ex_ope(OpeKind::RTS, AddrMode::Impl),
                0x61 => self.ex_ope(OpeKind::ADC, AddrMode::IdxIA),
                0x62 => self.undef(),
                0x63 => self.undef(),
                0x64 => self.undef(),
                0x65 => self.ex_ope(OpeKind::ADC, AddrMode::ZPA),
                0x66 => self.ex_ope(OpeKind::ROR, AddrMode::ZPA),
                0x67 => self.undef(),
                0x68 => self.ex_ope(OpeKind::PLA, AddrMode::Impl),
                0x69 => self.ex_ope(OpeKind::ADC, AddrMode::ImmA),
                0x6A => self.ex_ope(OpeKind::ROR, AddrMode::AccA),
                0x6B => self.undef(),
                0x6C => self.ex_ope(OpeKind::JMP, AddrMode::AbsIA),
                0x6D => self.ex_ope(OpeKind::ADC, AddrMode::AbsA),
                0x6E => self.ex_ope(OpeKind::ROR, AddrMode::AbsA),
                0x6F => self.undef(),

                0x70 => self.ex_ope(OpeKind::BVS, AddrMode::RelA),
                0x71 => self.ex_ope(OpeKind::ADC, AddrMode::IdrIA),
                0x72 => self.undef(),
                0x73 => self.undef(),
                0x74 => self.undef(),
                0x75 => self.ex_ope(OpeKind::ADC, AddrMode::ZPAX),
                0x76 => self.ex_ope(OpeKind::ROR, AddrMode::ZPAX),
                0x77 => self.undef(),
                0x78 => self.ex_ope(OpeKind::SEI, AddrMode::Impl),
                0x79 => self.ex_ope(OpeKind::ADC, AddrMode::AbsAY),
                0x7A => self.undef(),
                0x7B => self.undef(),
                0x7C => self.undef(),
                0x7D => self.ex_ope(OpeKind::ADC, AddrMode::AbsAX),
                0x7E => self.ex_ope(OpeKind::ROR, AddrMode::AbsAX),
                0x7F => self.undef(),

                0x80 => self.undef(),
                0x81 => self.ex_ope(OpeKind::STA, AddrMode::IdxIA),
                0x82 => self.undef(),
                0x83 => self.undef(),
                0x84 => self.ex_ope(OpeKind::STY, AddrMode::ZPA),
                0x85 => self.ex_ope(OpeKind::STA, AddrMode::ZPA),
                0x86 => self.ex_ope(OpeKind::STX, AddrMode::ZPA),
                0x87 => self.undef(),
                0x88 => self.ex_ope(OpeKind::DEY, AddrMode::Impl),
                0x89 => self.undef(),
                0x8A => self.ex_ope(OpeKind::TXA, AddrMode::Impl),
                0x8B => self.undef(),
                0x8C => self.ex_ope(OpeKind::TXA, AddrMode::Impl),
                0x8D => self.ex_ope(OpeKind::STY, AddrMode::AbsA),
                0x8E => self.ex_ope(OpeKind::STX, AddrMode::AbsA),
                0x8F => self.undef(),

                0x90 => self.ex_ope(OpeKind::BCC, AddrMode::RelA),
                0x91 => self.ex_ope(OpeKind::STA, AddrMode::IdrIA),
                0x92 => self.undef(),
                0x93 => self.undef(),
                0x94 => self.ex_ope(OpeKind::STY, AddrMode::ZPAX),
                0x95 => self.ex_ope(OpeKind::STA, AddrMode::ZPAX),
                0x96 => self.ex_ope(OpeKind::STX, AddrMode::ZPAY),
                0x97 => self.undef(),
                0x98 => self.ex_ope(OpeKind::TYA, AddrMode::Impl),
                0x99 => self.ex_ope(OpeKind::STA, AddrMode::AbsAY),
                0x9A => self.ex_ope(OpeKind::TXS, AddrMode::Impl),
                0x9B => self.undef(),
                0x9C => self.undef(),
                0x9D => self.ex_ope(OpeKind::STA, AddrMode::AbsAX),
                0x9E => self.undef(),
                0x9F => self.undef(),

                0xA0 => self.ex_ope(OpeKind::LDY, AddrMode::ImmA),
                0xA1 => self.ex_ope(OpeKind::LDA, AddrMode::IdxIA),
                0xA2 => self.ex_ope(OpeKind::LDX, AddrMode::ImmA),
                0xA3 => self.undef(),
                0xA4 => self.ex_ope(OpeKind::LDY, AddrMode::ZPA),
                0xA5 => self.ex_ope(OpeKind::LDA, AddrMode::ZPA),
                0xA6 => self.ex_ope(OpeKind::LDX, AddrMode::ZPA),
                0xA7 => self.undef(),
                0xA8 => self.ex_ope(OpeKind::TAY, AddrMode::Impl),
                0xA9 => self.ex_ope(OpeKind::LDA, AddrMode::ImmA),
                0xAA => self.ex_ope(OpeKind::TAX, AddrMode::Impl),
                0xAB => self.undef(),
                0xAC => self.ex_ope(OpeKind::LDY, AddrMode::AbsA),
                0xAD => self.ex_ope(OpeKind::LDA, AddrMode::AbsA),
                0xAE => self.ex_ope(OpeKind::LDX, AddrMode::AbsA),
                0xAF => self.undef(),

                0xB0 => self.ex_ope(OpeKind::BCS, AddrMode::RelA),
                0xB1 => self.ex_ope(OpeKind::LDA, AddrMode::IdrIA),
                0xB2 => self.undef(),
                0xB3 => self.undef(),
                0xB4 => self.ex_ope(OpeKind::LDY, AddrMode::ZPAX),
                0xB5 => self.ex_ope(OpeKind::LDA, AddrMode::ZPAX),
                0xB6 => self.ex_ope(OpeKind::LDX, AddrMode::ZPAY),
                0xB7 => self.undef(),
                0xB8 => self.ex_ope(OpeKind::CLV, AddrMode::Impl),
                0xB9 => self.ex_ope(OpeKind::LDA, AddrMode::AbsA),
                0xBA => self.ex_ope(OpeKind::TSX, AddrMode::Impl),
                0xBB => self.undef(),
                0xBC => self.ex_ope(OpeKind::LDY, AddrMode::AbsAX),
                0xBD => self.ex_ope(OpeKind::LDA, AddrMode::AbsA),
                0xBE => self.ex_ope(OpeKind::LDX, AddrMode::AbsAY),
                0xBF => self.undef(),

                0xC0 => self.ex_ope(OpeKind::CPY, AddrMode::ImmA),
                0xC1 => self.ex_ope(OpeKind::CMP, AddrMode::IdxIA),
                0xC2 => self.undef(),
                0xC3 => self.undef(),
                0xC4 => self.ex_ope(OpeKind::CPY, AddrMode::ZPA),
                0xC5 => self.ex_ope(OpeKind::CMP, AddrMode::ZPA),
                0xC6 => self.ex_ope(OpeKind::DEC, AddrMode::ZPA),
                0xC7 => self.undef(),
                0xC8 => self.ex_ope(OpeKind::INY, AddrMode::Impl),
                0xC9 => self.ex_ope(OpeKind::CMP, AddrMode::ImmA),
                0xCA => self.ex_ope(OpeKind::DEX, AddrMode::Impl),
                0xCB => self.undef(),
                0xCC => self.ex_ope(OpeKind::CPY, AddrMode::AbsA),
                0xCD => self.ex_ope(OpeKind::CMP, AddrMode::AbsA),
                0xCE => self.ex_ope(OpeKind::DEC, AddrMode::AbsA),
                0xCF => self.undef(),

                0xD0 => self.ex_ope(OpeKind::BNE, AddrMode::RelA),
                0xD1 => self.ex_ope(OpeKind::CMP, AddrMode::IdrIA),
                0xD2 => self.undef(),
                0xD3 => self.undef(),
                0xD4 => self.undef(),
                0xD5 => self.ex_ope(OpeKind::CMP, AddrMode::ZPAX),
                0xD6 => self.ex_ope(OpeKind::DEC, AddrMode::ZPAX),
                0xD7 => self.undef(),
                0xD8 => self.ex_ope(OpeKind::CLD, AddrMode::Impl),
                0xD9 => self.ex_ope(OpeKind::CMP, AddrMode::AbsAY),
                0xDA => self.undef(),
                0xDB => self.undef(),
                0xDC => self.undef(),
                0xDD => self.ex_ope(OpeKind::CMP, AddrMode::AbsAX),
                0xDE => self.ex_ope(OpeKind::DEC, AddrMode::AbsAX),
                0xDF => self.undef(),

                0xE0 => self.ex_ope(OpeKind::CPX, AddrMode::ImmA),
                0xE1 => self.ex_ope(OpeKind::SBC, AddrMode::IdxIA),
                0xE2 => self.undef(),
                0xE3 => self.undef(),
                0xE4 => self.ex_ope(OpeKind::CPX, AddrMode::ZPA),
                0xE5 => self.ex_ope(OpeKind::SBC, AddrMode::ZPA),
                0xE6 => self.ex_ope(OpeKind::INC, AddrMode::ZPA),
                0xE7 => self.undef(),
                0xE8 => self.ex_ope(OpeKind::INX, AddrMode::Impl),
                0xE9 => self.ex_ope(OpeKind::SBC, AddrMode::ImmA),
                0xEA => self.ex_ope(OpeKind::NOP, AddrMode::Impl),
                0xEB => self.undef(),
                0xEC => self.ex_ope(OpeKind::CPX, AddrMode::AbsA),
                0xED => self.ex_ope(OpeKind::SBC, AddrMode::AbsA),
                0xEE => self.ex_ope(OpeKind::INC, AddrMode::AbsA),
                0xEF => self.undef(),

                0xF0 => self.ex_ope(OpeKind::BEQ, AddrMode::RelA),
                0xF1 => self.ex_ope(OpeKind::SBC, AddrMode::IdrIA),
                0xF2 => self.undef(),
                0xF3 => self.undef(),
                0xF4 => self.undef(),
                0xF5 => self.ex_ope(OpeKind::SBC, AddrMode::ZPAX),
                0xF6 => self.ex_ope(OpeKind::INC, AddrMode::ZPAX),
                0xF7 => self.undef(),
                0xF8 => self.ex_ope(OpeKind::SED, AddrMode::Impl),
                0xF9 => self.ex_ope(OpeKind::SBC, AddrMode::AbsAY),
                0xFA => self.undef(),
                0xFB => self.undef(),
                0xFC => self.undef(),
                0xFD => self.ex_ope(OpeKind::SBC, AddrMode::AbsAX),
                0xFE => self.ex_ope(OpeKind::INC, AddrMode::AbsAX),
                0xFF => self.undef(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum AddrMode {
        AccA,
        ImmA,
        AbsA,
        AbsIA,
        AbsAX,
        AbsAY,
        ZPA,
        ZPAX,
        ZPAY,
        IZPA,
        IAA,
        Impl,
        RelA,
        IdxIA,
        IdrIA,
        AIA,
    }

    #[derive(Debug, Clone)]
    pub enum OpeKind {
        ADC,
        SBC, // flags: N V Z C

        AND,
        ORA,
        EOR, // flags: N Z

        ASL,
        LSR,
        ROL,
        ROR, // flags: N Z C

        BCC,
        BCS,
        BEQ,
        BNE,
        BVC,
        BVS,
        BPL,
        BMI, // flags: none

        BIT, // flags: N V Z

        JMP,
        JSR,
        RTS, // flags: none

        BRK, // flags: BI
        RTI, // flags: all

        CMP,
        CPX,
        CPY,
        INC,
        DEC,
        INX,
        DEX,
        INY,
        DEY, // flags: N Z

        CLC,
        SEC,
        CLI,
        SEI,
        CLD,
        SED,
        CLV, // flags: N Z

        LDA,
        LDX,
        LDY, // flags: N Z

        STA,
        STX,
        STY, // flags: none

        TAX,
        TXA,
        TAY,
        TYA,
        TSX, // flags: N Z
        TXS, // flags: none

        PHA, // flags: none
        PLA, // flags: N Z
        PHP, // flags: none
        PLP, // flags: all
        NOP, // flags: none
    }

    #[derive(Debug, Clone)]
    pub enum OpeFlags {
        NVZC,
        NZ,
        NZC,
        NVZ,
        BI,
        All,
        None,
    }

    #[derive(Debug, Clone)]
    pub struct Map {
        pub wram: [u8; 0x0800],
        pub wram_mirror: [u8; 0x0800],
        pub ppu_register: [u8; 0x0008],
        pub ppu_register_mirror: [u8; 0x0008],
        pub apu_pad: [u8; 0x0020],
        pub erom: [u8; 0x1FE0],
        pub eram: [u8; 0x2000],
        pub prg_rom1: [u8; 0x4000],
        pub prg_rom2: [u8; 0x4000],
    }

    impl Map {
        pub fn new() -> Self {
            Self {
                wram: [0; 0x0800],
                wram_mirror: [0; 0x0800],
                ppu_register: [0; 0x0008],
                ppu_register_mirror: [0; 0x0008],
                apu_pad: [0; 0x0020],
                erom: [0; 0x1FE0],
                eram: [0; 0x2000],
                prg_rom1: [0; 0x4000],
                prg_rom2: [0; 0x4000],
            }
        }

        pub fn addr(&self, n: u16) -> u8 {
            match n {
                0x0000..=0x07FF => self.wram[n as usize],
                0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize],
                0x2000..=0x2007 => self.ppu_register[(n - 0x2000) as usize],
                0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize],
                0x4000..=0x401F => self.apu_pad[(n - 0x4000) as usize],
                0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize],
                0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize],
                0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize],
                0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize],
            }
        }

        pub fn set(&mut self, n: u16, r: u8) {
            match n {
                0x0000..=0x07FF => self.wram[n as usize] = r,
                0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize] = r,
                0x2000..=0x2007 => self.ppu_register[(n - 0x2000) as usize] = r,
                0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize] = r,
                0x4000..=0x401F => self.apu_pad[(n - 0x4000) as usize] = r,
                0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize] = r,
                0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize] = r,
                0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize] = r,
                0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize] = r,
            };
        }
    }

    #[derive(Debug, Clone)]
    pub enum Interrupt {
        NMI,
        RESET,
        IRQ,
        BRK,
    }

    #[derive(Debug, Clone)]
    pub struct Register {
        pub a: u8,
        pub x: u8,
        pub y: u8,
        pub s: u8,
        pub p: P,
        pub pc: u16,
    }

    impl Register {
        pub fn new() -> Self {
            Self {
                a: 0,
                x: 0,
                y: 0,
                s: 0,
                p: P::new(),
                pc: 0,
            }
        }

        pub fn set_pc(&mut self) {
            let x = self.x as u16;
            let y = self.y as u16;
            self.pc = x + (y << 8);
        }
    }

    #[derive(Debug, Clone)]
    pub struct P {
        pub carry: bool,
        pub zero: bool,
        pub permit_irq: bool,
        pub decimal_mode: bool,
        pub break_mode: bool,
        pub reserved: u8,
        pub overflow: bool,
        pub negative: bool,
    }

    impl P {
        pub fn new() -> Self {
            Self {
                carry: false,
                zero: false,
                permit_irq: false,
                decimal_mode: false,
                break_mode: true,
                reserved: 0,
                overflow: false,
                negative: false,
            }
        }
    }
}

mod emurator {
    pub const SQUARE_SIZE: u32 = 8;
    pub const PLAYGROUND_WIDTH: u32 = 240;
    pub const PLAYGROUND_HEIGHT: u32 = 256;
    pub const NES_FILE: &str = "hello-world.nes";
}

pub fn main() -> Result<(), String> {
    let nes = nes::NES::new();
    // println!("{:#?}", nes);
    let mut cpu = cpu::CPU::new();
    // println!("{:#?}", cpu);
    {
        let prgs = &nes.header.info.prg_rom;
        if prgs.len() != 0x8000 {
            unimplemented!("prg_rom lengh is not 0x8000!");
        }
        for (i, n) in prgs.iter().enumerate() {
            match i {
                0x0000..=0x3FFF => cpu.map.prg_rom1[i] = *n,
                0x4000..=0x8000 => cpu.map.prg_rom2[i - 0x4000] = *n,
                _ => unreachable!(),
            }
        }
    }
    cpu.reset();

    let sprites = nes.read_sprites();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "fc2: render nes sprites",
            SQUARE_SIZE * PLAYGROUND_WIDTH,
            SQUARE_SIZE * PLAYGROUND_HEIGHT,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("\nUsing SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let (square_texture1, square_texture2, square_texture3, square_texture4) =
        dummy_texture(&mut canvas, &texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;
    // let mut frame: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(150, 150, 150));
        canvas.clear();

        for n in 0..nes.header.info.sprites_num {
            for i in 0..8 {
                for j in 0..8 {
                    let square_texture = match sprites[n as usize][i as usize][j as usize] {
                        0 => &square_texture1,
                        1 => &square_texture2,
                        2 => &square_texture3,
                        3 => &square_texture4,
                        _ => unreachable!(),
                    };

                    canvas.copy(
                        square_texture,
                        None,
                        Rect::new(
                            (j + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            (i + (n / PLAYGROUND_HEIGHT) * SQUARE_SIZE) as i32,
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ),
                    )?;
                }
            }
        }

        canvas.present();
        // frame += 1;
    }

    Ok(())
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>), String> {
    enum TextureColor {
        Zero,
        One,
        Two,
        Three,
    }
    let mut square_texture1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture3 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;
    let mut square_texture4 = texture_creator
        .create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
        .map_err(|e| e.to_string())?;

    {
        let textures = vec![
            (&mut square_texture1, TextureColor::Zero),
            (&mut square_texture2, TextureColor::One),
            (&mut square_texture3, TextureColor::Two),
            (&mut square_texture4, TextureColor::Three),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::Zero => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::One => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(85, 85, 85));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Two => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(170, 170, 170));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                    TextureColor::Three => {
                        for i in 0..SQUARE_SIZE {
                            for j in 0..SQUARE_SIZE {
                                texture_canvas.set_draw_color(Color::RGB(250, 250, 250));
                                texture_canvas
                                    .draw_point(Point::new(i as i32, j as i32))
                                    .expect("could not draw point");
                            }
                        }
                    }
                };
            })
            .map_err(|e| e.to_string())?;
    }
    Ok((
        square_texture1,
        square_texture2,
        square_texture3,
        square_texture4,
    ))
}
