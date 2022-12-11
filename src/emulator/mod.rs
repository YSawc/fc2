pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::texture::TextureBuffer;
use crate::ppu::oam::SpriteInfo;
use rustc_hash::FxHashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

pub struct Emulator<
    const PLAYGROUND_HEIGHT: u32,
    const PLAYGROUND_WIDTH: u32,
    const SQUARE_SIZE: u32,
    const SPRITE_SIZE: u32,
    const PPU_DRAW_LINE_CYCLE: u16,
    const VBLANK_LINE: u16,
    const TOTAL_LINE: u16,
    const VISIBLE_LINES: u16,
> {
    pub cpu: CPU,
    pub ppu_cycle: u16,
    pub drawing_line: u16,
    pub sdl: Sdl,
    canvas: Canvas<Window>,
    texture_buffer: TextureBuffer<PLAYGROUND_WIDTH, VISIBLE_LINES>,
}

impl<
        const PLAYGROUND_HEIGHT: u32,
        const PLAYGROUND_WIDTH: u32,
        const SQUARE_SIZE: u32,
        const SPRITE_SIZE: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VISIBLE_LINES: u16,
    > Default
    for Emulator<
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        SQUARE_SIZE,
        SPRITE_SIZE,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VISIBLE_LINES,
    >
{
    fn default() -> Self {
        Self::new()
    }
}

