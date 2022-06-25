pub mod configure;
pub mod renderer;

use crate::cpu::*;
use configure::PPU_CLOCK_RATE_FOR_CPU;

pub struct Emulator {
    pub cpu: CPU,
    pub cpu_cycle: u16,
    pub ppu_cycle: u16,
    pub ppu_clock_sync: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let cpu = CPU::default();
        Self {
            cpu,
            cpu_cycle: 0,
            ppu_cycle: 0,
            ppu_clock_sync: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn run(&mut self) {
        let cycle = match self.ppu_clock_sync {
            PPU_CLOCK_RATE_FOR_CPU => {
                self.ppu_clock_sync = 0;
                self.cpu.ex_ope()
            }
            _ => 0,
        };
        self.cpu_cycle += cycle as u16;
        self.cpu.bus.ppu.run(&mut self.cpu_cycle);
        self.ppu_clock_sync += 1;
    }
}
