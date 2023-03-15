pub mod pulse;
pub mod triangle;

use crate::apu::pulse::Pulse;
use crate::apu::triangle::Triangle;

#[derive(Debug, Clone)]
pub struct ChannelController {
    pub enable_pulse1: bool,
    pub enable_pulse2: bool,
    pub enable_triangle: bool,
    pub enable_noise: bool,
    pub enable_dmc: bool,
}

impl Default for ChannelController {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelController {
    pub fn new() -> Self {
        Self {
            enable_pulse1: false,
            enable_pulse2: false,
            enable_triangle: false,
            enable_noise: false,
            enable_dmc: false,
        }
    }

    pub fn addr(&mut self) -> u8 {
        let mut data = 0;
        data += self.enable_pulse1 as u8;
        data += (self.enable_pulse2 as u8) << 1;
        data += (self.enable_triangle as u8) << 2;
        data += (self.enable_noise as u8) << 3;
        data += (self.enable_dmc as u8) << 4;

        data
    }

    pub fn set(&mut self, data: u8) {
        self.enable_pulse1 = (data & 0b00000001) != 0;
        self.enable_pulse2 = (data & 0b00000010) != 0;
        self.enable_triangle = (data & 0b00000100) != 0;
        self.enable_noise = (data & 0b00001000) != 0;
        self.enable_dmc = (data & 0b00010000) != 0;
    }
}

#[derive(Debug, Clone)]
pub enum FrameMode {
    _4STEP,
    _5STEP,
}

#[derive(Debug, Clone)]
pub struct FrameCounter {
    pub mode: FrameMode,
    irq: bool,
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            mode: FrameMode::_4STEP,
            irq: false,
        }
    }

    pub fn set(&mut self, data: u8) {
        self.mode = match (data & 0b10000000) >> 7 {
            0 => FrameMode::_4STEP,
            1 => FrameMode::_5STEP,
            _ => unreachable!(),
        };

        self.irq = ((data & 0b01000000) >> 6) != 0;
    }

    pub fn get_envelop_count(&self) -> u8 {
        match self.mode {
            FrameMode::_4STEP => 60,
            FrameMode::_5STEP => 48,
        }
    }
}

#[derive(Debug, Clone)]
pub struct APU {
    pub pulse1: Pulse,
    pub pulse2: Pulse,
    pub triangle: Triangle,
    pub channel_controller: ChannelController,
    pub frame_counter: FrameCounter,
}

impl Default for APU {
    fn default() -> Self {
        Self::new()
    }
}

impl APU {
    pub const LENGTH_COUNTER: [u16; 0x20] = [
        10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, // 00-0F
        12, 16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30, // 10-1F
    ];

    pub fn new() -> Self {
        let pulse1 = Pulse::default();
        let pulse2 = Pulse::default();
        let triangle = Triangle::default();
        let channel_controller = ChannelController::default();
        let frame_counter = FrameCounter::default();

        Self {
            pulse1,
            pulse2,
            triangle,
            channel_controller,
            frame_counter,
        }
    }
}
