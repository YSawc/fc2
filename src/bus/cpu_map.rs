use crate::bus::Mapper;
use serde::{Deserialize, Serialize};
use serde_with::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Controller {
    pub d0: u8,
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
    pub d4: u8,
    pub open_bus: [u8; 0x0003],
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
    pub latch_flag: bool,
}

impl Default for InternalRegisters {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalRegisters {
    pub fn new() -> Self {
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

    pub fn on_latch(&mut self) {
        self.latch_flag = true;
    }

    pub fn off_latch(&mut self) {
        self.latch_flag = false;
    }

    pub fn toggle_latch(&mut self) {
        match self.latch_flag {
            true => self.off_latch(),
            false => self.on_latch(),
        }
    }

    pub fn inc_vram_addr(&mut self, n: u16) {
        self.current_vram += n;
    }

    pub fn copy_current_vram_to_tempolary_vram(&mut self) {
        self.current_vram = self.temporary_vram;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuRegister {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_data: u8,
    pub ppu_buffer: PpuBuffer,
    pub internal_registers: InternalRegisters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuCtrl {
    pub gen_nmi: bool,
    ppu_selector: bool,
    pub sprite_size: bool,
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

    fn set(&mut self, n: u8) {
        self.gen_nmi = (n & 0b10000000) != 0;
        self.ppu_selector = (n & 0b01000000) != 0;
        self.sprite_size = (n & 0b00100000) != 0;
        self.bk_table_addr = (n & 0b00010000) != 0;
        self.sprite_ptn_table_addr = (n & 0b00001000) != 0;
        self.vram_increment = (n & 0b00000100) != 0;
        self.base_name_table_addr = n & 0b00000011;
    }

    pub fn to_n(&self) -> u8 {
        let mut n = 0;
        n += self.gen_nmi as u8 * 0b10000000;
        n += self.ppu_selector as u8 * 0b01000000;
        n += self.sprite_size as u8 * 0b00100000;
        n += self.bk_table_addr as u8 * 0b00010000;
        n += self.sprite_ptn_table_addr as u8 * 0b00001000;
        n += self.vram_increment as u8 * 0b00000100;
        n += self.base_name_table_addr;
        n
    }

    pub fn is_deep_bk_index(&self) -> bool {
        self.bk_table_addr
    }

    pub fn increment_vram_num(&mut self) -> u16 {
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
    pub gray_scale: bool,
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

    fn set(&mut self, n: u8) {
        self.emf_blue = (n & 0b10000000) != 0;
        self.emf_green = (n & 0b01000000) != 0;
        self.emf_red = (n & 0b00100000) != 0;
        self.show_sprites = (n & 0b00010000) != 0;
        self.show_background = (n & 0b00001000) != 0;
        self.show_sprites_in_leftmost = (n & 0b00000100) != 0;
        self.show_background_in_leftmost = (n & 0b00000010) != 0;
        self.gray_scale = (n & 0b00000001) != 0;
    }

    fn to_n(&self) -> u8 {
        let mut n = 0;
        n += self.emf_blue as u8 * 0b10000000;
        n += self.emf_green as u8 * 0b01000000;
        n += self.emf_red as u8 * 0b00100000;
        n += self.show_sprites as u8 * 0b00010000;
        n += self.show_background as u8 * 0b00001000;
        n += self.show_sprites_in_leftmost as u8 * 0b00000100;
        n += self.show_background_in_leftmost as u8 * 0b00000010;
        n += self.gray_scale as u8;
        n
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpuStatus {
    pub in_vlank: bool,
    sprite_zero_hit: bool,
    sprite_evoluation: bool,
    bus: u8,
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
        }
    }
    fn set(&mut self, n: u8) {
        self.in_vlank = (n & 0b10000000) != 0;
        self.sprite_zero_hit = (n & 0b01000000) != 0;
        self.sprite_evoluation = (n & 0b00100000) != 0;
        self.bus = n & 0b00001111;
    }

    fn to_n(&self) -> u8 {
        let mut n = 0;
        n += self.in_vlank as u8 * 0b10000000;
        n += self.sprite_zero_hit as u8 * 0b01000000;
        n += self.sprite_evoluation as u8 * 0b00100000;
        n += self.bus;
        n
    }

    pub fn is_occured_sprite_zero_hit(&self) -> bool {
        self.sprite_zero_hit
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
        let n = self.buffer;
        self.buffer = self.vram_memory;
        n
    }

    pub fn set(&mut self, n: u8) {
        self.vram_memory = n;
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

    pub fn set(&mut self, n: u16, r: u8) {
        match n {
            0x2000 => {
                self.internal_registers.temporary_vram &= 0b1111001111111111;
                self.internal_registers.temporary_vram |= (r as u16 & 0b00000011) << 10;
                self.ppu_ctrl.set(r);
            }
            0x2001 => self.ppu_mask.set(r),
            0x2002 => self.ppu_status.set(r),
            0x2003 => self.oam_addr = r,
            0x2004 => self.oam_data = r,
            0x2005 => {
                let r = r as u16;
                match self.internal_registers.latch_flag {
                    true => {
                        self.internal_registers.temporary_vram &= 0b1000110000011111;
                        let b = (r & 0b00000111) << 12;
                        let m = ((r & 0b00111000) >> 3) << 5;
                        let h = ((r & 0b11000000) >> 6) << 8;
                        self.internal_registers.temporary_vram |= h | m | b
                    }
                    false => {
                        self.internal_registers.x_scroll = 0;
                        self.internal_registers.temporary_vram &= 0b1111111111100000;
                        let b = r & 0b00000111;
                        let h = (r & 0b11111000) >> 3;
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
                        self.internal_registers.temporary_vram |= r as u16;
                        self.internal_registers
                            .copy_current_vram_to_tempolary_vram();
                    }
                    false => {
                        self.internal_registers.temporary_vram &= 0b000000011111111;
                        self.internal_registers.temporary_vram |= (r as u16 & 0b00111111) << 8;
                    }
                }
                self.internal_registers.toggle_latch();
            }
            _ => unreachable!(),
        }
    }

    pub fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x2000 => self.ppu_ctrl.to_n(),
            0x2001 => self.ppu_mask.to_n(),
            0x2002 => self.ppu_status.to_n(),
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            _ => unreachable!(),
        }
    }

    pub fn constant_inc_vram(&mut self) {
        let n = self.ppu_ctrl.increment_vram_num();
        self.internal_registers.inc_vram_addr(n);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RP2A03 {
    pub oam_dma: u8,
    pub controller_0: Controller,
    pub controller_1: Controller,
    pub test_fanctionality: [u8; 0x0008],
}

impl Default for RP2A03 {
    fn default() -> Self {
        Self::new()
    }
}

impl RP2A03 {
    pub fn new() -> Self {
        Self {
            oam_dma: 0,
            controller_0: Controller::default(),
            controller_1: Controller::default(),
            test_fanctionality: [0; 0x0008],
        }
    }

    fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x4014 => self.oam_dma,
            0x4016 => self.controller_0.d0,
            0x4017 => self.controller_1.d0,
            _ => unreachable!(),
        }
    }

    fn set(&mut self, n: u16, r: u8) {
        match n {
            0x4014 => self.oam_dma = r,
            _ => unreachable!(),
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMap {
    #[serde_as(as = "[_; 0x0800]")]
    pub wram: [u8; 0x0800],
    #[serde_as(as = "[_; 0x1800]")]
    pub wram_mirror: [u8; 0x1800],
    pub ppu_register: PpuRegister,
    #[serde_as(as = "[_; 0x1FF8]")]
    pub ppu_register_mirror: [u8; 0x1FF8],
    pub rp2a03: RP2A03,
    pub func_apu_io: [u8; 0x0008],
    #[serde_as(as = "[_; 0x1FE0]")]
    pub erom: [u8; 0x1FE0],
    #[serde_as(as = "[_; 0x2000]")]
    pub eram: [u8; 0x2000],
    #[serde_as(as = "[_; 0x4000]")]
    pub prg_rom1: [u8; 0x4000],
    #[serde_as(as = "[_; 0x4000]")]
    pub prg_rom2: [u8; 0x4000],
}

impl CpuMap {
    pub fn new() -> Self {
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

    pub fn lh_ignore_overflowing_addr(&mut self, n: u16) -> (u8, u8) {
        let h = n & 0xff00;
        let l = (n as u8).wrapping_add(1);
        let next_addr = h | l as u16;
        let l = self.addr(n as u16);
        let h = self.addr(next_addr as u16);
        (l, h)
    }

    pub fn lh_zeropage_addr(&mut self, n: u8) -> (u8, u8) {
        let next_addr_mem = n.wrapping_add(1);
        let l = self.addr(n as u16);
        let h = self.addr(next_addr_mem as u16);
        (l, h)
    }

    pub fn lh_addr(&mut self, n: u16) -> (u8, u8) {
        let l = self.addr(n);
        let h = self.addr(n + 1);
        (l, h)
    }

    pub fn hl_addr(&mut self, n: u16) -> (u8, u8) {
        let h = self.addr(n);
        let l = self.addr(n + 1);
        (h, l)
    }
}

impl Mapper for CpuMap {
    fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x0000..=0x07FF => self.wram[n as usize],
            0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize],
            0x2000..=0x2007 => self.ppu_register.addr(n),
            0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize],
            0x4000..=0x4017 => self.rp2a03.addr(n),
            0x4018..=0x401F => self.func_apu_io[(n - 0x4018) as usize],
            0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize],
            0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize],
            0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize],
            0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize],
        }
    }

    fn set(&mut self, n: u16, r: u8) {
        match n {
            0x0000..=0x07FF => self.wram[n as usize] = r,
            0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize] = r,
            0x2000..=0x2007 => self.ppu_register.set(n, r),
            0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize] = r,
            0x4000..=0x4015 => self.rp2a03.set(n, r),
            0x4016..=0x4017 => unreachable!(),
            0x4018..=0x401F => self.func_apu_io[(n - 0x4017) as usize] = r,
            0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize] = r,
            0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize] = r,
            0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize] = r,
            0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize] = r,
        };
    }
}
