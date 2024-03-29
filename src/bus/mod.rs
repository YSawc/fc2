pub mod cpu_map;

use crate::apu::*;
use crate::nes::*;
use crate::ppu::*;
use cpu_map::*;
use serde::{Deserialize, Serialize};

pub trait Mapper {
    fn addr(&mut self, addr: u16) -> u8;
    fn set(&mut self, addr: u16, data: u8);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bus {
    pub cpu_bus: CpuMap,
    pub ppu: PPU,
    pub apu: APU,
    pub controller_polling_data: u16,
    controller_0_polled_data: u8,
    controller_1_polled_data: u8,
}

impl Bus {
    pub fn new(nes: &Nes) -> Self {
        Self {
            cpu_bus: CpuMap::default(),
            ppu: PPU::new(nes),
            apu: APU::default(),
            controller_polling_data: 0,
            controller_0_polled_data: 0,
            controller_1_polled_data: 0,
        }
    }
}

impl Mapper for Bus {
    fn addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x2001 | 0x2003..=0x2006 | 0x2008..=0x3FFF | 0x4014 | 0x4018..=0xFFFF => {
                self.cpu_bus.addr(addr)
            }
            0x2002 => {
                self.cpu_bus.ppu_register.internal_registers.off_latch();
                self.cpu_bus.addr(addr)
            }
            0x2007 => {
                let addr = self.cpu_bus.ppu_register.internal_registers.current_vram;
                self.cpu_bus.ppu_register.constant_inc_vram();
                self.cpu_bus
                    .ppu_register
                    .ppu_buffer
                    .set(self.ppu.map.addr(addr));
                self.cpu_bus.ppu_register.ppu_buffer.addr()
            }
            0x4000 => self.apu.pulse1.addr(0),
            0x4001 => self.apu.pulse1.addr(1),
            0x4002 => self.apu.pulse1.addr(2),
            0x4003 => self.apu.pulse1.addr(3),
            0x4004 => self.apu.pulse2.addr(0),
            0x4005 => self.apu.pulse2.sweep_addr(),
            0x4006 => self.apu.pulse2.addr(2),
            0x4007 => self.apu.pulse2.addr(3),
            0x4008..=0x4013 => 0,
            0x4015 => self.apu.channel_controller.addr(),
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

    fn set(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x2006 | 0x2008..=0x3FFF | 0x4014 | 0x4018..=0xFFFF => {
                self.cpu_bus.set(addr, data)
            }
            0x2007 => {
                let addr = self.cpu_bus.ppu_register.internal_registers.current_vram;
                self.cpu_bus.ppu_register.constant_inc_vram();
                self.ppu.map.set(addr, data);
            }
            0x4000 => self.apu.pulse1.set(0, data),
            0x4001 => self.apu.pulse1.sweep_set(data),
            0x4002 => self.apu.pulse1.set(2, data),
            0x4003 => self.apu.pulse1.set(3, data),
            0x4004 => self.apu.pulse2.set(0, data),
            0x4005 => self.apu.pulse2.sweep_set(data),
            0x4006 => self.apu.pulse2.set(2, data),
            0x4007 => self.apu.pulse2.set(3, data),
            0x4008 => self.apu.triangle.set(0, data),
            0x4009 => (),
            0x400A => self.apu.triangle.set(2, data),
            0x400B => self.apu.triangle.set(3, data),
            0x400C => self.apu.noise.set(0, data),
            0x400D => (),
            0x400E => self.apu.noise.set(2, data),
            0x400F => self.apu.noise.set(3, data),
            0x4010..=0x4013 => (),
            0x4015 => self.apu.channel_controller.set(data),
            0x4016 => match data % 2 {
                1 => {
                    let polling_data = self.controller_polling_data;
                    self.controller_0_polled_data = (polling_data & 0xff) as u8;
                    self.controller_1_polled_data = ((polling_data & (0xff << 8)) >> 8) as u8;
                }
                0 => {
                    self.controller_polling_data = 0;
                }
                _ => (),
            },
            0x4017 => {
                self.apu.frame_counter.set(data);
            }
        };
    }
}
