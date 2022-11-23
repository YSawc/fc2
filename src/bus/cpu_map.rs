use crate::bus::Mapper;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct PpuRegister {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: WriteTwiceRegister,
    pub ppu_addr: WriteTwiceRegister,
    pub ppu_data: u8,
    pub ppu_buffer: u16,
}

#[derive(Debug, Clone)]
pub struct WriteTwiceRegister {
    write_flag: bool,
    pub addr: u16,
}

impl Default for WriteTwiceRegister {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteTwiceRegister {
    fn new() -> Self {
        Self {
            write_flag: false,
            addr: 0,
        }
    }

    fn toggle_flag(&mut self) {
        match self.write_flag {
            true => self.write_flag = false,
            false => self.write_flag = true,
        }
    }

    fn set(&mut self, r: u8) {
        match self.write_flag {
            true => {
                self.addr += r as u16;
            }
            false => {
                self.addr = 0;
                self.addr += (r as u16) << 8;
            }
        }
        self.toggle_flag();
    }

    pub fn addr(&mut self) -> u16 {
        match self.write_flag {
            true => unreachable!(),
            false => self.addr,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PpuCtrl {
    pub gen_nmi: bool,
    ppu_selector: bool,
    pub sprite_size: bool,
    pub bk_table_addr: bool,
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

    fn to_n(&self) -> u8 {
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

    pub fn increment_vram_num(&mut self) -> u8 {
        match self.vram_increment {
            false => 1,
            true => 32,
        }
    }

    pub fn for_big(&self) -> bool {
        self.sprite_size
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            ppu_scroll: WriteTwiceRegister::default(),
            ppu_addr: WriteTwiceRegister::default(),
            ppu_data: 0,
            ppu_buffer: 0,
        }
    }

    pub fn set(&mut self, n: u16, r: u8) {
        // println!("n: {:0x?}, r: {:0x?}", n, r);
        match n {
            0x2000 => self.ppu_ctrl.set(r),
            0x2001 => self.ppu_mask.set(r),
            0x2002 => self.ppu_status.set(r),
            0x2003 => self.oam_addr = r,
            0x2004 => self.oam_data = r,
            0x2005 => self.ppu_scroll.set(r),
            0x2006 => self.ppu_addr.set(r),
            _ => unreachable!(),
        }
    }

    pub fn addr(&self, n: u16) -> u8 {
        match n {
            0x2000 => self.ppu_ctrl.to_n(),
            0x2001 => self.ppu_mask.to_n(),
            0x2002 => self.ppu_status.to_n(),
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            0x2007 => self.ppu_buffer as u8,
            _ => unreachable!(),
        }
    }

    pub fn relative_addr(&self, n: u16) -> u16 {
        match n {
            0x2005 => self.ppu_scroll.addr,
            0x2006 => self.ppu_addr.addr,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RP2A03 {
    pub pulse: [u8; 0x0008],
    pub triangle: [u8; 0x0004],
    pub noise: [u8; 0x0004],
    pub dmc: [u8; 0x0004],
    pub oam_dma: u8,
    pub snd_chn: u8,
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
            pulse: [0; 0x0008],
            triangle: [0; 0x0004],
            noise: [0; 0x0004],
            dmc: [0; 0x0004],
            oam_dma: 0,
            snd_chn: 0,
            controller_0: Controller::default(),
            controller_1: Controller::default(),
            test_fanctionality: [0; 0x0008],
        }
    }

    fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x4000..=0x4007 => self.pulse[(n - 0x4000) as usize],
            0x4008..=0x400B => self.triangle[(n - 0x4008) as usize],
            0x400C..=0x400F => self.noise[(n - 0x400C) as usize],
            0x4010..=0x4013 => self.dmc[(n - 0x4010) as usize],
            0x4014 => self.oam_dma,
            0x4015 => self.snd_chn,
            0x4016 => self.controller_0.d0,
            0x4017 => self.controller_1.d0,
            _ => unreachable!(),
        }
    }

    fn set(&mut self, n: u16, r: u8) {
        match n {
            0x4000..=0x4007 => self.pulse[(n - 0x4000) as usize] = r,
            0x4008..=0x400B => self.triangle[(n - 0x4008) as usize] = r,
            0x400C..=0x400F => self.noise[(n - 0x400C) as usize] = r,
            0x4010..=0x4013 => self.dmc[(n - 0x4010) as usize] = r,
            0x4014 => self.oam_dma = r,
            0x4015 => self.snd_chn = r,
            _ => {
                print!("{:0x?}", n);
                unreachable!();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CpuMap {
    pub wram: [u8; 0x0800],
    pub wram_mirror: [u8; 0x1800],
    pub ppu_register: PpuRegister,
    pub ppu_register_mirror: [u8; 0x1FF8],
    pub rp2a03: RP2A03,
    pub func_apu_io: [u8; 0x0008],
    pub erom: [u8; 0x1FE0],
    pub eram: [u8; 0x2000],
    pub prg_rom1: [u8; 0x4000],
    pub prg_rom2: [u8; 0x4000],
}

impl Default for CpuMap {
    fn default() -> Self {
        Self::new()
    }
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
