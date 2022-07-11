pub mod cpu_map;
pub mod ppu;

use cpu_map::*;
use ppu::*;

pub trait Mapper {
    fn addr(&self, n: u16) -> u8;
    fn set(&mut self, n: u16, r: u8);
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu_bus: CpuMap,
    pub ppu: PPU,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            cpu_bus: CpuMap::default(),
            ppu: PPU::default(),
        }
    }
}

impl Mapper for Bus {
    fn addr(&self, n: u16) -> u8 {
        match n {
            0x0000..=0x1FFF | 0x2008..=0xFFFF => self.cpu_bus.addr(n),
            0x2000..=0x2004 | 0x2007 => self.ppu.register.clone().addr(n),
            0x2005..=0x2006 => {
                let r = self.ppu.register.clone().relative_addr(n);
                self.ppu.map.addr(r)
            }
        }
    }

    fn set(&mut self, n: u16, r: u8) {
        match n {
            0x0000..=0x07FF => self.cpu_bus.wram[n as usize] = r,
            0x0800..=0x1FFF => self.cpu_bus.wram_mirror[(n - 0x0800) as usize] = r,
            0x2000..=0x2006 => self.ppu.register.set(n, r),
            0x2007 => {
                let n = self.ppu.register.ppu_addr.addr();
                let inc = self.ppu.register.ppu_ctrl.increment_vram_num() as u16;
                self.ppu.register.ppu_addr.addr += inc;
                self.ppu.map.set(n, r);
                self.ppu.register.ppu_buffer +=
                    self.ppu.register.ppu_ctrl.base_name_table_addr as u16;
            }
            0x2008..=0x3FFF => self.cpu_bus.ppu_register_mirror[(n - 0x2008) as usize] = r,
            0x4000..=0x401F => self.cpu_bus.apu_pad[(n - 0x4000) as usize] = r,
            0x4020..=0x5FFF => self.cpu_bus.erom[(n - 0x4020) as usize] = r,
            0x6000..=0x7FFF => self.cpu_bus.eram[(n - 0x6000) as usize] = r,
            0x8000..=0xBFFF => self.cpu_bus.prg_rom1[(n - 0x8000) as usize] = r,
            0xC000..=0xFFFF => self.cpu_bus.prg_rom2[(n - 0xC000) as usize] = r,
        };
    }
}
