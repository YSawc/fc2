use sdl2::audio::AudioCallback;

#[derive(Debug, Clone)]
pub struct Sweep {
    dividers_count: u8,
    is_enable: bool,
    dividers_period: u8,
    is_negative: bool,
    shift_count: u8,
}

impl Default for Sweep {
    fn default() -> Self {
        Self::new()
    }
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            dividers_count: 0,
            is_enable: false,
            dividers_period: 0,
            is_negative: false,
            shift_count: 0,
        }
    }

    pub fn addr(&mut self) -> u8 {
        let mut data = 0;
        data += (self.is_enable as u8) << 7;
        data += (self.dividers_period & 0b01110000) << 4;
        data += (self.is_negative as u8) << 3;
        data += self.shift_count;

        data
    }

    pub fn set(&mut self, data: u8) {
        self.dividers_count = 0;
        self.is_enable = (data & 0b10000000) != 0;
        self.dividers_period = (data & 0b01110000) >> 4;
        self.is_negative = (data & 0b00001000) != 0;
        self.shift_count = data & 0b00000111;
    }

    pub fn update(&mut self, timer: &mut u16) {
        if self.is_enable {
            self.dividers_count += 1;
            if self.dividers_count >= self.dividers_period {
                self.dividers_count -= self.dividers_period;
                if self.is_negative {
                    *timer >>= self.shift_count;
                } else {
                    *timer <<= self.shift_count;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pulse {
    pub timer: u16,
    pub sweep: Sweep,
    pub clock_count: u16,
    pub length_counter_index: u8,
    pub length_counter: u16,
    pub current_volume: u8,
    pub call_back_volume_buf: Vec<u8>,
    pub envelope_and_liner_counter: u8,
    pub devider_period: u8,
    pub phase: f32,
    pub phase_inc: f32,
    pub current_phase_inc: f32,
    pub call_back_phase_inc_buf: Vec<f32>,
    pub is_constant_volume: bool,
    pub envelope_volume: u8,
    pub sequencer_count: u8,
    pub duty: u8,
    pub call_back_duty_buf: Vec<u8>,
    pub counter_halt: bool,
    pub is_loop_envelope: bool,
}

impl Default for Pulse {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioCallback for Pulse {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        if self.call_back_volume_buf.len() == 0 {
            return;
        }

        for (i, x) in out.iter_mut().enumerate() {
            if i >= self.call_back_volume_buf.len() {
                break;
            }
            let duty = match self.call_back_duty_buf[i] {
                0 => 0.875,
                1 => 0.75,
                2 => 0.50,
                3 => 0.25,
                _ => unreachable!(),
            };
            *x = if self.phase <= duty {
                self.call_back_volume_buf[i] as f32 * 0.02
            } else {
                self.call_back_volume_buf[i] as f32 * (-0.02)
            };
            self.phase = (self.phase + self.call_back_phase_inc_buf[i]) % 1.0;
        }
        self.call_back_volume_buf = [].to_vec();
        self.call_back_phase_inc_buf = [].to_vec();
        self.call_back_duty_buf = [].to_vec();
    }
}

impl Pulse {
    pub const LENGTH_COUNTER: [u16; 0x20] = [
        10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96,
        22, 192, 24, 72, 26, 16, 28, 32, 30,
    ];

    pub fn new() -> Self {
        Self {
            timer: 0,
            sweep: Sweep::default(),
            clock_count: 0,
            envelope_and_liner_counter: 0,
            call_back_volume_buf: [].to_vec(),
            current_volume: 0,
            length_counter_index: 0,
            length_counter: 0,
            devider_period: 0,

            phase: 0.0,
            phase_inc: 0.0,
            current_phase_inc: 0.0,
            call_back_phase_inc_buf: [].to_vec(),
            is_constant_volume: false,
            envelope_volume: 0,
            sequencer_count: 0,
            duty: 0,
            call_back_duty_buf: [].to_vec(),
            counter_halt: false,
            is_loop_envelope: false,
        }
    }

    pub fn get_duty(&self) -> f32 {
        match self.duty {
            0 => 0.875,
            1 => 0.75,
            2 => 0.50,
            3 => 0.25,
            _ => unreachable!(),
        }
    }

    pub fn addr(&mut self, addr: u8) -> u8 {
        let mut n = 0;
        match addr {
            0 => {
                n += self.duty << 6;
                n += (self.counter_halt as u8) << 5;
                n += (self.is_constant_volume as u8) << 4;
                n += self.devider_period;
                n
            }
            1 => self.sweep.addr(),
            2 => {
                n += (self.timer & 0b11111111) as u8;
                n
            }
            3 => {
                n += ((self.timer & 0b11100000000) >> 8) as u8;
                n += ((self.length_counter_index & 0b11111000) >> 3) as u8;
                n
            }

            _ => unreachable!(),
        }
    }

    pub fn set(&mut self, addr: u8, data: u8) {
        match addr {
            0 => {
                self.duty = (data & 0b11000000) >> 6;
                self.counter_halt = (data & 0b00100000) != 0;
                self.is_loop_envelope = (data & 0b00100000) != 0;
                self.is_constant_volume = (data & 0b00010000) != 0;
                self.devider_period = data & 0b00001111;
            }
            2 => {
                self.timer &= 0x700;
                self.timer |= data as u16;
            }
            3 => {
                self.timer &= 0xFF;
                self.timer |= (data as u16 & 0b00000111) << 8;
                self.length_counter_index = data & 0b11111000;
                self.length_counter = Pulse::LENGTH_COUNTER[(data & 0b11111000) as usize >> 3];
                self.sequencer_count = self.devider_period;
                self.envelope_volume = 0x0F;
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_volume(&self) -> u8 {
        if self.is_constant_volume {
            self.envelope_volume
        } else {
            self.sequencer_count
        }
    }
}

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
    pub channel_controller: ChannelController,
    pub frame_counter: FrameCounter,
}

impl Default for APU {
    fn default() -> Self {
        Self::new()
    }
}

impl APU {
    pub fn new() -> Self {
        let pulse1 = Pulse::default();
        let pulse2 = Pulse::default();
        let channel_controller = ChannelController::default();
        let frame_counter = FrameCounter::default();

        Self {
            pulse1,
            pulse2,
            channel_controller,
            frame_counter,
        }
    }
}
