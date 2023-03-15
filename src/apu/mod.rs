use sdl2::audio::AudioCallback;
use sdl2::audio::AudioDeviceLockGuard;

#[derive(Debug, Clone)]
pub struct Sweep {
    pub dividers_count: u8,
    is_enable: bool,
    dividers_period: u8,
    is_negative: bool,
    pub shift_count: u8,
    reload_flag: bool,
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
            reload_flag: false,
        }
    }

    pub fn addr(&mut self) -> u8 {
        let mut data = 0;
        data += (self.is_enable as u8) << 7;
        data += self.dividers_period << 4;
        data += (self.is_negative as u8) << 3;
        data += self.shift_count;

        data
    }

    pub fn set(&mut self, data: u8) {
        self.dividers_count = 0;
        self.is_enable = (data & 0b10000000) != 0;
        self.dividers_period = (data & 0b01110000) >> 4;
        self.dividers_count = self.dividers_period;
        self.is_negative = (data & 0b00001000) != 0;
        self.shift_count = data & 0b00000111;
        self.reload_flag = true;
    }

    pub fn update(&mut self, timer: &mut u16) {
        if self.is_enable && *timer > 8 && *timer < 0x7FF && self.dividers_count == 0 {
            if self.is_negative {
                *timer = *timer - (*timer >> self.shift_count);
            } else {
                *timer = *timer + (*timer >> self.shift_count);
            }
            if *timer <= 8 || *timer >= 0x7FF {
                *timer = 0;
            }
        }

        if self.dividers_count > 0 {
            self.dividers_count -= 1;
        } else if self.dividers_count == 0 && self.reload_flag {
            self.dividers_count = self.dividers_period;
            self.reload_flag = false;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pulse {
    pub timer: u16,
    pub current_timer: u16,
    pub frame_counter: u8,
    pub sweep: Sweep,
    pub clock_count: u16,
    pub length_counter_index: u8,
    pub length_counter: u16,
    pub current_volume: u8,
    pub call_back_volume_buf: Vec<u8>,
    pub envelope_and_liner_counter: u8,
    pub envelope_divider: u8,
    pub constant_volume_and_devider_period: u8,
    pub phase: f32,
    pub phase_inc: f32,
    pub current_phase_inc: f32,
    pub call_back_phase_inc_buf: Vec<f32>,
    pub is_constant_volume: bool,
    pub sequencer_count: u8,
    pub duty: u8,
    pub call_back_duty_buf: Vec<u8>,
    pub is_loop_envelope_and_counter_halt: bool,
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
                self.call_back_volume_buf[i] as f32 * 0.005
            } else {
                self.call_back_volume_buf[i] as f32 * (-0.005)
            };
            self.phase = (self.phase + self.call_back_phase_inc_buf[i]) % 1.0;
        }
        self.call_back_volume_buf = [].to_vec();
        self.call_back_phase_inc_buf = [].to_vec();
        self.call_back_duty_buf = [].to_vec();
    }
}

