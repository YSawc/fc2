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

    pub fn bool_to_n(&mut self, b: bool) -> u8 {
        match b {
            true => 1,
            false => 0,
        }
    }

    pub fn s_to_bool(&mut self, n: u32) -> bool {
        match n {
            1 => true,
            0 => false,
            _ => unimplemented!(),
        }
    }

    pub fn set(&mut self, n: u8) {
        let s = format!("{:08b}", n);
        fn chars_nth(s: &str, n: usize) -> u32 {
            s.chars().nth(n).unwrap().to_digit(2).unwrap()
        }

        self.carry = self.s_to_bool(chars_nth(&s, 7));
        self.zero = self.s_to_bool(chars_nth(&s, 6));
        self.interrupt = self.s_to_bool(chars_nth(&s, 5));
        self.decimal = self.s_to_bool(chars_nth(&s, 4));
        self.break_mode = self.s_to_bool(chars_nth(&s, 3));
        self.reserved = match chars_nth(&s, 2) {
            1 => 1,
            0 => 0,
            _ => unimplemented!(),
        };
        self.overflow = self.s_to_bool(chars_nth(&s, 1));
        self.negative = self.s_to_bool(chars_nth(&s, 0));
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += self.bool_to_n(self.carry) << 7;
        n += self.bool_to_n(self.zero) << 6;
        n += self.bool_to_n(self.interrupt) << 5;
        n += self.bool_to_n(self.decimal) << 4;
        n += self.bool_to_n(self.break_mode) << 3;
        n += self.reserved << 2;
        n += self.bool_to_n(self.overflow) << 1;
        n += self.bool_to_n(self.negative);
        n
    }
}
