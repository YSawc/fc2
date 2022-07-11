pub mod mapper;
pub mod register;

use mapper::*;
use register::*;

#[derive(Debug, Clone)]
pub struct PPU {
    pub map: Map,
    pub register: Register,
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

impl PPU {
    pub fn new() -> Self {
        Self {
            map: Map::default(),
            register: Register::default(),
        }
    }

    pub fn run(&mut self, cpu_cycle: &mut u16) {
        if *cpu_cycle >= 341 {
            *cpu_cycle -= 341;
        }
    }
}
