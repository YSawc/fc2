use crate::apu::{FrameCounter, FrameMode, APU};
use sdl2::audio::{AudioCallback, AudioDeviceLockGuard};

#[derive(Debug, Clone)]
pub struct Noise {
    current_timer: u16,
    frame_counter: u8,
    clock_count: u16,
    envelope_divider: u8,
    constant_volume_and_devider_period: u8,
    period: u8,
    is_loop_noise: bool,
    is_constant_volume: bool,
    is_loop_envelope_and_counter_halt: bool,
    volume: u8,
    length_counter_index: u8,
    length_counter: u16,
    current_volume: u8,
    call_back_volume_buf: Vec<u8>,
    current_phase_inc: f32,
    call_back_phase_inc_buf: Vec<f32>,
}

impl Default for Noise {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioCallback for Noise {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for (i, x) in out.iter_mut().enumerate() {
            if i >= self.call_back_phase_inc_buf.len() {
                break;
            }
            *x = if self.current_phase_inc <= 0.5 {
                self.call_back_volume_buf[i] as f32 * 0.003
            } else {
                self.call_back_volume_buf[i] as f32 * (-0.003)
            };
            self.current_phase_inc =
                (self.current_phase_inc + self.call_back_phase_inc_buf[i]) % 1.0;
        }
        self.call_back_volume_buf = vec![];
        self.call_back_phase_inc_buf = vec![];
    }
}

impl Noise {
    const TIMER_PERIOD: [u16; 0x10] = [
        4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
    ];

    fn new() -> Self {
        Self {
            current_timer: 0,
            frame_counter: 0,
            clock_count: 0,
            envelope_divider: 0,
            constant_volume_and_devider_period: 0,
            period: 0,
            is_loop_noise: false,
            is_loop_envelope_and_counter_halt: false,
            is_constant_volume: false,
            volume: 0,
            length_counter_index: 0,
            length_counter: 0,
            current_volume: 0,
            call_back_volume_buf: vec![],
            current_phase_inc: 0.0,
            call_back_phase_inc_buf: vec![],
        }
    }

    pub fn set(&mut self, addr: u8, data: u8) {
        match addr {
            0 => {
                self.is_constant_volume = (data & 0b00010000) != 0;
                self.is_loop_envelope_and_counter_halt = (data & 0b00100000) != 0;
                self.volume = data & 0b00001111;
            }
            2 => {
                self.is_loop_noise = (data & 0b10000000) != 0;
                self.period = data & 0b00001111;
                self.current_timer = Self::TIMER_PERIOD[self.period as usize];
            }
            3 => {
                self.length_counter_index = (data & 0b11111000) >> 3;
                self.length_counter = APU::LENGTH_COUNTER[self.length_counter_index as usize];
            }
            _ => unreachable!(),
        }
    }

    fn get_volume(&self) -> u8 {
        if self.is_constant_volume {
            self.volume
        } else {
            0x0F
        }
    }

    fn update_envelop(&mut self) {
        if self.envelope_divider > 0 {
            self.envelope_divider -= 1
        } else {
            self.envelope_divider = 15;
            if self.constant_volume_and_devider_period > 0 {
                self.constant_volume_and_devider_period -= 1;
            }
        }
    }

    fn update_length_counter(&mut self) {
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
        }
        if self.frame_counter >= 240 {
            self.frame_counter = 0;
        }
    }

    fn insert_callback(&mut self, lock: &mut AudioDeviceLockGuard<Self>) {
        (*lock).call_back_volume_buf.push(self.current_volume);
        (*lock).call_back_phase_inc_buf.push(self.current_phase_inc);
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
        lock: &mut AudioDeviceLockGuard<Self>,
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
            self.current_volume = self.get_volume();
            self.current_phase_inc =
                (1789773.0 / ((32.0 * self.current_timer as f32) + 1.0)) / 44100 as f32;
        } else {
            self.current_volume = 0;
            self.current_phase_inc = 0.0;
        };
        self.insert_callback(lock);
    }
}
