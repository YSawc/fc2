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
        self.s = n as u16;
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
            interrupt: true,
            decimal: false,
            break_mode: false,
            reserved: 1,
            overflow: false,
            negative: false,
        }
    }

    pub fn set(&mut self, n: u8) {
        self.carry = (n & 0b00000001) != 0;
        self.zero = (n & 0b00000010) != 0;
        self.interrupt = (n & 0b00000100) != 0;
        self.decimal = (n & 0b00001000) != 0;
        self.break_mode = (n & 0b00010000) != 0;
        self.reserved = 1;
        self.overflow = (n & 0b01000000) != 0;
        self.negative = (n & 0b10000000) != 0;
    }

    pub fn to_n(&mut self) -> u8 {
        let mut n = 0;
        n += self.carry as u8 * 0b00000001;
        n += self.zero as u8 * 0b00000010;
        n += self.interrupt as u8 * 0b00000100;
        n += self.decimal as u8 * 0b00001000;
        n += self.break_mode as u8 * 0b00010000;
        n += self.reserved as u8 * 0b00100000;
        n += self.overflow as u8 * 0b01000000;
        n += self.negative as u8 * 0b10000000;
        n
    }
}
