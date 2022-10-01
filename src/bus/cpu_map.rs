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
            // 0x4016 => match self.controller_0.d0 {
            //     0 => (),
            //     1 => self.controller_0 = r,
            //     _ => (),
            // },
            // 0x4017 => self.controller_1 = r,
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
            ppu_register_mirror: [0; 0x1FF8],
            rp2a03: RP2A03::default(),
            func_apu_io: [0; 0x0008],
            erom: [0; 0x1FE0],
            eram: [0; 0x2000],
            prg_rom1: [0; 0x4000],
            prg_rom2: [0; 0x4000],
        }
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
            0x2000..=0x2007 => unreachable!(),
            0x2008..=0x3FFF => self.ppu_register_mirror[(n - 0x2008) as usize],
            0x4000..=0x4017 => self.rp2a03.addr(n),
            0x4018..=0x401F => self.func_apu_io[(n - 0x4017) as usize],
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
            0x2000..=0x2007 => unreachable!(),
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
