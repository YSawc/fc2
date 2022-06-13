#[derive(Debug, Clone)]
pub struct Map {
    pub wram: [u8; 0x0800],
    pub wram_mirror: [u8; 0x0800],
    pub ppu_register: [u8; 0x0008],
    pub ppu_register_mirror: [u8; 0x1FF8],
    pub apu_pad: [u8; 0x0020],
    pub erom: [u8; 0x1FE0],
    pub eram: [u8; 0x2000],
    pub prg_rom1: [u8; 0x4000],
    pub prg_rom2: [u8; 0x4000],
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Map {
    pub fn new() -> Self {
        Self {
            wram: [0; 0x0800],
            wram_mirror: [0; 0x0800],
            ppu_register: [0; 0x0008],
            ppu_register_mirror: [0; 0x1FF8],
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
            0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize],
            0x2000..=0x2007 => self.ppu_register[(n - 0x2000) as usize],
            0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize],
            0x4000..=0x401F => self.apu_pad[(n - 0x4000) as usize],
            0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize],
            0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize],
            0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize],
            0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize],
        }
    }

    pub fn set(&mut self, n: u16, r: u8) {
        match n {
            0x0000..=0x07FF => self.wram[n as usize] = r,
            0x0800..=0x1FFF => self.wram_mirror[(n - 0x0800) as usize] = r,
            0x2000..=0x2007 => self.ppu_register[(n - 0x2000) as usize] = r,
            0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize] = r,
            0x4000..=0x401F => self.apu_pad[(n - 0x4000) as usize] = r,
            0x4020..=0x5FFF => self.erom[(n - 0x4020) as usize] = r,
            0x6000..=0x7FFF => self.eram[(n - 0x6000) as usize] = r,
            0x8000..=0xBFFF => self.prg_rom1[(n - 0x8000) as usize] = r,
            0xC000..=0xFFFF => self.prg_rom2[(n - 0xC000) as usize] = r,
        };
    }
}
