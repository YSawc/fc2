pub mod mapper;
pub mod register;

use crate::nes::*;
use mapper::*;
use register::*;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub map: mapper::Map,
    pub register: Register,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        let map = Map::default();
        let register = Register::default();
        Self { map, register }
    }

    pub fn init(&mut self, nes: &Nes) {
        let prgs = &nes.header.info.prg_rom;
        if prgs.len() != 0x8000 {
            unimplemented!("prg_rom lengh is not 0x8000!");
        }
        for (i, n) in prgs.iter().enumerate() {
            match i {
                0x0000..=0x3FFF => self.map.prg_rom1[i] = *n,
                0x4000..=0x8000 => self.map.prg_rom2[i - 0x4000] = *n,
                _ => unreachable!(),
            }
        }
    }

    // pub fn handle_interrupt(&mut self, intr: Interrupt) {
    //     match intr {
    //         Interrupt::Nmi => (),
    //         Interrupt::Reset => self.reset(),
    //         Interrupt::Irq => (),
    //         Interrupt::Brk => (),
    //     }
    // }

    pub fn reset(&mut self) {
        self.read_addr(0xFFFC, 0xFFFD)
    }

    pub fn ex_plus(&mut self, l: u8, r: u8) -> u8 {
        if l.checked_add(r).is_none() {
            self.register.p.overflow = true;
            l + r - u8::MAX
        } else {
            l + r
        }
    }

    pub fn ex_minus(&mut self, l: u8, r: u8) -> u8 {
        if l < r {
            self.register.p.negative = true;
            r - l
        } else {
            l - r
        }
    }

    pub fn set_nz(&mut self, n: u8) {
        if (n >> 7) == 1 {
            self.register.p.negative = true;
        }
        if n == 0 {
            self.register.p.zero = true;
        }
    }

    pub fn fetch_code(&mut self) -> u8 {
        let pc = self.register.pc;
        self.map.addr(pc)
    }

    pub fn fetch_register(&mut self) -> u8 {
        let pc = self.register.pc;
        self.map.addr(pc)
    }

    pub fn fetch_next_register(&mut self) -> u8 {
        let pc = self.register.pc + 1;
        self.map.addr(pc)
    }

    pub fn _fetch_next_next_register(&mut self) -> u8 {
        let pc = self.register.pc + 2;
        self.map.addr(pc)
    }

    pub fn read_addr(&mut self, b: u16, u: u16) {
        self.register.x = self.map.addr(b).try_into().unwrap();
        self.register.y = self.map.addr(u) as i8;
        self.register.set_pc();
    }

    pub fn undef(&mut self) {
        self.register.pc += 1;
    }

    pub fn nop(&mut self) {
        self.register.pc += 1;
    }

    pub fn push_stack(&mut self, n: u8) {
        let b = self.register.s as u16;
        self.register.s -= 1;
        let r = b + (1 << 8);
        self.map.set(r, n);
    }

    pub fn pull_stack(&mut self) -> u8 {
        let b = self.register.s + 1;
        self.register.s += 1;
        let h = 1 << 8;
        let r = b + h;
        self.map.addr(r)
    }

    pub fn ex_ope(&mut self, opekind: OpeKind, addr_mode: AddrMode) {
        self.register.pc += 1;
        let r = match addr_mode {
            AddrMode::Acc | AddrMode::Impl => 0,
            AddrMode::Imm => self.fetch_register() as u16,
            AddrMode::Zp => {
                let b = self.fetch_register() as u16;
                b as u16
            }
            AddrMode::ZpX => {
                let mut b = self.fetch_register() as u16;
                b += self.register.x as u16;
                b as u16
            }
            AddrMode::ZpY => {
                let mut b = self.fetch_register() as u16;
                b += self.register.y as u16;
                b as u16
            }
            AddrMode::Abs => {
                let b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;
                (b + (h << 8)) as u16
            }
            AddrMode::AbsX => {
                let mut b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;
                b += self.register.x as u16;
                (b + (h << 8)) as u16
            }
            AddrMode::AbsY => {
                let mut b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;
                b += self.register.y as u16;
                (b + (h << 8)) as u16
            }
            AddrMode::Rel => {
                let b = self.register.pc + 1;
                let h = self.fetch_register() as u16;
                (b + h) as u16
            }
            AddrMode::IndX => {
                let mut b = self.fetch_register() as u16;
                b += self.register.x as u16;
                let t = b as u16;

                let b = self.map.addr(t) as u16;
                let h = self.map.addr(t + 1) as u16;
                b + (h << 8)
            }
            AddrMode::IndY => {
                let b = self.fetch_register() as u16;
                let t = b;

                let b = self.map.addr(t) as u16;
                let mut h = self.map.addr(t + 1) as u16;
                h += self.register.y as u16;
                b + (h << 8)
            }
            AddrMode::Ind => {
                let b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;

                let t = b + (h << 8);
                let b = self.map.addr(t) as u16;
                let h = self.map.addr(t + 1) as u16;

                b + (h << 8)
            }
        };
        // println!("r: {:0x?}", r);

        match addr_mode {
            AddrMode::Acc | AddrMode::Impl => (),
            AddrMode::Imm
            | AddrMode::Zp
            | AddrMode::ZpX
            | AddrMode::ZpY
            | AddrMode::Rel
            | AddrMode::IndX
            | AddrMode::IndY => self.register.pc += 1,
            AddrMode::Abs | AddrMode::AbsX | AddrMode::AbsY | AddrMode::Ind => {
                self.register.pc += 2
            }
        }

        match opekind {
            OpeKind::Adc => {
                let t = self.map.addr(r) >> 7;
                if self
                    .register
                    .a
                    .checked_add(self.map.addr(r).try_into().unwrap())
                    .is_some()
                {
                    self.register.a += self.map.addr(r) as i8;
                    if self.map.addr(r) >> 7 != t {
                        self.register.p.negative = true;
                    };
                    self.set_nz(self.register.a.try_into().unwrap());
                } else {
                    self.register.p.carry = true;
                }
            }
            OpeKind::Sbc => {
                let t = self.map.addr(r) >> 7;
                if self
                    .register
                    .a
                    .checked_sub(self.map.addr(r).try_into().unwrap())
                    .is_some()
                {
                    self.register.a -= self.map.addr(r) as i8;
                    if self.map.addr(r) >> 7 != t {
                        self.register.p.negative = true;
                    };
                    self.set_nz(self.register.a.try_into().unwrap());
                } else {
                    self.register.p.carry = true;
                }
            }
            OpeKind::And => {
                self.register.a &= self.map.addr(r) as i8;
            }
            OpeKind::Ora => {
                self.register.a |= self.map.addr(r) as i8;
            }
            OpeKind::Eor => {
                self.register.a ^= self.map.addr(r) as i8;
            }
            OpeKind::Bcc => {
                if !self.register.p.carry {
                    self.register.pc = r;
                }
            }
            OpeKind::Bcs => {
                if self.register.p.carry {
                    self.register.pc = r;
                }
            }
            OpeKind::Beq => {
                if self.register.p.zero {
                    self.register.pc = r;
                }
            }
            OpeKind::Bne => {
                if self.register.p.overflow {
                    self.register.pc = r;
                }
            }
            OpeKind::Bvc => {
                if !self.register.p.overflow {
                    self.register.pc = r;
                }
            }
            OpeKind::Bvs => {
                if self.register.p.overflow {
                    self.register.pc = r;
                }
            }
            OpeKind::Bpl => {
                if !self.register.p.negative {
                    self.register.pc = r;
                }
            }
            OpeKind::Bmi => {
                if self.register.p.negative {
                    self.register.pc = r;
                }
            }
            OpeKind::Cmp => {
                let v = self.map.addr(r) as i8;
                if self.register.a < v {
                    self.register.p.carry = false;
                } else {
                    self.register.p.carry = true;
                };
            }
            OpeKind::Cpx => {
                let v = self.map.addr(r) as i8;
                if self.register.x < v {
                    self.register.p.carry = false;
                } else {
                    self.register.p.carry = true;
                };
            }
            OpeKind::Cpy => {
                let v = self.map.addr(r) as i8;
                if self.register.y < v {
                    self.register.p.carry = false;
                } else {
                    self.register.p.carry = true;
                };
            }
            OpeKind::Inc => {
                let v = self.map.addr(r);
                let s = self.ex_plus(v, 1);
                self.map.set(r, s);
                self.set_nz(self.map.addr(r));
            }
            OpeKind::Dec => {
                let v = self.map.addr(r);
                let s = self.ex_minus(v, 1);
                self.map.set(r, s);
                self.set_nz(self.map.addr(r));
            }
            OpeKind::Inx => {
                self.register.x += 1;
                self.set_nz(self.register.x as u8);
            }
            OpeKind::Dex => {
                self.register.x -= 1;
                self.set_nz(self.register.x as u8);
            }
            OpeKind::Iny => {
                self.register.y += 1;
                self.set_nz(self.register.y as u8);
            }
            OpeKind::Dey => {
                self.register.y -= 1;
                self.set_nz(self.register.y as u8);
            }
            OpeKind::Clc => self.register.p.carry = false,
            OpeKind::Sec => self.register.p.carry = true,
            OpeKind::Cli => self.register.p.interrupt = false,
            OpeKind::Sei => self.register.p.interrupt = true,
            OpeKind::Cld => self.register.p.decimal = false,
            OpeKind::Sed => self.register.p.decimal = true,
            OpeKind::Clv => self.register.p.overflow = false,
            OpeKind::Lda => {
                self.register.a = self.map.addr(r) as i8;
                self.set_nz(self.register.a as u8);
            }
            OpeKind::Ldx => {
                self.register.x = self.map.addr(r) as i8;
                self.set_nz(self.register.x as u8);
            }
            OpeKind::Ldy => {
                self.register.y = self.map.addr(r) as i8;
                self.set_nz(self.register.y as u8);
            }
            OpeKind::Sta => {
                self.map.set(r, self.register.a as u8);
            }
            OpeKind::Stx => {
                self.map.set(r, self.register.x as u8);
            }
            OpeKind::Sty => {
                self.map.set(r, self.register.y as u8);
            }
            OpeKind::Tax => {
                self.register.x = self.register.a;
                self.register.a = 0;
                self.set_nz(self.register.x as u8);
            }
            OpeKind::Txa => {
                self.register.a = self.register.x;
                self.register.x = 0;
                self.set_nz(self.register.a as u8);
            }
            OpeKind::Tay => {
                self.register.y = self.register.a;
                self.register.a = 0;
                self.set_nz(self.register.y as u8);
            }
            OpeKind::Tya => {
                self.register.a = self.register.y;
                self.register.y = 0;
                self.set_nz(self.register.a as u8);
            }
            OpeKind::Txs => {
                self.register.set_s(self.register.x as u8);
                self.register.x = 0;
            }
            OpeKind::Pha => {
                self.register.a = self.map.addr(r).try_into().unwrap();
                self.push_stack(self.register.a as u8);
                self.register.a = 0;
            }
            OpeKind::Pla => {
                let n = self.pull_stack();
                self.register.a = n.try_into().unwrap();
                self.set_nz(self.register.a as u8);
            }
            OpeKind::Php => {
                let n = self.register.p.to_n();
                self.push_stack(n);
                self.set_nz(n);
            }
            OpeKind::Plp => {
                let n = self.pull_stack();
                self.register.p.set(n);
                let n2 = self.register.p.to_n();
                self.set_nz(n2);
            }
            OpeKind::Jmp => {
                self.register.pc = r;
            }
            OpeKind::Jsr => {
                let p = self.register.pc - 1;
                let h = ((p & 0xFF00) >> 8) as u8;
                let b = (p & 0x00FF) as u8;
                self.push_stack(h);
                self.push_stack(b);
                self.register.pc = r;
            }
            OpeKind::Rts => {
                let b = self.pull_stack() as u16;
                let h = self.pull_stack() as u16;
                let t = b + (h << 8);
                self.register.pc = t;
                self.register.pc += 1;
            }
            OpeKind::Brk => {
                if !self.register.p.interrupt {
                    self.register.pc -= 1;
                    let p = self.register.pc;
                    let h = ((p & 0xFF00) >> 8) as u8;
                    let b = (p & 0x00FF) as u8;
                    self.push_stack(h);
                    self.push_stack(b);
                    let n = self.register.p.to_n();
                    self.push_stack(n);
                    self.register.p.break_mode = true;
                    self.register.p.interrupt = true;
                    let h = self.map.addr(0xFFFE) as u16;
                    let b = self.map.addr(0xFFFF) as u16;
                    self.register.pc = b + (h << 8);
                }
            }
            OpeKind::Nop => {
                self.nop();
            }
            _ => {
                println!("opekind: {:?}, r: {:0x?} {}(10)", opekind, r, r);
                unimplemented!();
            }
        }
    }

    pub fn read_ope(&mut self) {
        let c = self.fetch_code();
        // print!("self.register.pc: {:0x?} ", self.register.pc);
        // print!(
        //     "c: {:0x?}, c1: {:0x?}, c2: {:0x?} ",
        //     c,
        //     self.fetch_next_register(),
        //     self._fetch_next_next_register()
        // );
        // print!("0x2000 {}, ", self.map.addr(0x2000));
        // print!("0x2001 {}, ", self.map.addr(0x2001));
        // print!("0x2002 {}, ", self.map.addr(0x2002));
        // print!("0x2003 {}, ", self.map.addr(0x2003));
        // print!("0x2004 {}, ", self.map.addr(0x2004));
        // print!("0x2005 {}, ", self.map.addr(0x2005));
        // print!("0x2006 {}, ", self.map.addr(0x2006));
        // println!("0x2007 {}", self.map.addr(0x2007));

        match c {
            0x00 => self.ex_ope(OpeKind::Brk, AddrMode::Impl),
            0x01 => self.ex_ope(OpeKind::Ora, AddrMode::IndX),
            0x02 => self.undef(),
            0x03 => self.undef(),
            0x04 => self.undef(),
            0x05 => self.ex_ope(OpeKind::Ora, AddrMode::Zp),
            0x06 => self.ex_ope(OpeKind::Asl, AddrMode::Zp),
            0x07 => self.undef(),
            0x08 => self.ex_ope(OpeKind::Php, AddrMode::Impl),
            0x09 => self.ex_ope(OpeKind::Ora, AddrMode::Imm),
            0x0A => self.ex_ope(OpeKind::Asl, AddrMode::Acc),
            0x0B => self.undef(),
            0x0C => self.undef(),
            0x0D => self.ex_ope(OpeKind::Ora, AddrMode::Abs),
            0x0E => self.ex_ope(OpeKind::Asl, AddrMode::Abs),
            0x0F => self.undef(),

            0x10 => self.ex_ope(OpeKind::Bpl, AddrMode::Rel),
            0x11 => self.ex_ope(OpeKind::Ora, AddrMode::IndY),
            0x12 => self.undef(),
            0x13 => self.undef(),
            0x14 => self.undef(),
            0x15 => self.ex_ope(OpeKind::Ora, AddrMode::ZpX),
            0x16 => self.ex_ope(OpeKind::Asl, AddrMode::ZpX),
            0x17 => self.undef(),
            0x18 => self.ex_ope(OpeKind::Clc, AddrMode::Impl),
            0x19 => self.ex_ope(OpeKind::Ora, AddrMode::AbsY),
            0x1A => self.undef(),
            0x1B => self.undef(),
            0x1C => self.undef(),
            0x1D => self.ex_ope(OpeKind::Ora, AddrMode::AbsX),
            0x1E => self.ex_ope(OpeKind::Asl, AddrMode::AbsX),
            0x1F => self.undef(),

            0x20 => self.ex_ope(OpeKind::Jsr, AddrMode::Abs),
            0x21 => self.ex_ope(OpeKind::And, AddrMode::IndX),
            0x22 => self.undef(),
            0x23 => self.undef(),
            0x24 => self.ex_ope(OpeKind::Bit, AddrMode::Zp),
            0x25 => self.ex_ope(OpeKind::And, AddrMode::Zp),
            0x26 => self.ex_ope(OpeKind::Rol, AddrMode::Zp),
            0x27 => self.undef(),
            0x28 => self.ex_ope(OpeKind::Plp, AddrMode::Impl),
            0x29 => self.ex_ope(OpeKind::And, AddrMode::Imm),
            0x2A => self.ex_ope(OpeKind::Rol, AddrMode::Acc),
            0x2B => self.undef(),
            0x2C => self.ex_ope(OpeKind::Bit, AddrMode::Abs),
            0x2D => self.ex_ope(OpeKind::And, AddrMode::Abs),
            0x2E => self.ex_ope(OpeKind::Rol, AddrMode::Abs),
            0x2F => self.undef(),

            0x30 => self.ex_ope(OpeKind::Bmi, AddrMode::Rel),
            0x31 => self.ex_ope(OpeKind::And, AddrMode::IndY),
            0x32 => self.undef(),
            0x33 => self.undef(),
            0x34 => self.undef(),
            0x35 => self.ex_ope(OpeKind::And, AddrMode::ZpX),
            0x36 => self.ex_ope(OpeKind::Rol, AddrMode::ZpX),
            0x37 => self.undef(),
            0x38 => self.ex_ope(OpeKind::Sec, AddrMode::Impl),
            0x39 => self.ex_ope(OpeKind::And, AddrMode::AbsY),
            0x3A => self.undef(),
            0x3B => self.undef(),
            0x3C => self.undef(),
            0x3D => self.ex_ope(OpeKind::And, AddrMode::AbsX),
            0x3E => self.ex_ope(OpeKind::Rol, AddrMode::AbsX),
            0x3F => self.undef(),

            0x40 => self.ex_ope(OpeKind::Rti, AddrMode::Impl),
            0x41 => self.ex_ope(OpeKind::Eor, AddrMode::IndX),
            0x42 => self.undef(),
            0x43 => self.undef(),
            0x44 => self.undef(),
            0x45 => self.ex_ope(OpeKind::Eor, AddrMode::Zp),
            0x46 => self.ex_ope(OpeKind::Lsr, AddrMode::Zp),
            0x47 => self.undef(),
            0x48 => self.ex_ope(OpeKind::Pha, AddrMode::Impl),
            0x49 => self.ex_ope(OpeKind::Eor, AddrMode::Imm),
            0x4A => self.ex_ope(OpeKind::Lsr, AddrMode::Acc),
            0x4B => self.undef(),
            0x4C => self.ex_ope(OpeKind::Jmp, AddrMode::Abs),
            0x4D => self.ex_ope(OpeKind::Eor, AddrMode::Abs),
            0x4E => self.ex_ope(OpeKind::Lsr, AddrMode::Abs),
            0x4F => self.undef(),

            0x50 => self.ex_ope(OpeKind::Bvc, AddrMode::Rel),
            0x51 => self.ex_ope(OpeKind::Eor, AddrMode::IndY),
            0x52 => self.undef(),
            0x53 => self.undef(),
            0x54 => self.undef(),
            0x55 => self.ex_ope(OpeKind::Eor, AddrMode::ZpX),
            0x56 => self.ex_ope(OpeKind::Lsr, AddrMode::ZpX),
            0x57 => self.undef(),
            0x58 => self.ex_ope(OpeKind::Cli, AddrMode::Impl),
            0x59 => self.ex_ope(OpeKind::Eor, AddrMode::AbsY),
            0x5A => self.undef(),
            0x5B => self.undef(),
            0x5C => self.undef(),
            0x5D => self.ex_ope(OpeKind::Eor, AddrMode::AbsX),
            0x5E => self.ex_ope(OpeKind::Lsr, AddrMode::AbsX),
            0x5F => self.undef(),

            0x60 => self.ex_ope(OpeKind::Rts, AddrMode::Impl),
            0x61 => self.ex_ope(OpeKind::Adc, AddrMode::IndX),
            0x62 => self.undef(),
            0x63 => self.undef(),
            0x64 => self.undef(),
            0x65 => self.ex_ope(OpeKind::Adc, AddrMode::Zp),
            0x66 => self.ex_ope(OpeKind::Ror, AddrMode::Zp),
            0x67 => self.undef(),
            0x68 => self.ex_ope(OpeKind::Pla, AddrMode::Impl),
            0x69 => self.ex_ope(OpeKind::Adc, AddrMode::Imm),
            0x6A => self.ex_ope(OpeKind::Ror, AddrMode::Acc),
            0x6B => self.undef(),
            0x6C => self.ex_ope(OpeKind::Jmp, AddrMode::Ind),
            0x6D => self.ex_ope(OpeKind::Adc, AddrMode::Abs),
            0x6E => self.ex_ope(OpeKind::Ror, AddrMode::Abs),
            0x6F => self.undef(),

            0x70 => self.ex_ope(OpeKind::Bvs, AddrMode::Rel),
            0x71 => self.ex_ope(OpeKind::Adc, AddrMode::IndY),
            0x72 => self.undef(),
            0x73 => self.undef(),
            0x74 => self.undef(),
            0x75 => self.ex_ope(OpeKind::Adc, AddrMode::ZpX),
            0x76 => self.ex_ope(OpeKind::Ror, AddrMode::ZpX),
            0x77 => self.undef(),
            0x78 => self.ex_ope(OpeKind::Sei, AddrMode::Impl),
            0x79 => self.ex_ope(OpeKind::Adc, AddrMode::AbsY),
            0x7A => self.undef(),
            0x7B => self.undef(),
            0x7C => self.undef(),
            0x7D => self.ex_ope(OpeKind::Adc, AddrMode::AbsX),
            0x7E => self.ex_ope(OpeKind::Ror, AddrMode::AbsX),
            0x7F => self.undef(),

            0x80 => self.undef(),
            0x81 => self.ex_ope(OpeKind::Sta, AddrMode::IndX),
            0x82 => self.undef(),
            0x83 => self.undef(),
            0x84 => self.ex_ope(OpeKind::Sty, AddrMode::Zp),
            0x85 => self.ex_ope(OpeKind::Sta, AddrMode::Zp),
            0x86 => self.ex_ope(OpeKind::Stx, AddrMode::Zp),
            0x87 => self.undef(),
            0x88 => self.ex_ope(OpeKind::Dey, AddrMode::Impl),
            0x89 => self.undef(),
            0x8A => self.ex_ope(OpeKind::Txa, AddrMode::Impl),
            0x8B => self.undef(),
            0x8C => self.ex_ope(OpeKind::Sty, AddrMode::Abs),
            0x8D => self.ex_ope(OpeKind::Sta, AddrMode::Abs),
            0x8E => self.ex_ope(OpeKind::Stx, AddrMode::Abs),
            0x8F => self.undef(),

            0x90 => self.ex_ope(OpeKind::Bcc, AddrMode::Rel),
            0x91 => self.ex_ope(OpeKind::Sta, AddrMode::IndY),
            0x92 => self.undef(),
            0x93 => self.undef(),
            0x94 => self.ex_ope(OpeKind::Sty, AddrMode::ZpX),
            0x95 => self.ex_ope(OpeKind::Sta, AddrMode::ZpX),
            0x96 => self.ex_ope(OpeKind::Stx, AddrMode::ZpY),
            0x97 => self.undef(),
            0x98 => self.ex_ope(OpeKind::Tya, AddrMode::Impl),
            0x99 => self.ex_ope(OpeKind::Sta, AddrMode::AbsY),
            0x9A => self.ex_ope(OpeKind::Txs, AddrMode::Impl),
            0x9B => self.undef(),
            0x9C => self.undef(),
            0x9D => self.ex_ope(OpeKind::Sta, AddrMode::AbsX),
            0x9E => self.undef(),
            0x9F => self.undef(),

            0xA0 => self.ex_ope(OpeKind::Ldy, AddrMode::Imm),
            0xA1 => self.ex_ope(OpeKind::Lda, AddrMode::IndX),
            0xA2 => self.ex_ope(OpeKind::Ldx, AddrMode::Imm),
            0xA3 => self.undef(),
            0xA4 => self.ex_ope(OpeKind::Ldy, AddrMode::Zp),
            0xA5 => self.ex_ope(OpeKind::Lda, AddrMode::Zp),
            0xA6 => self.ex_ope(OpeKind::Ldx, AddrMode::Zp),
            0xA7 => self.undef(),
            0xA8 => self.ex_ope(OpeKind::Tay, AddrMode::Impl),
            0xA9 => self.ex_ope(OpeKind::Lda, AddrMode::Imm),
            0xAA => self.ex_ope(OpeKind::Tax, AddrMode::Impl),
            0xAB => self.undef(),
            0xAC => self.ex_ope(OpeKind::Ldy, AddrMode::Abs),
            0xAD => self.ex_ope(OpeKind::Lda, AddrMode::Abs),
            0xAE => self.ex_ope(OpeKind::Ldx, AddrMode::Abs),
            0xAF => self.undef(),

            0xB0 => self.ex_ope(OpeKind::Bcs, AddrMode::Rel),
            0xB1 => self.ex_ope(OpeKind::Lda, AddrMode::IndY),
            0xB2 => self.undef(),
            0xB3 => self.undef(),
            0xB4 => self.ex_ope(OpeKind::Ldy, AddrMode::ZpX),
            0xB5 => self.ex_ope(OpeKind::Lda, AddrMode::ZpX),
            0xB6 => self.ex_ope(OpeKind::Ldx, AddrMode::ZpY),
            0xB7 => self.undef(),
            0xB8 => self.ex_ope(OpeKind::Clv, AddrMode::Impl),
            0xB9 => self.ex_ope(OpeKind::Lda, AddrMode::AbsY),
            0xBA => self.ex_ope(OpeKind::Tsx, AddrMode::Impl),
            0xBB => self.undef(),
            0xBC => self.ex_ope(OpeKind::Ldy, AddrMode::AbsX),
            0xBD => self.ex_ope(OpeKind::Lda, AddrMode::AbsX),
            0xBE => self.ex_ope(OpeKind::Ldx, AddrMode::AbsY),
            0xBF => self.undef(),

            0xC0 => self.ex_ope(OpeKind::Cpy, AddrMode::Imm),
            0xC1 => self.ex_ope(OpeKind::Cmp, AddrMode::IndX),
            0xC2 => self.undef(),
            0xC3 => self.undef(),
            0xC4 => self.ex_ope(OpeKind::Cpy, AddrMode::Zp),
            0xC5 => self.ex_ope(OpeKind::Cmp, AddrMode::Zp),
            0xC6 => self.ex_ope(OpeKind::Dec, AddrMode::Zp),
            0xC7 => self.undef(),
            0xC8 => self.ex_ope(OpeKind::Iny, AddrMode::Impl),
            0xC9 => self.ex_ope(OpeKind::Cmp, AddrMode::Imm),
            0xCA => self.ex_ope(OpeKind::Dex, AddrMode::Impl),
            0xCB => self.undef(),
            0xCC => self.ex_ope(OpeKind::Cpy, AddrMode::Abs),
            0xCD => self.ex_ope(OpeKind::Cmp, AddrMode::Abs),
            0xCE => self.ex_ope(OpeKind::Dec, AddrMode::Abs),
            0xCF => self.undef(),

            0xD0 => self.ex_ope(OpeKind::Bne, AddrMode::Rel),
            0xD1 => self.ex_ope(OpeKind::Cmp, AddrMode::IndY),
            0xD2 => self.undef(),
            0xD3 => self.undef(),
            0xD4 => self.undef(),
            0xD5 => self.ex_ope(OpeKind::Cmp, AddrMode::ZpX),
            0xD6 => self.ex_ope(OpeKind::Dec, AddrMode::ZpX),
            0xD7 => self.undef(),
            0xD8 => self.ex_ope(OpeKind::Cld, AddrMode::Impl),
            0xD9 => self.ex_ope(OpeKind::Cmp, AddrMode::AbsY),
            0xDA => self.undef(),
            0xDB => self.undef(),
            0xDC => self.undef(),
            0xDD => self.ex_ope(OpeKind::Cmp, AddrMode::AbsX),
            0xDE => self.ex_ope(OpeKind::Dec, AddrMode::AbsX),
            0xDF => self.undef(),

            0xE0 => self.ex_ope(OpeKind::Cpx, AddrMode::Imm),
            0xE1 => self.ex_ope(OpeKind::Sbc, AddrMode::IndX),
            0xE2 => self.undef(),
            0xE3 => self.undef(),
            0xE4 => self.ex_ope(OpeKind::Cpx, AddrMode::Zp),
            0xE5 => self.ex_ope(OpeKind::Sbc, AddrMode::Zp),
            0xE6 => self.ex_ope(OpeKind::Inc, AddrMode::Zp),
            0xE7 => self.undef(),
            0xE8 => self.ex_ope(OpeKind::Inx, AddrMode::Impl),
            0xE9 => self.ex_ope(OpeKind::Sbc, AddrMode::Imm),
            0xEA => self.ex_ope(OpeKind::Nop, AddrMode::Impl),
            0xEB => self.undef(),
            0xEC => self.ex_ope(OpeKind::Cpx, AddrMode::Abs),
            0xED => self.ex_ope(OpeKind::Sbc, AddrMode::Abs),
            0xEE => self.ex_ope(OpeKind::Inc, AddrMode::Abs),
            0xEF => self.undef(),

            0xF0 => self.ex_ope(OpeKind::Beq, AddrMode::Rel),
            0xF1 => self.ex_ope(OpeKind::Sbc, AddrMode::IndY),
            0xF2 => self.undef(),
            0xF3 => self.undef(),
            0xF4 => self.undef(),
            0xF5 => self.ex_ope(OpeKind::Sbc, AddrMode::ZpX),
            0xF6 => self.ex_ope(OpeKind::Inc, AddrMode::ZpX),
            0xF7 => self.undef(),
            0xF8 => self.ex_ope(OpeKind::Sed, AddrMode::Impl),
            0xF9 => self.ex_ope(OpeKind::Sbc, AddrMode::AbsY),
            0xFA => self.undef(),
            0xFB => self.undef(),
            0xFC => self.undef(),
            0xFD => self.ex_ope(OpeKind::Sbc, AddrMode::AbsX),
            0xFE => self.ex_ope(OpeKind::Inc, AddrMode::AbsX),
            0xFF => self.undef(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddrMode {
    Acc,
    Imm,
    Abs,
    AbsX,
    AbsY,
    Zp,
    ZpX,
    ZpY,
    Impl,
    Rel,
    IndX,
    IndY,
    Ind,
}

#[derive(Debug, Clone)]
pub enum OpeKind {
    Adc,
    Sbc, // flags: N V Z C

    And,
    Ora,
    Eor, // flags: N Z

    Asl,
    Lsr,
    Rol,
    Ror, // flags: N Z C

    Bcc,
    Bcs,
    Beq,
    Bne,
    Bvc,
    Bvs,
    Bpl,
    Bmi, // flags: none

    Bit, // flags: N V Z

    Jmp,
    Jsr,
    Rts, // flags: none

    Brk, // flags: Bi
    Rti, // flags: all

    Cmp,
    Cpx,
    Cpy,
    Inc,
    Dec,
    Inx,
    Dex,
    Iny,
    Dey, // flags: N Z

    Clc,
    Sec,
    Cli,
    Sei,
    Cld,
    Sed,
    Clv, // flags: N Z

    Lda,
    Ldx,
    Ldy, // flags: N Z

    Sta,
    Stx,
    Sty, // flags: none

    Tax,
    Txa,
    Tay,
    Tya,
    Tsx, // flags: N Z
    Txs, // flags: none

    Pha, // flags: none
    Pla, // flags: N Z
    Php, // flags: none
    Plp, // flags: all
    Nop, // flags: none
}

// #[derive(Debug, Clone)]
// pub enum OpeFlags {
//     Nvzc,
//     Nz,
//     Nzc,
//     Nvz,
//     Bi,
//     All,
//     None,
// }

// #[derive(Debug, Clone)]
// pub enum Interrupt {
//     Nmi,
//     Reset,
//     Irq,
//     Brk,
// }
