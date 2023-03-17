use crate::bus::Mapper;
use serde::{Deserialize, Serialize};
use serde_with::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controller {
    d0: u8,
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    open_bus: [u8; 0x0003],
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    pub fn new() -> Self {
        Self {
            d0: 0,
            d1: 0,
            d2: 0,
            d3: 0,
            d4: 0,
            open_bus: [0; 0x0003],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalRegisters {
    pub current_vram: u16,
    pub temporary_vram: u16,
    pub x_scroll: u8,
    latch_flag: bool,
}

impl Default for InternalRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalRegisters {
    fn new() -> Self {
        let current_vram = 0;
        let temporary_vram = 0;
        let x_scroll = 0;
        let latch_flag = false;

        Self {
            current_vram,
            temporary_vram,
            x_scroll,
            latch_flag,
        }
    }

    fn on_latch(&mut self) {
        self.latch_flag = true;
    }

    pub fn off_latch(&mut self) {
        self.latch_flag = false;
    }

    fn toggle_latch(&mut self) {
        match self.latch_flag {
            true => self.off_latch(),
            false => self.on_latch(),
        }
    }

    fn inc_vram_addr(&mut self, data: u16) {
        self.current_vram += data;
    }

    fn copy_current_vram_to_tempolary_vram(&mut self) {
        self.current_vram = self.temporary_vram;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuRegister {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    oam_addr: u8,
    oam_data: u8,
    ppu_data: u8,
    pub ppu_buffer: PpuBuffer,
    pub internal_registers: InternalRegisters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuCtrl {
    pub gen_nmi: bool,
    ppu_selector: bool,
    sprite_size: bool,
    bk_table_addr: bool,
    pub sprite_ptn_table_addr: bool,
    vram_increment: bool,
    pub base_name_table_addr: u8,
}

impl Default for PpuCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuCtrl {
    fn new() -> Self {
        Self {
            gen_nmi: false,
            ppu_selector: false,
            sprite_size: false,
            bk_table_addr: false,
            sprite_ptn_table_addr: false,
            vram_increment: false,
            base_name_table_addr: 0,
        }
    }

    fn set(&mut self, data: u8) {
        self.gen_nmi = (data & 0b10000000) != 0;
        self.ppu_selector = (data & 0b01000000) != 0;
        self.sprite_size = (data & 0b00100000) != 0;
        self.bk_table_addr = (data & 0b00010000) != 0;
        self.sprite_ptn_table_addr = (data & 0b00001000) != 0;
        self.vram_increment = (data & 0b00000100) != 0;
        self.base_name_table_addr = data & 0b00000011;
    }

    fn to_n(&self) -> u8 {
        let mut data = 0;
        data += self.gen_nmi as u8 * 0b10000000;
        data += self.ppu_selector as u8 * 0b01000000;
        data += self.sprite_size as u8 * 0b00100000;
        data += self.bk_table_addr as u8 * 0b00010000;
        data += self.sprite_ptn_table_addr as u8 * 0b00001000;
        data += self.vram_increment as u8 * 0b00000100;
        data += self.base_name_table_addr;
        data
    }

    pub fn is_deep_bk_index(&self) -> bool {
        self.bk_table_addr
    }

    fn increment_vram_num(&mut self) -> u16 {
        match self.vram_increment {
            false => 1,
            true => 32,
        }
    }

    pub fn for_big(&self) -> bool {
        self.sprite_size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuMask {
    emf_blue: bool,
    emf_green: bool,
    emf_red: bool,
    pub show_sprites: bool,
    show_background: bool,
    show_sprites_in_leftmost: bool,
    show_background_in_leftmost: bool,
    gray_scale: bool,
}

impl Default for PpuMask {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuMask {
    fn new() -> Self {
        Self {
            emf_blue: false,
            emf_green: false,
            emf_red: false,
            show_sprites: false,
            show_background: false,
            show_sprites_in_leftmost: false,
            show_background_in_leftmost: false,
            gray_scale: false,
        }
    }

    fn set(&mut self, data: u8) {
        self.emf_blue = (data & 0b10000000) != 0;
        self.emf_green = (data & 0b01000000) != 0;
        self.emf_red = (data & 0b00100000) != 0;
        self.show_sprites = (data & 0b00010000) != 0;
        self.show_background = (data & 0b00001000) != 0;
        self.show_sprites_in_leftmost = (data & 0b00000100) != 0;
        self.show_background_in_leftmost = (data & 0b00000010) != 0;
        self.gray_scale = (data & 0b00000001) != 0;
    }

    fn to_n(&self) -> u8 {
        let mut data = 0;
        data += self.emf_blue as u8 * 0b10000000;
        data += self.emf_green as u8 * 0b01000000;
        data += self.emf_red as u8 * 0b00100000;
        data += self.show_sprites as u8 * 0b00010000;
        data += self.show_background as u8 * 0b00001000;
        data += self.show_sprites_in_leftmost as u8 * 0b00000100;
        data += self.show_background_in_leftmost as u8 * 0b00000010;
        data += self.gray_scale as u8;
        data
    }

    pub fn apply_gray_scale(&self, color_idx: &mut usize) {
        if self.gray_scale {
            *color_idx &= 0b11110000
        }
    }

    pub fn is_show_sprites(&self) -> bool {
        self.show_sprites
    }

    pub fn is_show_background(&self) -> bool {
        self.show_background
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuStatus {
    pub in_vlank: bool,
    sprite_zero_hit: bool,
    sprite_evoluation: bool,
    bus: u8,
    pub line_occured_sprite_zero_hit: u16,
}

impl Default for PpuStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuStatus {
    fn new() -> Self {
        Self {
            in_vlank: false,
            sprite_zero_hit: false,
            sprite_evoluation: false,
            bus: 0,
            line_occured_sprite_zero_hit: 0,
        }
    }
    fn set(&mut self, data: u8) {
        self.in_vlank = (data & 0b10000000) != 0;
        self.sprite_zero_hit = (data & 0b01000000) != 0;
        self.sprite_evoluation = (data & 0b00100000) != 0;
        self.bus = data & 0b00001111;
    }

    fn to_n(&self) -> u8 {
        let mut data = 0;
        data += self.in_vlank as u8 * 0b10000000;
        data += self.sprite_zero_hit as u8 * 0b01000000;
        data += self.sprite_evoluation as u8 * 0b00100000;
        data += self.bus;
        data
    }

    pub fn is_occured_sprite_zero_hit(&self) -> bool {
        self.sprite_zero_hit
    }

    pub fn set_line_occured_sprite_zero_hit(&mut self, line: u16) {
        self.line_occured_sprite_zero_hit = line;
    }

    pub fn true_sprite_zero_hit(&mut self) {
        self.sprite_zero_hit = true;
    }

    pub fn false_sprite_zero_hit(&mut self) {
        self.sprite_zero_hit = false;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuBuffer {
    pub buffer: u8,
    pub vram_memory: u8,
}

impl Default for PpuBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuBuffer {
    fn new() -> Self {
        PpuBuffer {
            buffer: 0,
            vram_memory: 0,
        }
    }

    pub fn addr(&mut self) -> u8 {
        let data = self.buffer;
        self.buffer = self.vram_memory;
        data
    }

    pub fn set(&mut self, data: u8) {
        self.vram_memory = data;
    }
}

impl Default for PpuRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuRegister {
    fn new() -> Self {
        Self {
            ppu_ctrl: PpuCtrl::default(),
            ppu_mask: PpuMask::default(),
            ppu_status: PpuStatus::default(),
            oam_addr: 0,
            oam_data: 0,
            ppu_data: 0,
            ppu_buffer: PpuBuffer::default(),
            internal_registers: InternalRegisters::default(),
        }
    }

    fn set(&mut self, addr: u16, data: u8) {
        match addr {
            0x2000 => {
                self.internal_registers.temporary_vram &= 0b1111001111111111;
                self.internal_registers.temporary_vram |= (data as u16 & 0b00000011) << 10;
                self.ppu_ctrl.set(data);
            }
            0x2001 => self.ppu_mask.set(data),
            0x2002 => self.ppu_status.set(data),
            0x2003 => self.oam_addr = data,
            0x2004 => self.oam_data = data,
            0x2005 => {
                let data = data as u16;
                match self.internal_registers.latch_flag {
                    true => {
                        self.internal_registers.temporary_vram &= 0b1000110000011111;
                        let b = (data & 0b00000111) << 12;
                        let m = ((data & 0b00111000) >> 3) << 5;
                        let h = ((data & 0b11000000) >> 6) << 8;
                        self.internal_registers.temporary_vram |= h | m | b
                    }
                    false => {
                        self.internal_registers.x_scroll = 0;
                        self.internal_registers.temporary_vram &= 0b1111111111100000;
                        let b = data & 0b00000111;
                        let h = (data & 0b11111000) >> 3;
                        self.internal_registers.x_scroll = b as u8;
                        self.internal_registers.temporary_vram |= h;
                    }
                }
                self.internal_registers.toggle_latch();
            }
            0x2006 => {
                match self.internal_registers.latch_flag {
                    true => {
                        self.internal_registers.temporary_vram &= 0b111111100000000;
                        self.internal_registers.temporary_vram |= data as u16;
                        self.internal_registers
                            .copy_current_vram_to_tempolary_vram();
                    }
                    false => {
                        self.internal_registers.temporary_vram &= 0b000000011111111;
                        self.internal_registers.temporary_vram |= (data as u16 & 0b00111111) << 8;
                    }
                }
                self.internal_registers.toggle_latch();
            }
            _ => unreachable!(),
        }
    }

    fn addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ppu_ctrl.to_n(),
            0x2001 => self.ppu_mask.to_n(),
            0x2002 => self.ppu_status.to_n(),
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            _ => unreachable!(),
        }
    }

    pub fn constant_inc_vram(&mut self) {
        let data = self.ppu_ctrl.increment_vram_num();
        self.internal_registers.inc_vram_addr(data);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RP2A03 {
    oam_dma: u8,
    controller_0: Controller,
    controller_1: Controller,
    test_fanctionality: [u8; 0x0008],
}

impl Default for RP2A03 {
    fn default() -> Self {
        Self::new()
    }
}

impl RP2A03 {
    fn new() -> Self {
        Self {
            oam_dma: 0,
            controller_0: Controller::default(),
            controller_1: Controller::default(),
            test_fanctionality: [0; 0x0008],
        }
    }

    fn addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x4014 => self.oam_dma,
            0x4016 => self.controller_0.d0,
            0x4017 => self.controller_1.d0,
            _ => unreachable!(),
        }
    }

    fn set(&mut self, addr: u16, data: u8) {
        match addr {
            0x4014 => self.oam_dma = data,
            _ => unreachable!(),
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMap {
    #[serde_as(as = "[_; 0x0800]")]
    wram: [u8; 0x0800],
    #[serde_as(as = "[_; 0x1800]")]
    wram_mirror: [u8; 0x1800],
    pub ppu_register: PpuRegister,
    #[serde_as(as = "[_; 0x1FF8]")]
    ppu_register_mirror: [u8; 0x1FF8],
    rp2a03: RP2A03,
    func_apu_io: [u8; 0x0008],
    #[serde_as(as = "[_; 0x1FE0]")]
    erom: [u8; 0x1FE0],
    #[serde_as(as = "[_; 0x2000]")]
    eram: [u8; 0x2000],
    #[serde_as(as = "[_; 0x4000]")]
    pub prg_rom1: [u8; 0x4000],
    #[serde_as(as = "[_; 0x4000]")]
    prg_rom2: [u8; 0x4000],
}

impl Default for CpuMap {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuMap {
    fn new() -> Self {
        Self {
            wram: [0; 0x0800],
            wram_mirror: [0; 0x1800],
            ppu_register: PpuRegister::default(),
            ppu_register_mirror: [0; 0x1FF8],
            rp2a03: RP2A03::default(),
            func_apu_io: [0; 0x0008],
            erom: [0; 0x1FE0],
            eram: [0; 0x2000],
            prg_rom1: [0; 0x4000],
            prg_rom2: [0; 0x4000],
        }
    }

    pub fn lh_ignore_overflowing_addr(&mut self, data: u16) -> (u8, u8) {
        let h_data = data & 0xff00;
        let l_data = (data as u8).wrapping_add(1);
        let next_addr = h_data | l_data as u16;
        let l_data = self.addr(data as u16);
        let h_data = self.addr(next_addr as u16);
        (l_data, h_data)
    }

    pub fn lh_zeropage_addr(&mut self, data: u8) -> (u8, u8) {
        let next_addr_mem = data.wrapping_add(1);
        let l_data = self.addr(data as u16);
        let h_data = self.addr(next_addr_mem as u16);
        (l_data, h_data)
    }

    pub fn lh_addr(&mut self, data: u16) -> (u8, u8) {
        let l_data = self.addr(data);
        let h_data = self.addr(data + 1);
        (l_data, h_data)
    }

    pub fn hl_addr(&mut self, data: u16) -> (u8, u8) {
        let h_data = self.addr(data);
        let l_data = self.addr(data + 1);
        (h_data, l_data)
    }
}

impl Mapper for CpuMap {
    fn addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07FF => self.wram[addr as usize],
            0x0800..=0x1FFF => self.wram_mirror[(addr - 0x0800) as usize],
            0x2000..=0x2007 => self.ppu_register.addr(addr),
            0x2008..=0x3FFF => self.ppu_register_mirror[(addr - 0x2008) as usize],
            0x4000..=0x4017 => self.rp2a03.addr(addr),
            0x4018..=0x401F => self.func_apu_io[(addr - 0x4018) as usize],
            0x4020..=0x5FFF => self.erom[(addr - 0x4020) as usize],
            0x6000..=0x7FFF => self.eram[(addr - 0x6000) as usize],
            0x8000..=0xBFFF => self.prg_rom1[(addr - 0x8000) as usize],
            0xC000..=0xFFFF => self.prg_rom2[(addr - 0xC000) as usize],
        }
    }

    fn set(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x07FF => self.wram[addr as usize] = data,
            0x0800..=0x1FFF => self.wram_mirror[(addr - 0x0800) as usize] = data,
            0x2000..=0x2007 => self.ppu_register.set(addr, data),
            0x2008..=0x3FFF => self.ppu_register_mirror[(addr - 0x2008) as usize] = data,
            0x4000..=0x4015 => self.rp2a03.set(addr, data),
            0x4016..=0x4017 => unreachable!(),
            0x4018..=0x401F => self.func_apu_io[(addr - 0x4017) as usize] = data,
            0x4020..=0x5FFF => self.erom[(addr - 0x4020) as usize] = data,
            0x6000..=0x7FFF => self.eram[(addr - 0x6000) as usize] = data,
            0x8000..=0xBFFF => self.prg_rom1[(addr - 0x8000) as usize] = data,
            0xC000..=0xFFFF => self.prg_rom2[(addr - 0xC000) as usize] = data,
        };
    }
}