impl Pulse {
    pub fn new() -> Self {
        Self {
            timer: 0,
            current_timer: 0,
            frame_counter: 0,
            sweep: Sweep::default(),
            clock_count: 0,
            envelope_and_liner_counter: 0,
            envelope_divider: 15,
            call_back_volume_buf: [].to_vec(),
            current_volume: 0,
            length_counter_index: 0,
            length_counter: 0,
            constant_volume_and_devider_period: 0,
            phase: 0.0,
            phase_inc: 0.0,
            current_phase_inc: 0.0,
            call_back_phase_inc_buf: [].to_vec(),
            is_constant_volume: false,
            sequencer_count: 0,
            duty: 0,
            call_back_duty_buf: [].to_vec(),
            is_loop_envelope_and_counter_halt: false,
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
                n += (self.is_loop_envelope_and_counter_halt as u8) << 5;
                n += (self.is_constant_volume as u8) << 4;
                n += self.constant_volume_and_devider_period;
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
                self.is_loop_envelope_and_counter_halt = (data & 0b00100000) != 0;
                self.is_constant_volume = (data & 0b00010000) != 0;
                self.constant_volume_and_devider_period = data & 0b00001111;
            }
            2 => {
                self.timer &= 0x700;
                self.timer |= data as u16;
                self.current_timer = self.timer;
            }
            3 => {
                self.timer &= 0xFF;
                self.timer |= (data as u16 & 0b00000111) << 8;
                self.current_timer = self.timer;
                self.length_counter_index = (data & 0b11111000) >> 3;
                self.length_counter = APU::LENGTH_COUNTER[self.length_counter_index as usize];
                self.sequencer_count = self.constant_volume_and_devider_period;
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_volume(&self) -> u8 {
        if self.is_constant_volume {
            self.constant_volume_and_devider_period
        } else {
            0x0F
        }
    }

    pub fn update_envelop(&mut self) {
        if self.envelope_divider > 0 {
            self.envelope_divider -= 1
        } else {
            self.envelope_divider = 15;
            if self.constant_volume_and_devider_period > 0 {
                self.constant_volume_and_devider_period -= 1;
            }
        }
    }

    pub fn update_length_counter(&mut self) {
        if self.length_counter > 0 {
            self.length_counter -= 1;
        }
    }

    fn update_5step_frame(&mut self) {
        if self.frame_counter == 39
            || self.frame_counter == 78
            || self.frame_counter == 117
            || self.frame_counter == 156
            || self.frame_counter == 192
        {
            self.update_envelop();
        }
        if self.frame_counter == 78 || self.frame_counter == 192 {
            self.update_length_counter();
            self.sweep.update(&mut self.current_timer);
        }
        if self.frame_counter >= 192 {
            self.frame_counter = 0;
        }
    }

    fn update_4step_frame(&mut self) {
        if self.frame_counter == 60
            || self.frame_counter == 120
            || self.frame_counter == 180
            || self.frame_counter == 240
        {
            self.update_envelop();
        }
        if self.frame_counter == 120 || self.frame_counter == 240 {
            self.update_length_counter();
            self.sweep.update(&mut self.current_timer);
        }
        if self.frame_counter >= 240 {
            self.frame_counter = 0;
        }
    }

    fn insert_callback(&mut self, lock: &mut AudioDeviceLockGuard<Pulse>) {
        (*lock).call_back_volume_buf.push(self.current_volume);
        (*lock).call_back_phase_inc_buf.push(self.current_phase_inc);
        (*lock).call_back_duty_buf.push(self.duty);
    }

    fn is_signal_enable(&self, is_enable: &bool) -> bool {
        *is_enable
            && (self.length_counter > 0 || self.is_loop_envelope_and_counter_halt)
            && self.current_timer >= 8
    }

    pub fn update(
        &mut self,
        frame_counter: &mut FrameCounter,
        is_enable: &mut bool,
        lock: &mut AudioDeviceLockGuard<Pulse>,
    ) {
        if self.is_signal_enable(&is_enable) {
            (*lock).clock_count += 1;
            if (*lock).clock_count >= 240 {
                (*lock).clock_count -= 240;
                self.frame_counter += 1;
                match frame_counter.mode {
                    FrameMode::_4STEP => self.update_4step_frame(),
                    FrameMode::_5STEP => self.update_5step_frame(),
                }
                self.current_volume = self.get_volume();
                self.current_phase_inc =
                    (1789773.0 / ((16.0 * self.current_timer as f32) + 1.0)) / 44100 as f32;
            }
        } else {
            self.current_volume = 0;
            self.current_phase_inc = 0.0;
        };
        self.insert_callback(lock);
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub timer: u16,
    pub current_timer: u16,
    pub frame_counter: u8,
    pub clock_count: u16,
    pub controll_flag: bool,
    pub length_counter_halt_or_linear_counter_control: bool,
    pub linear_counter_load: u8,
    pub linear_counter: u8,
    pub linear_phase: f32,
    pub call_back_linear_phase_buf: Vec<f32>,
    pub linear_inc_phase: bool,
    pub call_back_phase_inc_buf: Vec<f32>,
    pub length_counter_index: u8,
    pub length_counter: u16,
    pub current_phase_inc: f32,
    pub is_loop_envelope_and_counter_halt: bool,
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioCallback for Triangle {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        if self.call_back_phase_inc_buf.len() == 0 {
            return;
        }

        for (i, x) in out.iter_mut().enumerate() {
            if i >= self.call_back_phase_inc_buf.len() {
                break;
            }
            *x = if self.linear_phase <= 0.5 {
                self.call_back_linear_phase_buf[i] * 0.003
            } else {
                -self.call_back_linear_phase_buf[i] * 0.003
            };
            self.linear_phase = (self.linear_phase + self.call_back_phase_inc_buf[i]) % 1.0;
        }
        self.call_back_linear_phase_buf = [].to_vec();
        self.call_back_phase_inc_buf = [].to_vec();
    }
}

impl Triangle {
    fn new() -> Self {
        Self {
            timer: 0,
            current_timer: 0,
            frame_counter: 0,
            clock_count: 0,
            controll_flag: false,
            length_counter_halt_or_linear_counter_control: false,
            linear_counter_load: 0,
            linear_counter: 0,
            linear_phase: 0.0,
            call_back_linear_phase_buf: vec![],
            linear_inc_phase: true,
            length_counter_index: 0,
            length_counter: 0,
            current_phase_inc: 0.0,
            call_back_phase_inc_buf: vec![],
            is_loop_envelope_and_counter_halt: false,
        }
    }

    pub fn update_liner_phase(&mut self) {
        match self.linear_inc_phase {
            true => {
                if self.linear_phase < 16.0 {
                    self.linear_phase += 1.0;
                } else {
                    self.linear_inc_phase = false;
                }
            }
            false => {
                if self.linear_phase > -16.0 {
                    self.linear_phase -= 1.0;
                } else {
                    self.linear_inc_phase = true;
                }
            }
        }
    }

    pub fn update_linear_counter(&mut self) {
        if self.length_counter_halt_or_linear_counter_control {
            self.linear_counter = self.linear_counter_load;
        }

        if self.linear_counter > 0 {
            self.linear_counter -= 1;
            self.update_liner_phase();
        }
    }

    pub fn update_length_counter(&mut self) {
        if self.length_counter > 0 {
            self.length_counter -= 1
        }
    }

    fn update_5step_frame(&mut self) {
        if self.frame_counter == 39
            || self.frame_counter == 78
            || self.frame_counter == 117
            || self.frame_counter == 156
            || self.frame_counter == 192
        {
            self.update_linear_counter();
        }
        if self.frame_counter == 78 || self.frame_counter == 192 {
            self.update_length_counter();
        }
        if self.frame_counter >= 192 {
            self.frame_counter = 0;
        }
    }

    fn update_4step_frame(&mut self) {
        if self.frame_counter == 60
            || self.frame_counter == 120
            || self.frame_counter == 180
            || self.frame_counter == 240
        {
            self.update_linear_counter();
        }
        if self.frame_counter == 120 || self.frame_counter == 240 {
            self.update_length_counter();
            // self.timer_update();
        }
        if self.frame_counter >= 240 {
            self.frame_counter = 0;
        }
    }

    fn insert_callback(&mut self, lock: &mut AudioDeviceLockGuard<Triangle>) {
        (*lock).call_back_linear_phase_buf.push(self.linear_phase);
        (*lock).call_back_phase_inc_buf.push(self.current_phase_inc);
    }

    fn is_signal_enable(&self, is_enable: &bool) -> bool {
        *is_enable && self.length_counter > 0 && self.current_timer >= 8
    }

    pub fn update(
        &mut self,
        frame_counter: &mut FrameCounter,
        is_enable: &mut bool,
        lock: &mut AudioDeviceLockGuard<Triangle>,
    ) {
        if self.is_signal_enable(&is_enable) {
            (*lock).clock_count += 1;
            if (*lock).clock_count >= 240 {
                (*lock).clock_count -= 240;
                self.frame_counter += 1;
                match frame_counter.mode {
                    FrameMode::_4STEP => self.update_4step_frame(),
                    FrameMode::_5STEP => self.update_5step_frame(),
                }
            }

            self.current_phase_inc =
                (1789773.0 / ((32.0 * self.current_timer as f32) + 1.0)) / 44100 as f32;
        } else {
            self.linear_phase = 0.0;
        };
        self.insert_callback(lock);
    }

    pub fn set(&mut self, addr: u8, data: u8) {
        match addr {
            0 => {
                self.length_counter_halt_or_linear_counter_control = (data & 0b10000000) != 0;
                self.linear_counter_load = data & 0b01111111;
            }
            2 => {
                self.timer &= 0x700;
                self.timer |= data as u16;
                self.current_timer = self.timer;
            }
            3 => {
                self.timer &= 0xFF;
                self.timer |= (data as u16 & 0b00000111) << 8;
                self.current_timer = self.timer;
                self.length_counter_index = (data & 0b11111000) >> 3;
                self.length_counter = APU::LENGTH_COUNTER[self.length_counter_index as usize];
                self.length_counter_halt_or_linear_counter_control = true;
            }
            _ => unimplemented!(),
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
