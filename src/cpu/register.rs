use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Register {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: P,
    pc: u16,
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Register {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: u8::MAX,
            p: P::default(),
            pc: 0,
        }
    }

    pub fn set_a(&mut self, data: u8) {
        self.a = data;
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_x(&mut self, data: u8) {
        self.x = data;
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn set_y(&mut self, data: u8) {
        self.y = data;
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn set_s(&mut self, data: u8) {
        self.s = data;
    }

    pub fn get_s(&self) -> u8 {
        self.s
    }

    pub fn set_p(&mut self, data: u8) {
        self.p.set(data);
    }

    pub fn get_p(&self) -> u8 {
        self.p.get()
    }

    pub fn access_p(&self) -> &P {
        &self.p
    }

    pub fn mut_access_p(&mut self) -> &mut P {
        &mut self.p
    }

    pub fn set_pc(&mut self, data: u16) {
        self.pc = data;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn inc_pc(&mut self, data: u16) {
        self.pc += data;
    }

    pub fn dec_pc(&mut self, data: u16) {
        self.pc -= data;
    }
}

#[derive(Serialize, Deserialize)]
pub struct P {
    carry: bool,
    zero: bool,
    interrupt: bool,
    decimal: bool,
    break_mode: bool,
    reserved: bool,
    overflow: bool,
    negative: bool,
}

impl Default for P {
    fn default() -> Self {
        Self::new()
    }
}

impl P {
    fn new() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt: true,
            decimal: false,
            break_mode: false,
            reserved: true,
            overflow: false,
            negative: false,
        }
    }

    fn set(&mut self, data: u8) {
        self.set_carry((data & 0b00000001) != 0);
        self.set_zero((data & 0b00000010) != 0);
        self.set_interrupt((data & 0b00000100) != 0);
        self.set_decimal((data & 0b00001000) != 0);
        self.set_break_mode((data & 0b00010000) != 0);
        self.set_reserved((data & 0b00010000) != 0);
        self.set_overflow((data & 0b01000000) != 0);
        self.set_negative((data & 0b10000000) != 0);
    }

    fn get(&self) -> u8 {
        self.to_n()
    }

    fn to_n(&self) -> u8 {
        let mut data = 0;
        data += self.get_carry() as u8 * 0b00000001;
        data += self.get_zero() as u8 * 0b00000010;
        data += self.get_interrupt() as u8 * 0b00000100;
        data += self.get_decimal() as u8 * 0b00001000;
        data += self.get_break_mode() as u8 * 0b00010000;
        data += self.get_reserved() as u8 * 0b00100000;
        data += self.get_overflow() as u8 * 0b01000000;
        data += self.get_negative() as u8 * 0b10000000;
        data
    }

    pub fn set_carry(&mut self, data: bool) {
        self.carry = data;
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }

    pub fn set_zero(&mut self, data: bool) {
        self.zero = data;
    }

    pub fn get_zero(&self) -> bool {
        self.zero
    }

    pub fn set_interrupt(&mut self, data: bool) {
        self.interrupt = data;
    }

    pub fn get_interrupt(&self) -> bool {
        self.interrupt
    }

    pub fn set_decimal(&mut self, data: bool) {
        self.decimal = data;
    }

    pub fn get_decimal(&self) -> bool {
        self.decimal
    }

    pub fn set_break_mode(&mut self, data: bool) {
        self.break_mode = data;
    }

    pub fn get_break_mode(&self) -> bool {
        self.break_mode
    }

    pub fn set_reserved(&mut self, data: bool) {
        self.reserved = data;
    }

    pub fn get_reserved(&self) -> bool {
        self.reserved
    }

    pub fn set_overflow(&mut self, data: bool) {
        self.overflow = data;
    }

    pub fn get_overflow(&self) -> bool {
        self.overflow
    }

    pub fn set_negative(&mut self, data: bool) {
        self.negative = data;
    }

    pub fn get_negative(&self) -> bool {
        self.negative
    }
}
