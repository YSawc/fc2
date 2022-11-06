#[derive(Debug, Clone)]
pub struct Register {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: WriteTwiceRegister,
    pub ppu_addr: WriteTwiceRegister,
    pub ppu_data: u8,
    pub ppu_buffer: u16,
    pub oam_dma: u8,
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
    sprite_size: bool,
    pub bk_table_addr: bool,
    sprite_ptn_table_addr: bool,
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

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
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
            oam_dma: 0,
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
            0x4014 => self.oam_dma = r,
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
            0x4014 => self.oam_dma,
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
