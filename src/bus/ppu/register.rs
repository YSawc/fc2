use crate::util::*;

#[derive(Debug, Clone)]
pub struct Register {
    pub ppu_ctrl: PpuCtrl,
    pub ppu_mask: PpuMask,
    pub ppu_status: PpuStatus,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,
    pub oam_dma: u8,
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
}

#[derive(Debug, Clone)]
pub struct Oam {
    pos_y: u8,
    index_num: u8,
    attr: u8,
    pos_x: u8,
}

impl Default for Oam {
    fn default() -> Self {
        Self::new()
    }
}

impl Oam {
    pub fn new() -> Self {
        Self {
            pos_y: 0,
            index_num: 0,
            attr: 0,
            pos_x: 0,
        }
    }

    pub fn set(&mut self, n: u32) {
        self.pos_y = (n & 0x11000000) as u8;
        self.index_num = (n & 0x00110000) as u8;
        self.attr = (n & 0x00001100) as u8;
        self.pos_x = (n & 0x00000011) as u8;
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
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,
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
            0x2005 => self.ppu_scroll = r,
            0x2006 => self.ppu_data = r,
            0x2007 => self.oam_dma = r,
            _ => unreachable!(),
        }
    }
}