impl<
        const PLAYGROUND_HEIGHT: u32,
        const PLAYGROUND_WIDTH: u32,
        const SQUARE_SIZE: u32,
        const SPRITE_SIZE: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VISIBLE_LINES: u16,
    >
    Emulator<
        PLAYGROUND_HEIGHT,
        PLAYGROUND_WIDTH,
        SQUARE_SIZE,
        SPRITE_SIZE,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VISIBLE_LINES,
    >
{
    fn new() -> Self {
        let cpu = CPU::default();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "fc2: render nes sprites",
                SQUARE_SIZE * PLAYGROUND_WIDTH,
                SQUARE_SIZE * PLAYGROUND_HEIGHT,
            )
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

        Self {
            cpu,
            ppu_cycle: 0,
            drawing_line: 0,
            sdl: sdl_context,
            canvas,
            texture_buffer,
        }
    }

    pub fn startup(&mut self) {
        let pc = self.cpu.get_pc() as u8;
        self.cpu.push_stack(pc);
        let p = self.cpu.get_p();
        self.cpu.push_stack(p);
        self.cpu.reset();
    }

    fn inc_ppu_cycle(&mut self) {
        self.ppu_cycle += (self.cpu.cycle * 3) as u16;
        self.cpu.clear_cycle();
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
                        let x = j as u32 + (n % PLAYGROUND_WIDTH) * SQUARE_SIZE;
                        let y = i as u32 + (n / PLAYGROUND_WIDTH) * SQUARE_SIZE;
                        (idx, x, y)
                    };
                    self.texture_buffer.insert_color(x as u8, y as u8, idx);
                }
            }
        }
        texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for n in 0..self.texture_buffer.buffer.len() {
                buffer[n] = self.texture_buffer.buffer[n];
            }
        })?;
        self.canvas.clear();
        self.canvas
            .copy(&texture, None, Some(Rect::new(0, 0, 256, 256)))?;
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

    fn handle_keyboard(&mut self, event_pump: &EventPump) -> Option<u32> {
        let pressed_scancodes: FxHashSet<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        for scancode in pressed_scancodes.iter() {
            match *scancode {
                Keycode::Escape => return None,
                Keycode::X => self.cpu.bus.controller_polling_data |= 0b00000001,
                Keycode::Z => self.cpu.bus.controller_polling_data |= 0b00000010,
                Keycode::Space => self.cpu.bus.controller_polling_data |= 0b00000100,
                Keycode::Return => self.cpu.bus.controller_polling_data |= 0b00001000,
                Keycode::Up => self.cpu.bus.controller_polling_data |= 0b00010000,
                Keycode::Down => self.cpu.bus.controller_polling_data |= 0b00100000,
                Keycode::Left => self.cpu.bus.controller_polling_data |= 0b01000000,
                Keycode::Right => self.cpu.bus.controller_polling_data |= 0b10000000,
                Keycode::N => self.cpu.bus.controller_polling_data |= 0b00000010 << 8,
                Keycode::M => self.cpu.bus.controller_polling_data |= 0b00000001 << 8,
                Keycode::Comma => self.cpu.bus.controller_polling_data |= 0b00000100 << 8,
                Keycode::Semicolon => self.cpu.bus.controller_polling_data |= 0b00001000 << 8,
                Keycode::I => self.cpu.bus.controller_polling_data |= 0b00010000 << 8,
                Keycode::J => self.cpu.bus.controller_polling_data |= 0b01000000 << 8,
                Keycode::L => self.cpu.bus.controller_polling_data |= 0b10000000 << 8,
                Keycode::K => self.cpu.bus.controller_polling_data |= 0b00100000 << 8,
                _ => (),
            };
        }
        Some(1)
    }

    pub fn main_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;

        'running: loop {
            match self.handle_keyboard(&event_pump) {
                Some(_) => (),
                None => break 'running,
            }
            event_pump.poll_event();

            self.run(&mut texture)?;
        }

        Ok(())
    }

    fn enable_render_bottom(&self) -> bool {
        self.drawing_line >= 8
    }

    fn draw_sprite_for_big_size(&mut self) -> Result<(), String> {
        self.draw_sprites_for_big_top()?;
        if self.enable_render_bottom() {
            self.set_secondary_oam_for_bottom();
            self.draw_sprites_for_big_bottom()?;
        };
        Ok(())
    }

    fn run(&mut self, texture: &mut Texture) -> Result<(), String> {
        self.cpu.ex_ope();
        self.inc_ppu_cycle();
        if self.ppu_cycle >= PPU_DRAW_LINE_CYCLE {
            self.ppu_cycle -= PPU_DRAW_LINE_CYCLE;
            if self.drawing_line < VISIBLE_LINES {
                self.draw_background()?;
                if self.cpu.bus.cpu_bus.ppu_register.ppu_mask.show_sprites {
                    self.set_secondary_oam_for_nomal();
                    if self.cpu.bus.cpu_bus.ppu_register.ppu_ctrl.for_big() {
                        self.draw_sprite_for_big_size()?;
                    } else {
                        self.draw_sprites_for_normal_size()?;
                    }
                }
            }
            if self.drawing_line == TOTAL_LINE {
                texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                    for n in 0..self.texture_buffer.buffer.len() {
                        buffer[n] = self.texture_buffer.buffer[n];
                    }
                })?;
                self.canvas.clear();
                self.canvas
                    .copy(&texture, None, Some(Rect::new(0, 0, 256, 256)))?;
                self.canvas.present();
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

    fn set_secondary_oam_for_nomal(&mut self) {
        self.cpu.bus.ppu.set_secondary_oam(self.drawing_line as u8);
    }

    fn set_secondary_oam_for_bottom(&mut self) {
        self.cpu
            .bus
            .ppu
            .set_secondary_oam(self.drawing_line as u8 - 8);
    }

    fn draw_background(&mut self) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH as u16 {
            let i1 = self.drawing_line / 8;
            let i2 = self.drawing_line % 8;
            let ppu_map = &mut self.cpu.bus.ppu.map;
            let ppu_mask = &self.cpu.bus.cpu_bus.ppu_register.ppu_mask;

            let attr_idx = {
                let relative_idx = (((self.drawing_line / 16) % 2) * 2) + (n / 2) % 2;
                let n = 0x23c0 + { (self.drawing_line / 32) * 8 } + (n / 4);
                let m = ppu_map.addr(n);
                (((m & (0b11 << relative_idx * 2)) >> (relative_idx * 2)) as usize) * 4
            };

            let deep_idx = 0x1000
                * self
                    .cpu
                    .bus
                    .cpu_bus
                    .ppu_register
                    .ppu_ctrl
                    .is_deep_bk_index() as u16;
            let background_idx = ppu_map.addr((0x2000 + i1 * PLAYGROUND_WIDTH as u16) + n);
            let base_addr = (background_idx as u16 * 0x10) as u16 + i2 + deep_idx;
            let background_row = ppu_map.addr(base_addr);
            let background_high = ppu_map.addr(base_addr + 0x8);
            for j in 0..8 {
                let (idx, x, y) = {
                    let idx = {
                        let row_idx = (background_row & (0b1 << (7 - j)) != 0) as u16;
                        let high_idx = (background_high & (0b1 << (7 - j)) != 0) as u16;
                        high_idx << 1 | row_idx
                    };
                    let x = j + n as u32 * SQUARE_SIZE;
                    let y = self.drawing_line;
                    (idx, x, y)
                };
                let mut sprite_color_idx =
                    ppu_map.addr(0x3F00 as u16 + attr_idx as u16 + idx as u16) as usize;
                if ppu_mask.gray_scale {
                    sprite_color_idx &= 0b11110000;
                }
                self.texture_buffer
                    .insert_color(x as u8, y as u8, sprite_color_idx);
            }
        }

        Ok(())
    }

    fn is_just_in_vblank_line(&self) -> bool {
        self.drawing_line == VBLANK_LINE
    }

    pub fn set_sprites(&mut self, chars: &Vec<u8>) {
        if chars.len() > 0x2000 {
            unimplemented!()
        }
        for (i, chr) in chars.iter().enumerate() {
            self.cpu.bus.ppu.map.set(i as u16, *chr);
        }
    }

    fn draw_sprites_for_big_top(&mut self) -> Result<(), String> {
        Ok(self.draw_sprites_for_big_size(false)?)
    }

    fn draw_sprites_for_big_bottom(&mut self) -> Result<(), String> {
        Ok(self.draw_sprites_for_big_size(true)?)
    }

    fn draw_sprites_for_big_size(&mut self, is_bottom: bool) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH * 8 {
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
                    if attr.priority
                        && !self
                            .cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .is_occured_sprite_zero_hit()
                    {
                        self.cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .true_sprite_zero_hit();
                        continue;
                    }
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
                        let (idx, x, y) = {
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
                                let y = self.drawing_line;
                                (idx, x, y)
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
                                let y = self.drawing_line;
                                (idx, x, y)
                            }
                        };

                        let mut color_idx =
                            ppu_map.sprite_pallet[pallet_base_idx + idx as usize] as usize;
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }

                        self.texture_buffer
                            .insert_color(x as u8, y as u8, color_idx);
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }

    fn draw_sprites_for_normal_size(&mut self) -> Result<(), String> {
        for n in 0..PLAYGROUND_WIDTH * 8 {
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
                    if attr.priority
                        && !self
                            .cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .is_occured_sprite_zero_hit()
                    {
                        self.cpu
                            .bus
                            .cpu_bus
                            .ppu_register
                            .ppu_status
                            .true_sprite_zero_hit();
                        continue;
                    }
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
                        let (idx, x, y) = {
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
                            let y = self.drawing_line;
                            (idx, x, y)
                        };
                        let mut color_idx =
                            ppu_map.sprite_pallet[pallet_base_idx + idx as usize] as usize;
                        if ppu_mask.gray_scale {
                            color_idx &= 0b11110000;
                        }
                        self.texture_buffer
                            .insert_color(x as u8, y as u8, color_idx);
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }
}
