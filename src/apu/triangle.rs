use crate::apu::{FrameCounter, FrameMode, APU};
use sdl2::audio::{AudioCallback, AudioDeviceLockGuard};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triangle {
    timer: u16,
    current_timer: u16,
    frame_counter: u8,
    clock_count: u16,
    controll_flag: bool,
    length_counter_halt_or_linear_counter_control: bool,
    linear_counter_load: u8,
    linear_counter: u8,
    linear_phase: f32,
    call_back_linear_phase_buf: Vec<f32>,
    linear_inc_phase: bool,
    call_back_phase_inc_buf: Vec<f32>,
    length_counter_index: u8,
    length_counter: u16,
    current_phase_inc: f32,
    is_loop_envelope_and_counter_halt: bool,
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

    fn update_liner_phase(&mut self) {
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

    fn update_linear_counter(&mut self) {
        if !self.controll_flag {
            self.length_counter_halt_or_linear_counter_control = false;
        }

        if self.length_counter_halt_or_linear_counter_control {
            self.linear_counter = self.linear_counter_load;
        }

        if self.linear_counter > 0 {
            self.linear_counter -= 1;
            self.update_liner_phase();
        }
    }

    fn update_length_counter(&mut self) {
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
        }
        if self.frame_counter >= 240 {
            self.frame_counter = 0;
        }
    }

    fn insert_callback(&mut self, lock: &mut AudioDeviceLockGuard<Self>) {
        (*lock).call_back_linear_phase_buf.push(self.linear_phase);
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
                self.controll_flag = (data & 0b10000000) != 0;
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
            _ => unreachable!(),
        }
    }
}
