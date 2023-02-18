pub mod configure;
use std::env;

use crate::apu::*;
use sdl2::audio::AudioDevice;
use sdl2::audio::AudioSpecDesired;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::texture::TextureBuffer;
use crate::nes::*;
use crate::ppu::oam::SpriteInfo;
use crate::util::*;
use rustc_hash::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Emulator<
    const TILE_COUNTS_ON_WIDTH: u32,
    const WINDOW_HEIGHT: u32,
    const WINDOW_WIDTH: u32,
    const PPU_DRAW_LINE_CYCLE: u16,
    const VBLANK_LINE: u16,
    const TOTAL_LINE: u16,
    const VISIBLE_LINES: u16,
    const APU_UPDATE_CYCLE: u16,
> {
    pub cpu: CPU,
    pub ppu_cycle: u16,
    pub apu_cycle: u16,
    pub drawing_line: u16,
    pub sdl: Sdl,
    canvas: Canvas<Window>,
    texture_buffer: TextureBuffer<TILE_COUNTS_ON_WIDTH>,
    pub pad_data: u16,
    pub audio_device_pulse1: AudioDevice<Pulse>,
    pub audio_device_pulse2: AudioDevice<Pulse>,
}

impl<
        const TILE_COUNTS_ON_WIDTH: u32,
        const WINDOW_HEIGHT: u32,
        const WINDOW_WIDTH: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VISIBLE_LINES: u16,
        const APU_UPDATE_CYCLE: u16,
    >
    Emulator<
        TILE_COUNTS_ON_WIDTH,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VISIBLE_LINES,
        APU_UPDATE_CYCLE,
    >
{
    pub fn new(nes: &Nes) -> Self {
        let mut cpu = CPU::new(nes);
        cpu.prepare_operators();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("fc2: render nes sprites", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_buffer = TextureBuffer::new();

        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: None,
            channels: None,
            samples: None,
        };

        let default_audio = |desired_spec: &mut AudioSpecDesired| {
            audio_subsystem.open_playback(None, desired_spec, |_spec| Pulse::new())
        };

        let audio_device_pulse1 = default_audio(&mut desired_spec.clone()).unwrap();
        let audio_device_pulse2 = default_audio(&mut desired_spec.clone()).unwrap();

        audio_device_pulse1.resume();
        audio_device_pulse2.resume();

        Self {
            cpu,
            ppu_cycle: 0,
            apu_cycle: 0,
            drawing_line: 0,
            sdl: sdl_context,
            canvas,
            texture_buffer,
            pad_data: 0,
            audio_device_pulse1,
            audio_device_pulse2,
        }
    }

    pub fn startup(&mut self) {
        let pc = self.cpu.get_pc() as u8;
        self.cpu.push_stack(pc);
        let p = self.cpu.get_p();
        self.cpu.push_stack(p);
        self.cpu.reset();
    }

    fn inc_apu_cycle(&mut self) {
        self.apu_cycle += self.cpu.cycle as u16;
    }

    fn inc_ppu_cycle(&mut self) {
        self.ppu_cycle += (self.cpu.cycle * 3) as u16;
    }

    fn update_texture_buffer(&mut self, texture: &mut Texture) -> Result<(), String> {
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for (key, value) in self.texture_buffer.buffer.iter().enumerate() {
                buffer[key] = *value;
            }
        })?;

        Ok(())
    }

    pub fn render_all_sprites(&mut self, sprites_num: u32) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;
        let ppu_map = &mut self.cpu.bus.ppu.map;

        for n in 0..sprites_num {
            for i in 0..8 {
                let sprite_row_line = ppu_map.addr((n * 0x10) as u16 + i);
                let sprite_high_line = ppu_map.addr((n * 0x10) as u16 + i + 0x8);
                for j in 0..8 {
                    let (idx, x, y) = {
                        let idx = {
                            let r = ((sprite_row_line & (0b1 << (7 - j))) != 0) as u16;
                            let h = ((sprite_high_line & (0b1 << (7 - j))) != 0) as u16;
                            (h << 1 | r) as usize
                        };
                        let x = j as u32 + (n % TILE_COUNTS_ON_WIDTH) * 8;
                        let y = i as u32 + (n / TILE_COUNTS_ON_WIDTH) * 8;
                        (idx, x, y)
                    };
                    self.texture_buffer.insert_color(x as u8, y as u8, idx);
                }
            }
        }

        self.update_texture_buffer(&mut texture)?;

        self.canvas.clear();
        self.canvas
            .copy(&texture, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_WIDTH))?;
        self.canvas.present();

        'show_sprites: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'show_sprites,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn reference_keycode(&mut self, keycode: Keycode) -> u16 {
        match keycode {
            Keycode::X => 0b00000001,
            Keycode::Z => 0b00000010,
            Keycode::Space => 0b00000100,
            Keycode::Return => 0b00001000,
            Keycode::Up => 0b00010000,
            Keycode::Down => 0b00100000,
            Keycode::Left => 0b01000000,
            Keycode::Right => 0b10000000,
            Keycode::N => 0b00000010 << 8,
            Keycode::M => 0b00000001 << 8,
            Keycode::Comma => 0b00000100 << 8,
            Keycode::Semicolon => 0b00001000 << 8,
            Keycode::I => 0b00010000 << 8,
            Keycode::J => 0b01000000 << 8,
            Keycode::L => 0b10000000 << 8,
            Keycode::K => 0b00100000 << 8,
            _ => 0,
        }
    }

    fn save_state(&self) {
        let file_path: Vec<String> = env::args().collect();

        let file_name = Path::new(&file_path[file_path.len() - 1])
            .to_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .nth(0)
            .unwrap();
        let mut file = File::create(format!("saves/{}_save.json", file_name)).unwrap();
        let serialized = serde_json::to_string(&self.cpu).unwrap();
        file.write_fmt(format_args!("{}", serialized)).unwrap();
    }

    fn load_state(&mut self) {
        let file_path: Vec<String> = env::args().collect();
        let file_name = Path::new(&file_path[file_path.len() - 1])
            .to_str()
            .unwrap()
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .nth(0)
            .unwrap();

        match File::open(format!("saves/{}_save.json", file_name)) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();
                let cpu: CPU = serde_json::from_str(&contents).unwrap();
                self.cpu = cpu;
            }
            _ => (),
        }
    }

    fn handle_keyboard(&mut self, event_pump: &mut EventPump) -> Option<()> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return None,
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => self.save_state(),
                Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => self.load_state(),
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    self.pad_data |= self.reference_keycode(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    self.pad_data &= !self.reference_keycode(key);
                }
                _ => {}
            }
        }
        self.cpu.bus.controller_polling_data = self.pad_data;

        Some(())
    }

    pub fn main_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;
        'running: loop {
            match self.handle_keyboard(&mut event_pump) {
                None => break 'running,
                _ => (),
            }
            self.run(&mut texture)?;
        }

        Ok(())
    }

    fn enable_render_bottom(&self) -> bool {
        self.drawing_line >= 8
    }

    fn insert_sprite_behind_background_for_big_size(&mut self) -> Result<(), String> {
        self.insert_sprites_for_big_top()?;
        if self.enable_render_bottom() {
            self.set_secondary_oam_behind_background_on_bottom();
            self.insert_sprites_for_big_bottom()?;
        };
        Ok(())
    }

    fn insert_sprite_front_of_background_for_big_size(&mut self) -> Result<(), String> {
        self.insert_sprites_for_big_top()?;
        if self.enable_render_bottom() {
            self.set_secondary_oam_front_of_background_on_bottom();
            self.insert_sprites_for_big_bottom()?;
        };
        Ok(())
    }

    fn insert_sprites_behinds_background(&mut self) -> Result<(), String> {
        if self.cpu.bus.cpu_bus.ppu_register.ppu_mask.show_sprites {
            self.set_secondary_oam_behind_background_for_nomal();
            if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.for_big() {
                self.insert_sprite_behind_background_for_big_size()?;
            } else {
                self.insert_sprites_for_normal_size()?;
            }
        }
        Ok(())
    }

    fn insert_sprites_front_of_background(&mut self) -> Result<(), String> {
        if self.cpu.bus.cpu_bus.ppu_register.ppu_mask.show_sprites {
            self.set_secondary_oam_front_of_background_for_nomal();
            if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.for_big() {
                self.insert_sprite_front_of_background_for_big_size()?;
            } else {
                self.insert_sprites_for_normal_size()?;
            }
        }
        Ok(())
    }

    fn draw_line(&mut self, texture: &mut Texture) -> Result<(), String> {
        self.update_texture_buffer(texture)?;
        self.canvas
            .copy(&texture, None, Rect::new(0, 0, WINDOW_WIDTH, WINDOW_WIDTH))?;
        self.canvas.present();

        Ok(())
    }

    fn cpu_update(&mut self) {
        let now = Instant::now();
        self.cpu.ex_ope();
        while now.elapsed().as_nanos() < 1055 {}
    }

    fn update_pulse_audio_devices(&mut self) {
        let update_pulse_audio_device =
            |frame_counter: &FrameCounter,
             is_enable: &mut bool,
             audio_device: &mut AudioDevice<Pulse>,
             pulse: &mut Pulse| {
                let mut lock = audio_device.lock();
                (*lock).call_back_duty_buf.push(pulse.duty);
                let envelope_count = frame_counter.get_envelop_count() as u16;
                if *is_enable && pulse.length_counter > 0 && pulse.timer >= 8 {
                    (*lock).clock_count += 1;
                    if (*lock).clock_count >= envelope_count {
                        (*lock).clock_count -= envelope_count;
                        if !pulse.counter_halt {
                            pulse.envelope_and_liner_counter += 1;
                            match frame_counter.mode {
                                FrameMode::_4STEP => {
                                    if pulse.envelope_and_liner_counter == 2
                                        || pulse.envelope_and_liner_counter == 4
                                    {
                                        pulse.sweep.update(&mut pulse.timer);
                                    }
                                    if pulse.envelope_and_liner_counter > 4 {
                                        pulse.envelope_and_liner_counter = 0;
                                        if pulse.length_counter <= 0 {
                                            pulse.length_counter = 0;
                                            *is_enable = false;
                                        } else {
                                            pulse.length_counter -= 1;
                                        };
                                    }
                                }
                                FrameMode::_5STEP => {
                                    if pulse.envelope_and_liner_counter == 2
                                        || pulse.envelope_and_liner_counter == 5
                                    {
                                        pulse.sweep.update(&mut pulse.timer);
                                    }
                                    if pulse.envelope_and_liner_counter > 5 {
                                        pulse.envelope_and_liner_counter = 0;
                                        if pulse.length_counter <= 0 {
                                            pulse.length_counter = 0;
                                            *is_enable = false;
                                        } else {
                                            pulse.length_counter -= 1;
                                        };
                                    }
                                }
                            }
                        }
                        pulse.current_volume = pulse.get_volume();
                        pulse.current_phase_inc =
                            (1780000.0 / ((16.0 * pulse.timer as f32) + 1.0)) / 44100 as f32;
                    }
                } else {
                    pulse.current_volume = 0;
                    pulse.current_phase_inc = 0.0;
                };
                (*lock).call_back_volume_buf.push(pulse.current_volume);
                (*lock)
                    .call_back_phase_inc_buf
                    .push(pulse.current_phase_inc);
                (*lock).call_back_duty_buf.push(pulse.duty);
            };

        update_pulse_audio_device(
            &self.cpu.bus.apu.frame_counter,
            &mut self.cpu.bus.apu.channel_controller.enable_pulse1,
            &mut self.audio_device_pulse1,
            &mut self.cpu.bus.apu.pulse1,
        );

        update_pulse_audio_device(
            &self.cpu.bus.apu.frame_counter,
            &mut self.cpu.bus.apu.channel_controller.enable_pulse2,
            &mut self.audio_device_pulse2,
            &mut self.cpu.bus.apu.pulse2,
        );
    }

    fn apu_update(&mut self) {
        self.inc_apu_cycle();
        while self.apu_cycle >= APU_UPDATE_CYCLE {
            self.apu_cycle -= APU_UPDATE_CYCLE;
            self.update_pulse_audio_devices();
        }
    }

    fn ppu_update(&mut self, texture: &mut Texture) -> Result<(), String> {
        self.inc_ppu_cycle();
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VISIBLE_LINES {
                self.insert_sprites_behinds_background()?;
                self.insert_background_line()?;
                self.insert_sprites_front_of_background()?;
            }
            if self.drawing_line == TOTAL_LINE {
                self.draw_line(texture)?;
                self.drawing_line = 0;
            } else {
                self.drawing_line += 1;
            }
            if self.is_just_in_vblank_line() {
                self.cpu.bus.cpu_bus.ppu_register.ppu_status.in_vlank = true;
                if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.gen_nmi {
                    self.cpu.interrupt(Interrupt::Nmi);
                }
            } else if self.drawing_line == 0 {
                self.cpu.bus.cpu_bus.ppu_register.ppu_status.in_vlank = false;
                self.cpu
                    .bus
                    .cpu_bus
                    .ppu_register
                    .ppu_status
                    .false_sprite_zero_hit();
                self.cpu.set_interrupt(false);
            }
        }

        Ok(())
    }

    fn run(&mut self, texture: &mut Texture) -> Result<(), String> {
        self.cpu_update();
        self.apu_update();
        self.ppu_update(texture)?;
        self.cpu.clear_cycle();
        Ok(())
    }

    fn set_secondary_oam_behind_background_for_nomal(&mut self) {
        let behind_background = true;
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8, behind_background);
    }

    fn set_secondary_oam_front_of_background_for_nomal(&mut self) {
        let behind_background = false;
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8, behind_background);
    }

    fn set_secondary_oam_behind_background_on_bottom(&mut self) {
        let behind_background = true;
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8 - 8, behind_background);
    }

    fn set_secondary_oam_front_of_background_on_bottom(&mut self) {
        let behind_background = false;
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8 - 8, behind_background);
    }

    pub fn refers_base_nametable(&self) -> (bool, bool) {
        let base_name_table_addr = (self
            .cpu
            .bus
            .cpu_bus
            .ppu_register
            .internal_registers
            .temporary_vram
            & 0b0000110000000000)
            >> 10;
        match base_name_table_addr {
            0b00 => (false, false),
            0b01 => (true, false),
            0b10 => (false, true),
            0b11 => (true, true),
            _ => unreachable!(),
        }
    }

    fn build_base_nametable_addr(&self) -> u16 {
        let mut base_addr = 0x2000;
        let (additional_x, additional_y) = self.refers_base_nametable();

        if additional_x {
            base_addr += 0x400;
        }

        if additional_y {
            base_addr += 0x800;
        }

        base_addr
    }

    fn calc_tile_idx(&self, x: u16, y: u16, data: u16) -> u16 {
        let mut base_addr = self.build_base_nametable_addr();
        let mut x = x / 8 + data;
        let mut y = ((y + self.drawing_line) / 8) * 0x20;

        if x > 0x1F {
            x -= 0x20;
            base_addr ^= 0x400;
            if x > 0x1F {
                x -= 0x20;
                base_addr ^= 0x400;
            }
        }

        if y > 0x3A0 {
            y -= 0x3C0;
            base_addr ^= 0x800;
            if y > 0x3A0 {
                y -= 0x3C0;
                base_addr ^= 0x800;
            }
        }

        base_addr + x + y
    }

    // When over 0xFF dot scroll position, refers side nametable continuous.
    fn refers_tile_nametable(&self, scrolled_x: u16, scrolled_y: u16) -> [usize; 33] {
        let mut arr: [usize; 33] = [0; 33];
        for i in 0..33 as u16 {
            let x = self.calc_tile_idx(scrolled_x, scrolled_y, i);
            arr[i as usize] = x as usize;
        }

        arr
    }

    fn build_background_dot_info(
        &mut self,
        background_row: u8,
        background_high: u8,
        tile_idx: usize,
        shift_count: u32,
        x_per_tile: u32,
    ) -> (u16, u8, u8) {
        let idx = {
            let row_idx = (background_row & (0b1 << shift_count) != 0) as u16;
            let high_idx = (background_high & (0b1 << shift_count) != 0) as u16;
            high_idx << 1 | row_idx
        };
        let x = (x_per_tile + tile_idx as u32 * 8) as u8;
        let y = self.drawing_line as u8;
        (idx, x, y)
    }

    fn build_left_background_tile(
        &mut self,
        nametable: usize,
        tile_idx: usize,
        attr_idx: usize,
        left_x_ratio: u32,
        scrolled_y: u16,
    ) {
        let (background_row, background_high) =
            self.pick_row_high_tile_background(nametable, scrolled_y);
        for i in 0..left_x_ratio {
            let (palette_idx, x, y) = self.build_background_dot_info(
                background_row,
                background_high,
                tile_idx,
                left_x_ratio - 1 - i,
                i,
            );
            let sprite_color_idx = self.calc_background_color_idx(attr_idx, palette_idx);
            self.texture_buffer.insert_color(x, y, sprite_color_idx);
        }
    }

    fn build_right_background_tile(
        &mut self,
        nametable: usize,
        tile_idx: usize,
        attr_idx: usize,
        left_x_ratio: u32,
        right_x_ratio: u32,
        scrolled_y: u16,
    ) {
        let (background_row, background_high) =
            self.pick_row_high_tile_background(nametable, scrolled_y);
        for i in 0..right_x_ratio {
            let (palette_idx, x, y) = self.build_background_dot_info(
                background_row,
                background_high,
                tile_idx,
                7 - i,
                left_x_ratio + i,
            );
            let sprite_color_idx = self.calc_background_color_idx(attr_idx, palette_idx);
            self.texture_buffer.insert_color(x, y, sprite_color_idx);
        }
    }

    fn calc_relative_addr_with_base_nametable(&self, nametable: usize) -> usize {
        let addr = match nametable {
            0x2000..=0x23BF => nametable - 0x2000,
            0x2400..=0x27BF => nametable - 0x2400,
            0x2800..=0x2BBF => nametable - 0x2800,
            0x2C00..=0x2FBF => nametable - 0x2C00,
            _ => unreachable!(),
        };
        addr & 0xFFF
    }

    fn pick_attr_base_addr(&mut self, nametable: usize) -> usize {
        match nametable {
            0x2000..=0x23BF => 0x23C0,
            0x2400..=0x27BF => 0x27C0,
            0x2800..=0x2BBF => 0x2BC0,
            0x2C00..=0x2FBF => 0x2FC0,
            _ => unreachable!(),
        }
    }

    fn build_attr_idx(&mut self, nametable: usize) -> usize {
        let corner_idx = (nametable & 0x1F) / 4;
        let addr_relative_with_base_nametable =
            self.calc_relative_addr_with_base_nametable(nametable);

        let relative_idx =
            (((addr_relative_with_base_nametable / 0x40) % 2) * 2) + ((nametable & 0x1F) / 2) % 2;
        let belongs_attr_idx = self.pick_attr_base_addr(nametable)
            + (addr_relative_with_base_nametable / 0x80) * 8
            + corner_idx;
        let belongs_palette = self.cpu.bus.ppu.map.addr(belongs_attr_idx as u16) as usize;
        ((belongs_palette & (0b11 << relative_idx * 2)) >> (relative_idx * 2)) * 4
    }

    fn build_background_tiles(
        &mut self,
        tile_nametables: [usize; 33],
        scrolled_x: u16,
        scrolled_y: u16,
    ) {
        let (left_x_ratio, right_x_ratio) = calc_scrolled_tile_ratio(scrolled_x);
        for tile_idx in 0..TILE_COUNTS_ON_WIDTH as usize {
            let (left_nametable, right_nametable) =
                { (tile_nametables[tile_idx], tile_nametables[tile_idx + 1]) };
            let (left_attr_idx, right_attr_idx) = (
                self.build_attr_idx(left_nametable),
                self.build_attr_idx(right_nametable),
            );

            self.build_left_background_tile(
                left_nametable,
                tile_idx,
                left_attr_idx,
                left_x_ratio,
                scrolled_y,
            );
            self.build_right_background_tile(
                right_nametable,
                tile_idx,
                right_attr_idx,
                left_x_ratio,
                right_x_ratio,
                scrolled_y,
            );
        }
    }

    fn calc_background_color_idx(&mut self, attr_idx: usize, pallete_idx: u16) -> usize {
        let mut sprite_color_idx =
            self.cpu
                .bus
                .ppu
                .map
                .addr(0x3F00 as u16 + attr_idx as u16 + pallete_idx) as usize;
        if self.cpu.bus.cpu_bus.ppu_register.ppu_mask.gray_scale {
            sprite_color_idx &= 0b11110000;
        }
        sprite_color_idx
    }

    fn pick_row_high_tile_background(
        &mut self,
        tile_nametable: usize,
        scrolled_y: u16,
    ) -> (u8, u8) {
        let background_idx = self.cpu.bus.ppu.map.addr(tile_nametable as u16) as u16;
        let deep_idx = 0x1000
            * self
                .cpu
                .bus
                .cpu_bus
                .ppu_register
                .ppu_ctrl
                .is_deep_bk_index() as u16;

        let base_addr =
            background_idx as u16 * 0x10 + (scrolled_y + self.drawing_line) % 8 + deep_idx;
        let row = self.cpu.bus.ppu.map.addr(base_addr);
        let high = self.cpu.bus.ppu.map.addr(base_addr + 0x8);
        (row, high)
    }

    fn get_scroll_addrs(&self) -> (u16, u16) {
        let ppu_register = &self.cpu.bus.cpu_bus.ppu_register;

        let scrolled_x = {
            let l_data = if ppu_register.ppu_status.is_occured_sprite_zero_hit() {
                ppu_register.internal_registers.x_scroll
            } else {
                0
            };
            let h_data = (ppu_register.internal_registers.temporary_vram & 0b00011111) << 3;
            h_data | l_data as u16
        };

        let scrolled_y = {
            let bottom_data =
                (ppu_register.internal_registers.temporary_vram & 0b0111000000000000) >> 12;
            let middle_data =
                ((ppu_register.internal_registers.temporary_vram & 0b011100000) >> 5) << 3;
            let high_data =
                ((ppu_register.internal_registers.temporary_vram & 0b0000001100000000) >> 8) << 6;

            high_data | middle_data | bottom_data
        };
        (scrolled_x, scrolled_y)
    }

    fn insert_background_line(&mut self) -> Result<(), String> {
        let (scrolled_x, scrolled_y) = self.get_scroll_addrs();
        let tile_nametables = self.refers_tile_nametable(scrolled_x, scrolled_y);

        self.build_background_tiles(tile_nametables, scrolled_x, scrolled_y);
        Ok(())
    }

    fn is_just_in_vblank_line(&self) -> bool {
        self.drawing_line == VBLANK_LINE
    }

    pub fn set_sprites(&mut self, chars: &Vec<u8>) {
        for (i, chr) in chars.iter().enumerate() {
            self.cpu.bus.ppu.map.set(i as u16, *chr);
            if i == 0x2000 {
                return;
            }
        }
    }

    fn insert_sprites_for_big_top(&mut self) -> Result<(), String> {
        Ok(self.insert_sprites_for_big_size(false)?)
    }

    fn insert_sprites_for_big_bottom(&mut self) -> Result<(), String> {
        Ok(self.insert_sprites_for_big_size(true)?)
    }

    fn insert_sprites_for_big_size(&mut self, is_bottom: bool) -> Result<(), String> {
        let mut color_info: FxHashMap<u8, usize> = FxHashMap::default();
        for n in 0..TILE_COUNTS_ON_WIDTH * 8 {
            match self
                .cpu
                .bus
                .ppu
                .secondary_oam
                .pick_sprite_info_with_x(n as u8)
            {
                Some(SpriteInfo {
                    pos_y,
                    tile_index,
                    pos_x,
                    attr,
                }) => {
                    let relative_hight = (self.drawing_line - *pos_y as u16) % 8;
                    let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;
                    let base_addr = (((tile_index.tile_number + 1) % 2) == 0) as u16 * 0x1000
                        + ((tile_index.tile_number as u16) / 2) * 0x20
                        + relative_hight;
                    let ppu_map = &mut self.cpu.bus.ppu.map;
                    let (sprite_row, sprite_high) = {
                        if is_bottom {
                            (
                                ppu_map.addr(base_addr + 0x10),
                                ppu_map.addr(base_addr + 0x18),
                            )
                        } else {
                            (ppu_map.addr(base_addr), ppu_map.addr(base_addr + 8))
                        }
                    };
                    let pallet_base_idx = (attr.palette * 4) as usize;
                    let for_count = if is_bottom { 16 } else { 8 };
                    for i in for_count - 8..for_count {
                        let (idx, x) = {
                            if is_bottom {
                                let idx = {
                                    let r = (sprite_row & (0b1 << (15 - i)) != 0) as u16;
                                    let h = (sprite_high & (0b1 << (15 - i)) != 0) as u16;
                                    let idx = h << 1 | r;
                                    if idx == 0 {
                                        continue;
                                    }
                                    idx
                                };
                                let x = pos_x.wrapping_add(i - 8);
                                (idx, x)
                            } else {
                                let idx = {
                                    let r = (sprite_row & (0b1 << (7 - i)) != 0) as u16;
                                    let h = (sprite_high & (0b1 << (7 - i)) != 0) as u16;
                                    let idx = h << 1 | r;
                                    if idx == 0 {
                                        continue;
                                    }
                                    idx
                                };
                                let x = pos_x.wrapping_add(i);
                                (idx, x)
                            }
                        };

                        let pallet_idx = pallet_base_idx + idx as usize;
                        let mut color_idx = ppu_map.sprite_pallet[pallet_idx] as usize;
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }

                        if !self
                            .cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .is_occured_sprite_zero_hit()
                            && attr.priority
                            && pallet_idx % 4 != 0
                        {
                            self.cpu
                                .bus
                                .cpu_bus
                                .ppu_register
                                .ppu_status
                                .true_sprite_zero_hit();
                        }

                        if !color_info.contains_key(&x) {
                            color_info.insert(x, color_idx);
                        }
                    }
                }
                None => continue,
            };
        }
        self.texture_buffer
            .insert_colors(color_info, self.drawing_line as u8);
        Ok(())
    }

    fn insert_sprites_for_normal_size(&mut self) -> Result<(), String> {
        let mut color_info: FxHashMap<u8, usize> = FxHashMap::default();
        for n in 0..TILE_COUNTS_ON_WIDTH * 8 {
            match self
                .cpu
                .bus
                .ppu
                .secondary_oam
                .pick_sprite_info_with_x(n as u8)
            {
                Some(SpriteInfo {
                    pos_y,
                    tile_index,
                    pos_x,
                    attr,
                }) => {
                    let ppu_ctrl = &self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl;
                    let relative_hight = (self.drawing_line - *pos_y as u16) % 8;
                    let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;
                    let base_addr = tile_index.bank_of_tile as u16 * 0x1000
                        + ppu_ctrl.sprite_ptn_table_addr as u16 * 0x1000
                        + (tile_index.tile_number as u16) * 0x10
                        + if attr.flip_sprite_vertically {
                            7 - relative_hight
                        } else {
                            relative_hight
                        };
                    let ppu_map = &mut self.cpu.bus.ppu.map;
                    let sprite_row = ppu_map.addr(base_addr);
                    let sprite_high = ppu_map.addr(base_addr + 8);
                    let pallet_base_idx = (attr.palette * 4) as usize;
                    for i in 0..8 {
                        let (idx, x) = {
                            let idx = {
                                let i = if attr.flip_sprite_horizontally {
                                    i
                                } else {
                                    7 - i
                                };
                                let r = (sprite_row & (0b1 << i) != 0) as u16;
                                let h = (sprite_high & (0b1 << i) != 0) as u16;
                                let idx = h << 1 | r;
                                if idx == 0 {
                                    continue;
                                }
                                idx
                            };
                            let x = pos_x.wrapping_add(i);
                            (idx, x)
                        };
                        let pallet_idx = pallet_base_idx + idx as usize;
                        let mut color_idx = ppu_map.sprite_pallet[pallet_idx] as usize;
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }

                        if !self
                            .cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .is_occured_sprite_zero_hit()
                            && attr.priority
                            && pallet_idx % 4 != 0
                        {
                            self.cpu
                                .bus
                                .cpu_bus
                                .ppu_register
                                .ppu_status
                                .true_sprite_zero_hit();
                        }

                        if !color_info.contains_key(&x) {
                            color_info.insert(x, color_idx);
                        }
                    }
                }
                None => continue,
            };
        }
        self.texture_buffer
            .insert_colors(color_info, self.drawing_line as u8);
        Ok(())
    }
}
