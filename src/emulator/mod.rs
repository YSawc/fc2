pub mod configure;
pub mod texture;
use crate::bus::Mapper;
use crate::cpu::*;
use crate::emulator::texture::TextureBuffer;
use crate::nes::*;
use crate::ppu::oam::SpriteInfo;
use crate::util::*;
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
> {
    pub cpu: CPU,
    pub ppu_cycle: u16,
    pub drawing_line: u16,
    pub sdl: Sdl,
    canvas: Canvas<Window>,
    texture_buffer: TextureBuffer<TILE_COUNTS_ON_WIDTH>,
    pub pad_data: u16,
}

impl<
        const TILE_COUNTS_ON_WIDTH: u32,
        const WINDOW_HEIGHT: u32,
        const WINDOW_WIDTH: u32,
        const PPU_DRAW_LINE_CYCLE: u16,
        const VBLANK_LINE: u16,
        const TOTAL_LINE: u16,
        const VISIBLE_LINES: u16,
    >
    Emulator<
        TILE_COUNTS_ON_WIDTH,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
        PPU_DRAW_LINE_CYCLE,
        VBLANK_LINE,
        TOTAL_LINE,
        VISIBLE_LINES,
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

        Self {
            cpu,
            ppu_cycle: 0,
            drawing_line: 0,
            sdl: sdl_context,
            canvas,
            texture_buffer,
            pad_data: 0,
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

    fn handle_keyboard(&mut self, event_pump: &mut EventPump) -> Result<Option<()>, String> {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(None),
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

        Ok(Some(()))
    }

    pub fn main_loop(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl.event_pump()?;
        let texture_creator: TextureCreator<_> = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;
        'running: loop {
            match self.handle_keyboard(&mut event_pump)? {
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

    fn run(&mut self, texture: &mut Texture) -> Result<(), String> {
        self.cpu.ex_ope();
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

    fn calc_tile_idx(&self, x: u16, y: u16, n: u16) -> u16 {
        let mut base_addr = self.build_base_nametable_addr();
        let mut x = x / 8 + n;
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
            let l = if ppu_register.ppu_status.is_occured_sprite_zero_hit() {
                ppu_register.internal_registers.x_scroll
            } else {
                0
            };
            let h = (ppu_register.internal_registers.temporary_vram & 0b00011111) << 3;
            h | l as u16
        };

        let scrolled_y = {
            let b = (ppu_register.internal_registers.temporary_vram & 0b0111000000000000) >> 12;
            let m = ((ppu_register.internal_registers.temporary_vram & 0b011100000) >> 5) << 3;
            let h =
                ((ppu_register.internal_registers.temporary_vram & 0b0000001100000000) >> 8) << 6;

            h | m | b
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

                        self.texture_buffer
                            .insert_color(x as u8, y as u8, color_idx);
                    }
                }
                None => continue,
            };
        }
        Ok(())
    }

    fn insert_sprites_for_normal_size(&mut self) -> Result<(), String> {
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
