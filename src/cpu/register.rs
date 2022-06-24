use crate::util::*;

#[derive(Debug, Clone)]
pub struct Register {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub s: u16,
    pub p: P,
    pub pc: u16,
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 255,
            p: P::default(),
            pc: 0,
        }
    }

    pub fn set_pc(&mut self) {
        let x = self.x as u16;
        let y = self.y as u16;
        self.pc = x + (y << 8);
    }

    pub fn set_s(&mut self, n: u8) {
        self.s = (1 << 8) | (n as u16);
    }
}

#[derive(Debug, Clone)]
pub struct P {
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub break_mode: bool,
    pub reserved: u8,
    pub overflow: bool,
    pub negative: bool,
}

impl Default for P {
    fn default() -> Self {
        Self::new()
    }
}

impl P {
    pub fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            break_mode: true,
            reserved: 0,
            overflow: false,
            negative: false,
        }
    }

    pub fn set(&mut self, n: u8) {
        let s = format!("{:08b}", n);
        fn chars_nth(s: &str, n: usize) -> u32 {
            s.chars().nth(n).unwrap().to_digit(2).unwrap()
        }

        self.carry = n_to_bool(chars_nth(&s, 7));
        self.zero = n_to_bool(chars_nth(&s, 6));
        self.interrupt = n_to_bool(chars_nth(&s, 5));
        self.decimal = n_to_bool(chars_nth(&s, 4));
        self.break_mode = n_to_bool(chars_nth(&s, 3));
        self.reserved = n & 0b00000100;
        self.overflow = n_to_bool(chars_nth(&s, 1));
        self.negative = n_to_bool(chars_nth(&s, 0));
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += bool_to_n(self.carry) << 7;
        n += bool_to_n(self.zero) << 6;
        n += bool_to_n(self.interrupt) << 5;
        n += bool_to_n(self.decimal) << 4;
        n += bool_to_n(self.break_mode) << 3;
        n += self.reserved << 2;
        n += bool_to_n(self.overflow) << 1;
        n += bool_to_n(self.negative);
        n
    }
}
