pub mod cpu_map;

use crate::ppu::*;
use cpu_map::*;

pub trait Mapper {
    fn addr(&mut self, n: u16) -> u8;
    fn set(&mut self, n: u16, r: u8);
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub cpu_bus: CpuMap,
    pub ppu: PPU,
    pub controller_0_polling_data: u8,
    pub controller_0_polled_data: u8,
    pub controller_1_polling_data: u8,
    pub controller_1_polled_data: u8,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    fn new() -> Self {
        Self {
            cpu_bus: CpuMap::default(),
            ppu: PPU::default(),
            controller_0_polling_data: 0,
            controller_0_polled_data: 0,
            controller_1_polling_data: 0,
            controller_1_polled_data: 0,
        }
    }
}

impl Mapper for Bus {
    fn addr(&mut self, n: u16) -> u8 {
        match n {
            0x0000..=0x2004 | 0x2007..=0x4015 | 0x4018..=0xFFFF => self.cpu_bus.addr(n),
            0x2005..=0x2006 => {
                let r = self.cpu_bus.ppu_register.relative_addr(n);
                self.cpu_bus.addr(r)
            }
            0x4016 => {
                let n = self.controller_0_polled_data & 0x1;
                self.controller_0_polled_data >>= 0x1;
                n
            }
            0x4017 => {
                let n = self.controller_1_polled_data & 0x1;
                self.controller_1_polled_data >>= 0x1;
                n
            }
        }
    }

    fn set(&mut self, n: u16, r: u8) {
        match n {
            0x0000..=0x2006 | 0x2008..=0x4015 | 0x4018..=0xFFFF => self.cpu_bus.set(n, r),
            0x2007 => {
                let n = self.cpu_bus.ppu_register.ppu_addr.addr();
                let inc = self.cpu_bus.ppu_register.ppu_ctrl.increment_vram_num() as u16;
                self.cpu_bus.ppu_register.ppu_addr.addr += inc;
                self.ppu.map.set(n, r);
                self.cpu_bus.ppu_register.ppu_buffer +=
                    self.cpu_bus.ppu_register.ppu_ctrl.base_name_table_addr as u16;
            }
            0x4016 => match r % 2 {
                1 => {
                    self.controller_0_polled_data = self.controller_0_polling_data;
                }
                0 => {
                    self.controller_0_polling_data = 0;
                }
                _ => (),
            },
            0x4017 => match r {
                1 => {
                    self.controller_1_polled_data = self.controller_1_polling_data;
                    self.controller_1_polling_data = 0;
                }
                _ => (),
            },
        };
    }
}
