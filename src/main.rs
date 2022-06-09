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
    pub struct Nes {
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
        pub fn new(buffer: &[u8]) -> Self {
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
        pub fn new(buffer: &[u8]) -> Self {
            let info = Info::new(buffer);
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

    impl Nes {
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
    pub struct Cpu {
        pub map: Map,
        pub register: Register,
    }

    impl Cpu {
        pub fn new() -> Self {
            let map = Map::new();
            let register = Register::new();
            Self { map, register }
        }

        // pub fn handle_interrupt(&mut self, intr: Interrupt) {
        //     match intr {
        //         Interrupt::Nmi => (),
        //         Interrupt::Reset => self.reset(),
        //         Interrupt::Irq => (),
        //         Interrupt::Brk => (),
        //     }
        // }

        pub fn reset(&mut self) {
            self.read_addr(0xFFFC, 0xFFFD)
        }

        pub fn ex_plus(&mut self, l: u8, r: u8) -> u8 {
            if l.checked_add(r).is_none() {
                self.register.p.overflow = true;
                l + r - u8::MAX
            } else {
                l + r
            }
        }

        pub fn ex_minus(&mut self, l: u8, r: u8) -> u8 {
            if l < r {
                self.register.p.negative = true;
                r - l
            } else {
                l - r
            }
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

        pub fn fetch_next_register(&mut self) -> u8 {
            let pc = self.register.pc + 1;
            self.map.addr(pc)
        }

        pub fn _fetch_next_next_register(&mut self) -> u8 {
            let pc = self.register.pc + 2;
            self.map.addr(pc)
        }

        pub fn read_addr(&mut self, b: u16, u: u16) {
            self.register.x = self.map.addr(b).try_into().unwrap();
            self.register.y = self.map.addr(u) as i8;
            self.register.set_pc();
        }

        pub fn undef(&mut self) {
            self.register.pc += 1;
        }

        pub fn nop(&mut self) {
            self.register.pc += 1;
        }

        pub fn push_stack(&mut self, n: u8) {
            let b = self.register.s as u16;
            self.register.s -= 1;
            let r = b + (1 << 8);
            self.map.set(r, n);
        }

        pub fn pull_stack(&mut self) -> u8 {
            let b = self.register.s + 1;
            self.register.s += 1;
            let h = 1 << 8;
            let r = b + h;
            self.map.addr(r)
        }

        pub fn ex_ope(&mut self, opekind: OpeKind, addr_mode: AddrMode) {
            self.register.pc += 1;
            let r = match addr_mode {
                AddrMode::Acc | AddrMode::Impl => 0,
                AddrMode::Imm => self.fetch_register() as u16,
                AddrMode::Zp => {
                    let b = self.fetch_register() as u16;
                    b as u16
                }
                AddrMode::ZpX => {
                    let mut b = self.fetch_register() as u16;
                    b += self.register.x as u16;
                    b as u16
                }
                AddrMode::ZpY => {
                    let mut b = self.fetch_register() as u16;
                    b += self.register.y as u16;
                    b as u16
                }
                AddrMode::Abs => {
                    let b = self.fetch_register() as u16;
                    let h = self.fetch_next_register() as u16;
                    (b + (h << 8)) as u16
                }
                AddrMode::AbsX => {
                    let mut b = self.fetch_register() as u16;
                    let h = self.fetch_next_register() as u16;
                    b += self.register.x as u16;
                    (b + (h << 8)) as u16
                }
                AddrMode::AbsY => {
                    let mut b = self.fetch_register() as u16;
                    let h = self.fetch_next_register() as u16;
                    b += self.register.y as u16;
                    (b + (h << 8)) as u16
                }
                AddrMode::Rel => {
                    let b = self.register.pc + 1;
                    let h = self.fetch_register() as u16;
                    (b + h) as u16
                }
                AddrMode::IndX => {
                    let mut b = self.fetch_register() as u16;
                    b += self.register.x as u16;
                    let t = b as u16;

                    let b = self.map.addr(t) as u16;
                    let h = self.map.addr(t + 1) as u16;
                    b + (h << 8)
                }
                AddrMode::IndY => {
                    let b = self.fetch_register() as u16;
                    let t = b;

                    let b = self.map.addr(t) as u16;
                    let mut h = self.map.addr(t + 1) as u16;
                    h += self.register.y as u16;
                    b + (h << 8)
                }
                AddrMode::Ind => {
                    let b = self.fetch_register() as u16;
                    let h = self.fetch_next_register() as u16;

                    let t = b + (h << 8);
                    let b = self.map.addr(t) as u16;
                    let h = self.map.addr(t + 1) as u16;

                    b + (h << 8)
                }
            };
            // println!("r: {:0x?}", r);

            match addr_mode {
                AddrMode::Acc | AddrMode::Impl => (),
                AddrMode::Imm
                | AddrMode::Zp
                | AddrMode::ZpX
                | AddrMode::ZpY
                | AddrMode::Rel
                | AddrMode::IndX
                | AddrMode::IndY => self.register.pc += 1,
                AddrMode::Abs | AddrMode::AbsX | AddrMode::AbsY | AddrMode::Ind => {
                    self.register.pc += 2
                }
            }

            match opekind {
                OpeKind::Adc => {
                    let t = self.map.addr(r) >> 7;
                    if self
                        .register
                        .a
                        .checked_add(self.map.addr(r).try_into().unwrap())
                        .is_some()
                    {
                        self.register.a += self.map.addr(r) as i8;
                        if self.map.addr(r) >> 7 != t {
                            self.register.p.negative = true;
                        };
                        self.set_nz(self.register.a.try_into().unwrap());
                    } else {
                        self.register.p.carry = true;
                    }
                }
                OpeKind::Sbc => {
                    let t = self.map.addr(r) >> 7;
                    if self
                        .register
                        .a
                        .checked_sub(self.map.addr(r).try_into().unwrap())
                        .is_some()
                    {
                        self.register.a -= self.map.addr(r) as i8;
                        if self.map.addr(r) >> 7 != t {
                            self.register.p.negative = true;
                        };
                        self.set_nz(self.register.a.try_into().unwrap());
                    } else {
                        self.register.p.carry = true;
                    }
                }
                OpeKind::And => {
                    self.register.a &= self.map.addr(r) as i8;
                }
                OpeKind::Ora => {
                    self.register.a |= self.map.addr(r) as i8;
                }
                OpeKind::Eor => {
                    self.register.a ^= self.map.addr(r) as i8;
                }
                OpeKind::Bcc => {
                    if !self.register.p.carry {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bcs => {
                    if self.register.p.carry {
                        self.register.pc = r;
                    }
                }
                OpeKind::Beq => {
                    if self.register.p.zero {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bne => {
                    if self.register.p.overflow {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bvc => {
                    if !self.register.p.overflow {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bvs => {
                    if self.register.p.overflow {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bpl => {
                    if !self.register.p.negative {
                        self.register.pc = r;
                    }
                }
                OpeKind::Bmi => {
                    if self.register.p.negative {
                        self.register.pc = r;
                    }
                }
                OpeKind::Cmp => {
                    let v = self.map.addr(r) as i8;
                    if self.register.a < v {
                        self.register.p.carry = false;
                    } else {
                        self.register.p.carry = true;
                    };
                }
                OpeKind::Cpx => {
                    let v = self.map.addr(r) as i8;
                    if self.register.x < v {
                        self.register.p.carry = false;
                    } else {
                        self.register.p.carry = true;
                    };
                }
                OpeKind::Cpy => {
                    let v = self.map.addr(r) as i8;
                    if self.register.y < v {
                        self.register.p.carry = false;
                    } else {
                        self.register.p.carry = true;
                    };
                }
                OpeKind::Inc => {
                    let v = self.map.addr(r);
                    let s = self.ex_plus(v, 1);
                    self.map.set(r, s);
                    self.set_nz(self.map.addr(r));
                }
                OpeKind::Dec => {
                    let v = self.map.addr(r);
                    let s = self.ex_minus(v, 1);
                    self.map.set(r, s);
                    self.set_nz(self.map.addr(r));
                }
                OpeKind::Inx => {
                    self.register.x += 1;
                    self.set_nz(self.register.x as u8);
                }
                OpeKind::Dex => {
                    self.register.x -= 1;
                    self.set_nz(self.register.x as u8);
                }
                OpeKind::Iny => {
                    self.register.y += 1;
                    self.set_nz(self.register.y as u8);
                }
                OpeKind::Dey => {
                    self.register.y -= 1;
                    self.set_nz(self.register.y as u8);
                }
                OpeKind::Clc => self.register.p.carry = false,
                OpeKind::Sec => self.register.p.carry = true,
                OpeKind::Cli => self.register.p.interrupt = false,
                OpeKind::Sei => self.register.p.interrupt = true,
                OpeKind::Cld => self.register.p.decimal = false,
                OpeKind::Sed => self.register.p.decimal = true,
                OpeKind::Clv => self.register.p.overflow = false,
                OpeKind::Lda => {
                    self.register.a = self.map.addr(r) as i8;
                    self.set_nz(self.register.a as u8);
                }
                OpeKind::Ldx => {
                    self.register.x = self.map.addr(r) as i8;
                    self.set_nz(self.register.x as u8);
                }
                OpeKind::Ldy => {
                    self.register.y = self.map.addr(r) as i8;
                    self.set_nz(self.register.y as u8);
                }
                OpeKind::Sta => {
                    self.map.set(r, self.register.a as u8);
                }
                OpeKind::Stx => {
                    self.map.set(r, self.register.x as u8);
                }
                OpeKind::Sty => {
                    self.map.set(r, self.register.y as u8);
                }
                OpeKind::Tax => {
                    self.register.x = self.register.a;
                    self.register.a = 0;
                    self.set_nz(self.register.x as u8);
                }
                OpeKind::Txa => {
                    self.register.a = self.register.x;
                    self.register.x = 0;
                    self.set_nz(self.register.a as u8);
                }
                OpeKind::Tay => {
                    self.register.y = self.register.a;
                    self.register.a = 0;
                    self.set_nz(self.register.y as u8);
                }
                OpeKind::Tya => {
                    self.register.a = self.register.y;
                    self.register.y = 0;
                    self.set_nz(self.register.a as u8);
                }
                OpeKind::Txs => {
                    self.register.set_s(self.register.x as u8);
                    self.register.x = 0;
                }
                OpeKind::Pha => {
                    self.register.a = self.map.addr(r).try_into().unwrap();
                    self.push_stack(self.register.a as u8);
                    self.register.a = 0;
                }
                OpeKind::Pla => {
                    let n = self.pull_stack();
                    self.register.a = n.try_into().unwrap();
                    self.set_nz(self.register.a as u8);
                }
                OpeKind::Php => {
                    let n = self.register.p.to_n();
                    self.push_stack(n);
                    self.set_nz(n);
                }
                OpeKind::Plp => {
                    let n = self.pull_stack();
                    self.register.p.set(n);
                    let n2 = self.register.p.to_n();
                    self.set_nz(n2);
                }
                OpeKind::Jmp => {
                    self.register.pc = r;
                }
                OpeKind::Jsr => {
                    let p = self.register.pc - 1;
                    let h = ((p & 0xFF00) >> 8) as u8;
                    let b = (p & 0x00FF) as u8;
                    self.push_stack(h);
                    self.push_stack(b);
                    self.register.pc = r;
                }
                OpeKind::Rts => {
                    let b = self.pull_stack() as u16;
                    let h = self.pull_stack() as u16;
                    let t = b + (h << 8);
                    self.register.pc = t;
                    self.register.pc += 1;
                }
                OpeKind::Brk => {
                    if !self.register.p.interrupt {
                        self.register.pc -= 1;
                        let p = self.register.pc;
                        let h = ((p & 0xFF00) >> 8) as u8;
                        let b = (p & 0x00FF) as u8;
                        self.push_stack(h);
                        self.push_stack(b);
                        let n = self.register.p.to_n();
                        self.push_stack(n);
                        self.register.p.break_mode = true;
                        self.register.p.interrupt = true;
                        let h = self.map.addr(0xFFFE) as u16;
                        let b = self.map.addr(0xFFFF) as u16;
                        self.register.pc = b + (h << 8);
                    }
                }
                OpeKind::Nop => {
                    self.nop();
                }
                _ => {
                    println!("opekind: {:?}, r: {:0x?} {}(10)", opekind, r, r);
                    unimplemented!();
                }
            }
        }

        pub fn read_ope(&mut self) {
            let c = self.fetch_code();
            // print!("self.register.pc: {:0x?} ", self.register.pc);
            // print!(
            //     "c: {:0x?}, c1: {:0x?}, c2: {:0x?} ",
            //     c,
            //     self.fetch_next_register(),
            //     self._fetch_next_next_register()
            // );
            // print!("0x2000 {}, ", self.map.addr(0x2000));
            // print!("0x2001 {}, ", self.map.addr(0x2001));
            // print!("0x2002 {}, ", self.map.addr(0x2002));
            // print!("0x2003 {}, ", self.map.addr(0x2003));
            // print!("0x2004 {}, ", self.map.addr(0x2004));
            // print!("0x2005 {}, ", self.map.addr(0x2005));
            // print!("0x2006 {}, ", self.map.addr(0x2006));
            // println!("0x2007 {}", self.map.addr(0x2007));

            match c {
                0x00 => self.ex_ope(OpeKind::Brk, AddrMode::Impl),
                0x01 => self.ex_ope(OpeKind::Ora, AddrMode::IndX),
                0x02 => self.undef(),
                0x03 => self.undef(),
                0x04 => self.undef(),
                0x05 => self.ex_ope(OpeKind::Ora, AddrMode::Zp),
                0x06 => self.ex_ope(OpeKind::Asl, AddrMode::Zp),
                0x07 => self.undef(),
                0x08 => self.ex_ope(OpeKind::Php, AddrMode::Impl),
                0x09 => self.ex_ope(OpeKind::Ora, AddrMode::Imm),
                0x0A => self.ex_ope(OpeKind::Asl, AddrMode::Acc),
                0x0B => self.undef(),
                0x0C => self.undef(),
                0x0D => self.ex_ope(OpeKind::Ora, AddrMode::Abs),
                0x0E => self.ex_ope(OpeKind::Asl, AddrMode::Abs),
                0x0F => self.undef(),

                0x10 => self.ex_ope(OpeKind::Bpl, AddrMode::Rel),
                0x11 => self.ex_ope(OpeKind::Ora, AddrMode::IndY),
                0x12 => self.undef(),
                0x13 => self.undef(),
                0x14 => self.undef(),
                0x15 => self.ex_ope(OpeKind::Ora, AddrMode::ZpX),
                0x16 => self.ex_ope(OpeKind::Asl, AddrMode::ZpX),
                0x17 => self.undef(),
                0x18 => self.ex_ope(OpeKind::Clc, AddrMode::Impl),
                0x19 => self.ex_ope(OpeKind::Ora, AddrMode::AbsY),
                0x1A => self.undef(),
                0x1B => self.undef(),
                0x1C => self.undef(),
                0x1D => self.ex_ope(OpeKind::Ora, AddrMode::AbsX),
                0x1E => self.ex_ope(OpeKind::Asl, AddrMode::AbsX),
                0x1F => self.undef(),

                0x20 => self.ex_ope(OpeKind::Jsr, AddrMode::Abs),
                0x21 => self.ex_ope(OpeKind::And, AddrMode::IndX),
                0x22 => self.undef(),
                0x23 => self.undef(),
                0x24 => self.ex_ope(OpeKind::Bit, AddrMode::Zp),
                0x25 => self.ex_ope(OpeKind::And, AddrMode::Zp),
                0x26 => self.ex_ope(OpeKind::Rol, AddrMode::Zp),
                0x27 => self.undef(),
                0x28 => self.ex_ope(OpeKind::Plp, AddrMode::Impl),
                0x29 => self.ex_ope(OpeKind::And, AddrMode::Imm),
                0x2A => self.ex_ope(OpeKind::Rol, AddrMode::Acc),
                0x2B => self.undef(),
                0x2C => self.ex_ope(OpeKind::Bit, AddrMode::Abs),
                0x2D => self.ex_ope(OpeKind::And, AddrMode::Abs),
                0x2E => self.ex_ope(OpeKind::Rol, AddrMode::Abs),
                0x2F => self.undef(),

                0x30 => self.ex_ope(OpeKind::Bmi, AddrMode::Rel),
                0x31 => self.ex_ope(OpeKind::And, AddrMode::IndY),
                0x32 => self.undef(),
                0x33 => self.undef(),
                0x34 => self.undef(),
                0x35 => self.ex_ope(OpeKind::And, AddrMode::ZpX),
                0x36 => self.ex_ope(OpeKind::Rol, AddrMode::ZpX),
                0x37 => self.undef(),
                0x38 => self.ex_ope(OpeKind::Sec, AddrMode::Impl),
                0x39 => self.ex_ope(OpeKind::And, AddrMode::AbsY),
                0x3A => self.undef(),
                0x3B => self.undef(),
                0x3C => self.undef(),
                0x3D => self.ex_ope(OpeKind::And, AddrMode::AbsX),
                0x3E => self.ex_ope(OpeKind::Rol, AddrMode::AbsX),
                0x3F => self.undef(),

                0x40 => self.ex_ope(OpeKind::Rti, AddrMode::Impl),
                0x41 => self.ex_ope(OpeKind::Eor, AddrMode::IndX),
                0x42 => self.undef(),
                0x43 => self.undef(),
                0x44 => self.undef(),
                0x45 => self.ex_ope(OpeKind::Eor, AddrMode::Zp),
                0x46 => self.ex_ope(OpeKind::Lsr, AddrMode::Zp),
                0x47 => self.undef(),
                0x48 => self.ex_ope(OpeKind::Pha, AddrMode::Impl),
                0x49 => self.ex_ope(OpeKind::Eor, AddrMode::Imm),
                0x4A => self.ex_ope(OpeKind::Lsr, AddrMode::Acc),
                0x4B => self.undef(),
                0x4C => self.ex_ope(OpeKind::Jmp, AddrMode::Abs),
                0x4D => self.ex_ope(OpeKind::Eor, AddrMode::Abs),
                0x4E => self.ex_ope(OpeKind::Lsr, AddrMode::Abs),
                0x4F => self.undef(),

                0x50 => self.ex_ope(OpeKind::Bvc, AddrMode::Rel),
                0x51 => self.ex_ope(OpeKind::Eor, AddrMode::IndY),
                0x52 => self.undef(),
                0x53 => self.undef(),
                0x54 => self.undef(),
                0x55 => self.ex_ope(OpeKind::Eor, AddrMode::ZpX),
                0x56 => self.ex_ope(OpeKind::Lsr, AddrMode::ZpX),
                0x57 => self.undef(),
                0x58 => self.ex_ope(OpeKind::Cli, AddrMode::Impl),
                0x59 => self.ex_ope(OpeKind::Eor, AddrMode::AbsY),
                0x5A => self.undef(),
                0x5B => self.undef(),
                0x5C => self.undef(),
                0x5D => self.ex_ope(OpeKind::Eor, AddrMode::AbsX),
                0x5E => self.ex_ope(OpeKind::Lsr, AddrMode::AbsX),
                0x5F => self.undef(),

                0x60 => self.ex_ope(OpeKind::Rts, AddrMode::Impl),
                0x61 => self.ex_ope(OpeKind::Adc, AddrMode::IndX),
                0x62 => self.undef(),
                0x63 => self.undef(),
                0x64 => self.undef(),
                0x65 => self.ex_ope(OpeKind::Adc, AddrMode::Zp),
                0x66 => self.ex_ope(OpeKind::Ror, AddrMode::Zp),
                0x67 => self.undef(),
                0x68 => self.ex_ope(OpeKind::Pla, AddrMode::Impl),
                0x69 => self.ex_ope(OpeKind::Adc, AddrMode::Imm),
                0x6A => self.ex_ope(OpeKind::Ror, AddrMode::Acc),
                0x6B => self.undef(),
                0x6C => self.ex_ope(OpeKind::Jmp, AddrMode::Ind),
                0x6D => self.ex_ope(OpeKind::Adc, AddrMode::Abs),
                0x6E => self.ex_ope(OpeKind::Ror, AddrMode::Abs),
                0x6F => self.undef(),

                0x70 => self.ex_ope(OpeKind::Bvs, AddrMode::Rel),
                0x71 => self.ex_ope(OpeKind::Adc, AddrMode::IndY),
                0x72 => self.undef(),
                0x73 => self.undef(),
                0x74 => self.undef(),
                0x75 => self.ex_ope(OpeKind::Adc, AddrMode::ZpX),
                0x76 => self.ex_ope(OpeKind::Ror, AddrMode::ZpX),
                0x77 => self.undef(),
                0x78 => self.ex_ope(OpeKind::Sei, AddrMode::Impl),
                0x79 => self.ex_ope(OpeKind::Adc, AddrMode::AbsY),
                0x7A => self.undef(),
                0x7B => self.undef(),
                0x7C => self.undef(),
                0x7D => self.ex_ope(OpeKind::Adc, AddrMode::AbsX),
                0x7E => self.ex_ope(OpeKind::Ror, AddrMode::AbsX),
                0x7F => self.undef(),

                0x80 => self.undef(),
                0x81 => self.ex_ope(OpeKind::Sta, AddrMode::IndX),
                0x82 => self.undef(),
                0x83 => self.undef(),
                0x84 => self.ex_ope(OpeKind::Sty, AddrMode::Zp),
                0x85 => self.ex_ope(OpeKind::Sta, AddrMode::Zp),
                0x86 => self.ex_ope(OpeKind::Stx, AddrMode::Zp),
                0x87 => self.undef(),
                0x88 => self.ex_ope(OpeKind::Dey, AddrMode::Impl),
                0x89 => self.undef(),
                0x8A => self.ex_ope(OpeKind::Txa, AddrMode::Impl),
                0x8B => self.undef(),
                0x8C => self.ex_ope(OpeKind::Sty, AddrMode::Abs),
                0x8D => self.ex_ope(OpeKind::Sta, AddrMode::Abs),
                0x8E => self.ex_ope(OpeKind::Stx, AddrMode::Abs),
                0x8F => self.undef(),

                0x90 => self.ex_ope(OpeKind::Bcc, AddrMode::Rel),
                0x91 => self.ex_ope(OpeKind::Sta, AddrMode::IndY),
                0x92 => self.undef(),
                0x93 => self.undef(),
                0x94 => self.ex_ope(OpeKind::Sty, AddrMode::ZpX),
                0x95 => self.ex_ope(OpeKind::Sta, AddrMode::ZpX),
                0x96 => self.ex_ope(OpeKind::Stx, AddrMode::ZpY),
                0x97 => self.undef(),
                0x98 => self.ex_ope(OpeKind::Tya, AddrMode::Impl),
                0x99 => self.ex_ope(OpeKind::Sta, AddrMode::AbsY),
                0x9A => self.ex_ope(OpeKind::Txs, AddrMode::Impl),
                0x9B => self.undef(),
                0x9C => self.undef(),
                0x9D => self.ex_ope(OpeKind::Sta, AddrMode::AbsX),
                0x9E => self.undef(),
                0x9F => self.undef(),

                0xA0 => self.ex_ope(OpeKind::Ldy, AddrMode::Imm),
                0xA1 => self.ex_ope(OpeKind::Lda, AddrMode::IndX),
                0xA2 => self.ex_ope(OpeKind::Ldx, AddrMode::Imm),
                0xA3 => self.undef(),
                0xA4 => self.ex_ope(OpeKind::Ldy, AddrMode::Zp),
                0xA5 => self.ex_ope(OpeKind::Lda, AddrMode::Zp),
                0xA6 => self.ex_ope(OpeKind::Ldx, AddrMode::Zp),
                0xA7 => self.undef(),
                0xA8 => self.ex_ope(OpeKind::Tay, AddrMode::Impl),
                0xA9 => self.ex_ope(OpeKind::Lda, AddrMode::Imm),
                0xAA => self.ex_ope(OpeKind::Tax, AddrMode::Impl),
                0xAB => self.undef(),
                0xAC => self.ex_ope(OpeKind::Ldy, AddrMode::Abs),
                0xAD => self.ex_ope(OpeKind::Lda, AddrMode::Abs),
                0xAE => self.ex_ope(OpeKind::Ldx, AddrMode::Abs),
                0xAF => self.undef(),

                0xB0 => self.ex_ope(OpeKind::Bcs, AddrMode::Rel),
                0xB1 => self.ex_ope(OpeKind::Lda, AddrMode::IndY),
                0xB2 => self.undef(),
                0xB3 => self.undef(),
                0xB4 => self.ex_ope(OpeKind::Ldy, AddrMode::ZpX),
                0xB5 => self.ex_ope(OpeKind::Lda, AddrMode::ZpX),
                0xB6 => self.ex_ope(OpeKind::Ldx, AddrMode::ZpY),
                0xB7 => self.undef(),
                0xB8 => self.ex_ope(OpeKind::Clv, AddrMode::Impl),
                0xB9 => self.ex_ope(OpeKind::Lda, AddrMode::AbsY),
                0xBA => self.ex_ope(OpeKind::Tsx, AddrMode::Impl),
                0xBB => self.undef(),
                0xBC => self.ex_ope(OpeKind::Ldy, AddrMode::AbsX),
                0xBD => self.ex_ope(OpeKind::Lda, AddrMode::AbsX),
                0xBE => self.ex_ope(OpeKind::Ldx, AddrMode::AbsY),
                0xBF => self.undef(),

                0xC0 => self.ex_ope(OpeKind::Cpy, AddrMode::Imm),
                0xC1 => self.ex_ope(OpeKind::Cmp, AddrMode::IndX),
                0xC2 => self.undef(),
                0xC3 => self.undef(),
                0xC4 => self.ex_ope(OpeKind::Cpy, AddrMode::Zp),
                0xC5 => self.ex_ope(OpeKind::Cmp, AddrMode::Zp),
                0xC6 => self.ex_ope(OpeKind::Dec, AddrMode::Zp),
                0xC7 => self.undef(),
                0xC8 => self.ex_ope(OpeKind::Iny, AddrMode::Impl),
                0xC9 => self.ex_ope(OpeKind::Cmp, AddrMode::Imm),
                0xCA => self.ex_ope(OpeKind::Dex, AddrMode::Impl),
                0xCB => self.undef(),
                0xCC => self.ex_ope(OpeKind::Cpy, AddrMode::Abs),
                0xCD => self.ex_ope(OpeKind::Cmp, AddrMode::Abs),
                0xCE => self.ex_ope(OpeKind::Dec, AddrMode::Abs),
                0xCF => self.undef(),

                0xD0 => self.ex_ope(OpeKind::Bne, AddrMode::Rel),
                0xD1 => self.ex_ope(OpeKind::Cmp, AddrMode::IndY),
                0xD2 => self.undef(),
                0xD3 => self.undef(),
                0xD4 => self.undef(),
                0xD5 => self.ex_ope(OpeKind::Cmp, AddrMode::ZpX),
                0xD6 => self.ex_ope(OpeKind::Dec, AddrMode::ZpX),
                0xD7 => self.undef(),
                0xD8 => self.ex_ope(OpeKind::Cld, AddrMode::Impl),
                0xD9 => self.ex_ope(OpeKind::Cmp, AddrMode::AbsY),
                0xDA => self.undef(),
                0xDB => self.undef(),
                0xDC => self.undef(),
                0xDD => self.ex_ope(OpeKind::Cmp, AddrMode::AbsX),
                0xDE => self.ex_ope(OpeKind::Dec, AddrMode::AbsX),
                0xDF => self.undef(),

                0xE0 => self.ex_ope(OpeKind::Cpx, AddrMode::Imm),
                0xE1 => self.ex_ope(OpeKind::Sbc, AddrMode::IndX),
                0xE2 => self.undef(),
                0xE3 => self.undef(),
                0xE4 => self.ex_ope(OpeKind::Cpx, AddrMode::Zp),
                0xE5 => self.ex_ope(OpeKind::Sbc, AddrMode::Zp),
                0xE6 => self.ex_ope(OpeKind::Inc, AddrMode::Zp),
                0xE7 => self.undef(),
                0xE8 => self.ex_ope(OpeKind::Inx, AddrMode::Impl),
                0xE9 => self.ex_ope(OpeKind::Sbc, AddrMode::Imm),
                0xEA => self.ex_ope(OpeKind::Nop, AddrMode::Impl),
                0xEB => self.undef(),
                0xEC => self.ex_ope(OpeKind::Cpx, AddrMode::Abs),
                0xED => self.ex_ope(OpeKind::Sbc, AddrMode::Abs),
                0xEE => self.ex_ope(OpeKind::Inc, AddrMode::Abs),
                0xEF => self.undef(),

                0xF0 => self.ex_ope(OpeKind::Beq, AddrMode::Rel),
                0xF1 => self.ex_ope(OpeKind::Sbc, AddrMode::IndY),
                0xF2 => self.undef(),
                0xF3 => self.undef(),
                0xF4 => self.undef(),
                0xF5 => self.ex_ope(OpeKind::Sbc, AddrMode::ZpX),
                0xF6 => self.ex_ope(OpeKind::Inc, AddrMode::ZpX),
                0xF7 => self.undef(),
                0xF8 => self.ex_ope(OpeKind::Sed, AddrMode::Impl),
                0xF9 => self.ex_ope(OpeKind::Sbc, AddrMode::AbsY),
                0xFA => self.undef(),
                0xFB => self.undef(),
                0xFC => self.undef(),
                0xFD => self.ex_ope(OpeKind::Sbc, AddrMode::AbsX),
                0xFE => self.ex_ope(OpeKind::Inc, AddrMode::AbsX),
                0xFF => self.undef(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum AddrMode {
        Acc,
        Imm,
        Abs,
        AbsX,
        AbsY,
        Zp,
        ZpX,
        ZpY,
        Impl,
        Rel,
        IndX,
        IndY,
        Ind,
    }

    #[derive(Debug, Clone)]
    pub enum OpeKind {
        Adc,
        Sbc, // flags: N V Z C

        And,
        Ora,
        Eor, // flags: N Z

        Asl,
        Lsr,
        Rol,
        Ror, // flags: N Z C

        Bcc,
        Bcs,
        Beq,
        Bne,
        Bvc,
        Bvs,
        Bpl,
        Bmi, // flags: none

        Bit, // flags: N V Z

        Jmp,
        Jsr,
        Rts, // flags: none

        Brk, // flags: Bi
        Rti, // flags: all

        Cmp,
        Cpx,
        Cpy,
        Inc,
        Dec,
        Inx,
        Dex,
        Iny,
        Dey, // flags: N Z

        Clc,
        Sec,
        Cli,
        Sei,
        Cld,
        Sed,
        Clv, // flags: N Z

        Lda,
        Ldx,
        Ldy, // flags: N Z

        Sta,
        Stx,
        Sty, // flags: none

        Tax,
        Txa,
        Tay,
        Tya,
        Tsx, // flags: N Z
        Txs, // flags: none

        Pha, // flags: none
        Pla, // flags: N Z
        Php, // flags: none
        Plp, // flags: all
        Nop, // flags: none
    }

    // #[derive(Debug, Clone)]
    // pub enum OpeFlags {
    //     Nvzc,
    //     Nz,
    //     Nzc,
    //     Nvz,
    //     Bi,
    //     All,
    //     None,
    // }

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

    // #[derive(Debug, Clone)]
    // pub enum Interrupt {
    //     Nmi,
    //     Reset,
    //     Irq,
    //     Brk,
    // }

    #[derive(Debug, Clone)]
    pub struct Register {
        pub a: i8,
        pub x: i8,
        pub y: i8,
        pub s: u16,
        pub p: P,
        pub pc: u16,
    }

    impl Register {
        pub fn new() -> Self {
            Self {
                a: 0,
                x: 0,
                y: 0,
                s: 255,
                p: P::new(),
                pc: 0,
            }
        }

        pub fn set_pc(&mut self) {
            let x = self.x as u16;
            let y = self.y as u16;
            self.pc = x + (y << 8);
        }

        pub fn set_s(&mut self, n: u8) {
            let h = 1 << 8;
            self.s = (n as u16) + h;
        }
    }

    #[derive(Debug, Clone)]
    pub struct P {
        pub carry: bool,
        pub zero: bool,
        pub interrupt: bool,
        pub decimal: bool,
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
                interrupt: false,
                decimal: false,
                break_mode: true,
                reserved: 0,
                overflow: false,
                negative: false,
            }
        }

        pub fn bool_to_n(&mut self, b: bool) -> u8 {
            match b {
                true => 1,
                false => 0,
            }
        }

        pub fn s_to_bool(&mut self, n: u32) -> bool {
            match n {
                1 => true,
                0 => false,
                _ => unimplemented!(),
            }
        }

        pub fn set(&mut self, n: u8) {
            let s = format!("{:08b}", n);
            fn chars_nth(s: &str, n: usize) -> u32 {
                s.chars().nth(n).unwrap().to_digit(2).unwrap()
            }

            self.carry = self.s_to_bool(chars_nth(&s, 7));
            self.zero = self.s_to_bool(chars_nth(&s, 6));
            self.interrupt = self.s_to_bool(chars_nth(&s, 5));
            self.decimal = self.s_to_bool(chars_nth(&s, 4));
            self.break_mode = self.s_to_bool(chars_nth(&s, 3));
            self.reserved = match chars_nth(&s, 2) {
                1 => 1,
                0 => 0,
                _ => unimplemented!(),
            };
            self.overflow = self.s_to_bool(chars_nth(&s, 1));
            self.negative = self.s_to_bool(chars_nth(&s, 0));
        }

        pub fn to_n(&mut self) -> u8 {
            let mut n = 0;
            n += self.bool_to_n(self.carry) << 7;
            n += self.bool_to_n(self.zero) << 6;
            n += self.bool_to_n(self.interrupt) << 5;
            n += self.bool_to_n(self.decimal) << 4;
            n += self.bool_to_n(self.break_mode) << 3;
            n += self.reserved << 2;
            n += self.bool_to_n(self.overflow) << 1;
            n += self.bool_to_n(self.negative);
            n
        }
    }
}

mod emurator {
    pub const SQUARE_SIZE: u32 = 8;
    pub const PLAYGROUND_WIDTH: u32 = 32;
    pub const PLAYGROUND_HEIGHT: u32 = 30;
    pub const NES_FILE: &str = "hello-world.nes";
}

pub fn main() -> Result<(), String> {
    let nes = nes::Nes::new();
    // println!("{:#?}", nes);
    let mut cpu = cpu::Cpu::new();
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
                            (i + (n / PLAYGROUND_WIDTH) * SQUARE_SIZE) as i32,
                            SQUARE_SIZE,
                            SQUARE_SIZE,
                        ),
                    )?;
                }
            }
        }

        canvas.present();
        cpu.read_ope();
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
