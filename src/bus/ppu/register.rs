use crate::util::*;

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
    pub fn new() -> Self {
        Self {
            write_flag: false,
            addr: 0,
        }
    }

    pub fn toggle_flag(&mut self) {
        match self.write_flag {
            true => self.write_flag = false,
            false => self.write_flag = true,
        }
    }

    pub fn set(&mut self, r: u8) {
        match self.write_flag {
            true => {
                self.addr = 0;
                self.addr += (r as u16) << 8;
            }
            false => self.addr += r as u16,
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
    nmi: bool,
    ppu_selector: bool,
    sprite_size: bool,
    bk_table_addr: bool,
    sprite_ptn_table_addr: bool,
    vram_increment: bool,
    base_name_table_addr: u8,
}

impl Default for PpuCtrl {
    fn default() -> Self {
        Self::new()
    }
}

impl PpuCtrl {
    pub fn new() -> Self {
        Self {
            nmi: false,
            ppu_selector: false,
            sprite_size: false,
            bk_table_addr: false,
            sprite_ptn_table_addr: false,
            vram_increment: false,
            base_name_table_addr: 0,
        }
    }

    pub fn set(&mut self, n: u8) {
        let s = format!("{:08b}", n);
        fn chars_nth(s: &str, n: usize) -> u32 {
            s.chars().nth(n).unwrap().to_digit(2).unwrap()
        }

        self.nmi = n_to_bool(chars_nth(&s, 7));
        self.ppu_selector = n_to_bool(chars_nth(&s, 6));
        self.sprite_size = n_to_bool(chars_nth(&s, 5));
        self.bk_table_addr = n_to_bool(chars_nth(&s, 4));
        self.sprite_ptn_table_addr = n_to_bool(chars_nth(&s, 3));
        self.vram_increment = n_to_bool(chars_nth(&s, 2));
        self.base_name_table_addr = n & 0b00000011;
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += bool_to_n(self.nmi) << 7;
        n += bool_to_n(self.ppu_selector) << 6;
        n += bool_to_n(self.sprite_size) << 5;
        n += bool_to_n(self.bk_table_addr) << 4;
        n += bool_to_n(self.sprite_ptn_table_addr) << 3;
        n += bool_to_n(self.vram_increment) << 2;
        n += self.base_name_table_addr;
        n
    }
}

#[derive(Debug, Clone)]
pub struct PpuMask {
    emf_blue: bool,
    emf_green: bool,
    emf_red: bool,
    show_sprites: bool,
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
    pub fn new() -> Self {
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

    pub fn set(&mut self, n: u8) {
        let s = format!("{:08b}", n);
        fn chars_nth(s: &str, n: usize) -> u32 {
            s.chars().nth(n).unwrap().to_digit(2).unwrap()
        }

        self.emf_blue = n_to_bool(chars_nth(&s, 7));
        self.emf_green = n_to_bool(chars_nth(&s, 6));
        self.emf_red = n_to_bool(chars_nth(&s, 5));
        self.show_sprites = n_to_bool(chars_nth(&s, 4));
        self.show_background = n_to_bool(chars_nth(&s, 3));
        self.show_sprites_in_leftmost = n_to_bool(chars_nth(&s, 2));
        self.show_background_in_leftmost = n_to_bool(chars_nth(&s, 1));
        self.gray_scale = n_to_bool(chars_nth(&s, 0));
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += bool_to_n(self.emf_blue);
        n += bool_to_n(self.emf_green);
        n += bool_to_n(self.emf_red);
        n += bool_to_n(self.show_sprites);
        n += bool_to_n(self.show_background);
        n += bool_to_n(self.show_sprites_in_leftmost);
        n += bool_to_n(self.show_background_in_leftmost);
        n += bool_to_n(self.gray_scale);
        n
    }
}

#[derive(Debug, Clone)]
pub struct PpuStatus {
    virtical_blank_in_vlank: bool,
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
    pub fn new() -> Self {
        Self {
            virtical_blank_in_vlank: false,
            sprite_zero_hit: false,
            sprite_evoluation: false,
            bus: 0,
        }
    }
    pub fn set(&mut self, n: u8) {
        let s = format!("{:08b}", n);
        let s = s.to_string();
        fn chars_nth(s: &str, n: usize) -> u32 {
            s.chars().nth(n).unwrap().to_digit(2).unwrap()
        }

        self.virtical_blank_in_vlank = n_to_bool(chars_nth(&s, 7));
        self.sprite_zero_hit = n_to_bool(chars_nth(&s, 6));
        self.sprite_evoluation = n_to_bool(chars_nth(&s, 5));
        self.bus = n & 0b00001111;
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += bool_to_n(self.virtical_blank_in_vlank) << 7;
        n += bool_to_n(self.sprite_zero_hit) << 6;
        n += bool_to_n(self.sprite_evoluation) << 5;
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
    pub fn new() -> Self {
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

    pub fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x2000 => self.ppu_ctrl.to_n(),
            0x2001 => self.ppu_mask.to_n(),
            0x2002 => self.ppu_status.to_n(),
            0x2003 => self.oam_addr,
            0x2004 => self.oam_data,
            0x4014 => self.oam_dma,
            _ => unreachable!(),
        }
    }

    pub fn relative_addr(&mut self, n: u16) -> u16 {
        match n {
            0x2005 => self.ppu_scroll.addr,
            0x2006 => self.ppu_addr.addr,
            _ => unreachable!(),
        }
    }
}
