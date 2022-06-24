pub mod mapper;
pub mod register;

use mapper::*;

#[derive(Debug, Clone)]
pub struct PPU {
    pub map: Map,
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
        }
    }
}
