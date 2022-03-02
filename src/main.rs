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
            self.read(0xFFFC, 0xFFFD)
        }

        pub fn read(&mut self, b: u16, u: u16) {
            self.register.x = self.map.addr(b);
            self.register.y = self.map.addr(u);
            self.register.set_pc();
        }
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
                0x0800..=0x1FFF => self.wram_mirror[(n - 0x07FF) as usize],
                0x2000..=0x2007 => self.ppu_register[(n - 0x1FFF) as usize],
                0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2007) as usize],
                0x4000..=0x401F => self.apu_pad[(n - 0x3FFF) as usize],
                0x4020..=0x5FFF => self.erom[(n - 0x401F) as usize],
                0x6000..=0x7FFF => self.eram[(n - 0x5FFF) as usize],
                0x8000..=0xBFFF => self.prg_rom1[(n - 0x7FFF) as usize],
                0xC000..=0xFFFF => self.prg_rom2[(n - 0xBFFF) as usize],
            }
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
        pub pc: u8,
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
            let x = self.x;
            let y = self.y;
            self.pc = x + (y << 2);
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
    cpu.reset();
    {
        let prgs = &nes.header.info.prg_rom;
        if prgs.len() != 0x8000 {
            unimplemented!("prg_rom lengh is not 0x8000!");
        }
        for (i, n) in prgs.iter().enumerate() {
            match i {
                0x0000..=0x3FFF => cpu.map.prg_rom1[i] = *n,
                0x4000..=0x8000 => {
                    let i = i - 0x4000;
                    cpu.map.prg_rom2[i] = *n;
                }
                _ => unreachable!(),
            }
        }
    }

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
