pub mod mapper;
pub mod operator;
pub mod register;

use crate::nes::*;
use crate::util::*;
use mapper::*;
use operator::*;
use register::*;

use rustc_hash::*;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub map: mapper::Map,
    pub register: Register,
    pub operators: FxHashMap<u8, Operator>,
}

impl Default for Cpu {
    fn default() -> Self {
        let mut cpu = Self::new();
        cpu.prepare_operators();
        cpu
    }
}

impl Cpu {
    pub fn new() -> Self {
        let map = Map::default();
        let register = Register::default();
        let operators = FxHashMap::default();

        Self {
            map,
            register,
            operators,
        }
    }

    fn prepare_operators(&mut self) {
        let mut operators = FxHashMap::default();
        macro_rules! ope_reserved {
            ( $($id:expr => ($ope_kind:path, $addr_mode:path)),+ ) => {
                $(
                    operators.insert(
                        $id,
                        Operator {
                            ope_kind: $ope_kind,
                            addr_mode: $addr_mode
                        }
                    );
                )+
            };
        }
        ope_reserved! {
            0x00 => (OpeKind::Brk, AddrMode::Impl),
            0x01 => (OpeKind::Ora, AddrMode::IndX),
            0x05 => (OpeKind::Ora, AddrMode::Zp),
            0x06 => (OpeKind::Asl, AddrMode::Zp),
            0x08 => (OpeKind::Php, AddrMode::Impl),
            0x09 => (OpeKind::Ora, AddrMode::Imm),
            0x0A => (OpeKind::Asl, AddrMode::Acc),
            0x0D => (OpeKind::Ora, AddrMode::Abs),
            0x0E => (OpeKind::Asl, AddrMode::Abs),

            0x10 => (OpeKind::Bpl, AddrMode::Rel),
            0x11 => (OpeKind::Ora, AddrMode::IndY),
            0x15 => (OpeKind::Ora, AddrMode::ZpX),
            0x16 => (OpeKind::Asl, AddrMode::ZpX),
            0x18 => (OpeKind::Clc, AddrMode::Impl),
            0x19 => (OpeKind::Ora, AddrMode::AbsY),
            0x1D => (OpeKind::Ora, AddrMode::AbsX),
            0x1E => (OpeKind::Asl, AddrMode::AbsX),

            0x20 => (OpeKind::Jsr, AddrMode::Abs),
            0x21 => (OpeKind::And, AddrMode::IndX),
            0x24 => (OpeKind::Bit, AddrMode::Zp),
            0x25 => (OpeKind::And, AddrMode::Zp),
            0x26 => (OpeKind::Rol, AddrMode::Zp),
            0x28 => (OpeKind::Plp, AddrMode::Impl),
            0x29 => (OpeKind::And, AddrMode::Imm),
            0x2A => (OpeKind::Rol, AddrMode::Acc),
            0x2C => (OpeKind::Bit, AddrMode::Abs),
            0x2D => (OpeKind::And, AddrMode::Abs),
            0x2E => (OpeKind::Rol, AddrMode::Abs),

            0x30 => (OpeKind::Bmi, AddrMode::Rel),
            0x31 => (OpeKind::And, AddrMode::IndY),
            0x35 => (OpeKind::And, AddrMode::ZpX),
            0x36 => (OpeKind::Rol, AddrMode::ZpX),
            0x38 => (OpeKind::Sec, AddrMode::Impl),
            0x39 => (OpeKind::And, AddrMode::AbsY),
            0x3D => (OpeKind::And, AddrMode::AbsX),
            0x3E => (OpeKind::Rol, AddrMode::AbsX),

            0x40 => (OpeKind::Rti, AddrMode::Impl),
            0x41 => (OpeKind::Eor, AddrMode::IndX),
            0x45 => (OpeKind::Eor, AddrMode::Zp),
            0x46 => (OpeKind::Lsr, AddrMode::Zp),
            0x48 => (OpeKind::Pha, AddrMode::Impl),
            0x49 => (OpeKind::Eor, AddrMode::Imm),
            0x4A => (OpeKind::Lsr, AddrMode::Acc),
            0x4C => (OpeKind::Jmp, AddrMode::Abs),
            0x4D => (OpeKind::Eor, AddrMode::Abs),
            0x4E => (OpeKind::Lsr, AddrMode::Abs),

            0x50 => (OpeKind::Bvc, AddrMode::Rel),
            0x51 => (OpeKind::Eor, AddrMode::IndY),
            0x55 => (OpeKind::Eor, AddrMode::ZpX),
            0x56 => (OpeKind::Lsr, AddrMode::ZpX),
            0x58 => (OpeKind::Cli, AddrMode::Impl),
            0x59 => (OpeKind::Eor, AddrMode::AbsY),
            0x5D => (OpeKind::Eor, AddrMode::AbsX),
            0x5E => (OpeKind::Lsr, AddrMode::AbsX),

            0x60 => (OpeKind::Rts, AddrMode::Impl),
            0x61 => (OpeKind::Adc, AddrMode::IndX),
            0x65 => (OpeKind::Adc, AddrMode::Zp),
            0x66 => (OpeKind::Ror, AddrMode::Zp),
            0x68 => (OpeKind::Pla, AddrMode::Impl),
            0x69 => (OpeKind::Adc, AddrMode::Imm),
            0x6A => (OpeKind::Ror, AddrMode::Acc),
            0x6C => (OpeKind::Jmp, AddrMode::Ind),
            0x6D => (OpeKind::Adc, AddrMode::Abs),
            0x6E => (OpeKind::Ror, AddrMode::Abs),

            0x70 => (OpeKind::Bvs, AddrMode::Rel),
            0x71 => (OpeKind::Adc, AddrMode::IndY),
            0x75 => (OpeKind::Adc, AddrMode::ZpX),
            0x76 => (OpeKind::Ror, AddrMode::ZpX),
            0x78 => (OpeKind::Sei, AddrMode::Impl),
            0x79 => (OpeKind::Adc, AddrMode::AbsY),
            0x7D => (OpeKind::Adc, AddrMode::AbsX),
            0x7E => (OpeKind::Ror, AddrMode::AbsX),

            0x81 => (OpeKind::Sta, AddrMode::IndX),
            0x84 => (OpeKind::Sty, AddrMode::Zp),
            0x85 => (OpeKind::Sta, AddrMode::Zp),
            0x86 => (OpeKind::Stx, AddrMode::Zp),
            0x88 => (OpeKind::Dey, AddrMode::Impl),
            0x8A => (OpeKind::Txa, AddrMode::Impl),
            0x8C => (OpeKind::Sty, AddrMode::Abs),
            0x8D => (OpeKind::Sta, AddrMode::Abs),
            0x8E => (OpeKind::Stx, AddrMode::Abs),

            0x90 => (OpeKind::Bcc, AddrMode::Rel),
            0x91 => (OpeKind::Sta, AddrMode::IndY),
            0x94 => (OpeKind::Sty, AddrMode::ZpX),
            0x95 => (OpeKind::Sta, AddrMode::ZpX),
            0x96 => (OpeKind::Stx, AddrMode::ZpY),
            0x98 => (OpeKind::Tya, AddrMode::Impl),
            0x99 => (OpeKind::Sta, AddrMode::AbsY),
            0x9A => (OpeKind::Txs, AddrMode::Impl),
            0x9D => (OpeKind::Sta, AddrMode::AbsX),

            0xA0 => (OpeKind::Ldy, AddrMode::Imm),
            0xA1 => (OpeKind::Lda, AddrMode::IndX),
            0xA2 => (OpeKind::Ldx, AddrMode::Imm),
            0xA4 => (OpeKind::Ldy, AddrMode::Zp),
            0xA5 => (OpeKind::Lda, AddrMode::Zp),
            0xA6 => (OpeKind::Ldx, AddrMode::Zp),
            0xA8 => (OpeKind::Tay, AddrMode::Impl),
            0xA9 => (OpeKind::Lda, AddrMode::Imm),
            0xAA => (OpeKind::Tax, AddrMode::Impl),
            0xAC => (OpeKind::Ldy, AddrMode::Abs),
            0xAD => (OpeKind::Lda, AddrMode::Abs),
            0xAE => (OpeKind::Ldx, AddrMode::Abs),

            0xB0 => (OpeKind::Bcs, AddrMode::Rel),
            0xB1 => (OpeKind::Lda, AddrMode::IndY),
            0xB4 => (OpeKind::Ldy, AddrMode::ZpX),
            0xB5 => (OpeKind::Lda, AddrMode::ZpX),
            0xB6 => (OpeKind::Ldx, AddrMode::ZpY),
            0xB8 => (OpeKind::Clv, AddrMode::Impl),
            0xB9 => (OpeKind::Lda, AddrMode::AbsY),
            0xBA => (OpeKind::Tsx, AddrMode::Impl),
            0xBC => (OpeKind::Ldy, AddrMode::AbsX),
            0xBD => (OpeKind::Lda, AddrMode::AbsX),
            0xBE => (OpeKind::Ldx, AddrMode::AbsY),

            0xC0 => (OpeKind::Cpy, AddrMode::Imm),
            0xC1 => (OpeKind::Cmp, AddrMode::IndX),
            0xC4 => (OpeKind::Cpy, AddrMode::Zp),
            0xC5 => (OpeKind::Cmp, AddrMode::Zp),
            0xC6 => (OpeKind::Dec, AddrMode::Zp),
            0xC8 => (OpeKind::Iny, AddrMode::Impl),
            0xC9 => (OpeKind::Cmp, AddrMode::Imm),
            0xCA => (OpeKind::Dex, AddrMode::Impl),
            0xCC => (OpeKind::Cpy, AddrMode::Abs),
            0xCD => (OpeKind::Cmp, AddrMode::Abs),
            0xCE => (OpeKind::Dec, AddrMode::Abs),

            0xD0 => (OpeKind::Bne, AddrMode::Rel),
            0xD1 => (OpeKind::Cmp, AddrMode::IndY),
            0xD5 => (OpeKind::Cmp, AddrMode::ZpX),
            0xD6 => (OpeKind::Dec, AddrMode::ZpX),
            0xD8 => (OpeKind::Cld, AddrMode::Impl),
            0xD9 => (OpeKind::Cmp, AddrMode::AbsY),
            0xDD => (OpeKind::Cmp, AddrMode::AbsX),
            0xDE => (OpeKind::Dec, AddrMode::AbsX),

            0xE0 => (OpeKind::Cpx, AddrMode::Imm),
            0xE1 => (OpeKind::Sbc, AddrMode::IndX),
            0xE4 => (OpeKind::Cpx, AddrMode::Zp),
            0xE5 => (OpeKind::Sbc, AddrMode::Zp),
            0xE6 => (OpeKind::Inc, AddrMode::Zp),
            0xE8 => (OpeKind::Inx, AddrMode::Impl),
            0xE9 => (OpeKind::Sbc, AddrMode::Imm),
            0xEA => (OpeKind::Nop, AddrMode::Impl),
            0xEC => (OpeKind::Cpx, AddrMode::Abs),
            0xED => (OpeKind::Sbc, AddrMode::Abs),
            0xEE => (OpeKind::Inc, AddrMode::Abs),

            0xF0 => (OpeKind::Beq, AddrMode::Rel),
            0xF1 => (OpeKind::Sbc, AddrMode::IndY),
            0xF5 => (OpeKind::Sbc, AddrMode::ZpX),
            0xF6 => (OpeKind::Inc, AddrMode::ZpX),
            0xF8 => (OpeKind::Sed, AddrMode::Impl),
            0xF9 => (OpeKind::Sbc, AddrMode::AbsY),
            0xFD => (OpeKind::Sbc, AddrMode::AbsX),
            0xFE => (OpeKind::Inc, AddrMode::AbsX)
        };
        self.operators = operators;
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

    pub fn fetch_next_next_register(&mut self) -> u8 {
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

    pub fn ex_addr_mode(&mut self, addr_mode: AddrMode) -> u16 {
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
                ((h << 8) | b) as u16
            }
            AddrMode::AbsX => {
                let mut b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;
                b += self.register.x as u16;
                ((h << 8) | b) as u16
            }
            AddrMode::AbsY => {
                let mut b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;
                b += self.register.y as u16;
                ((h << 8) | b) as u16
            }
            AddrMode::Rel => {
                let b = self.register.pc + 1;
                let h = self.fetch_register() as u16;
                (b + h) as u16
            }
            AddrMode::IndX => {
                let l = self.fetch_register();
                let r = self.register.x as u8;
                let t = ex_plus_ignoring_overflow(l, r) as u16;

                let b = self.map.addr(t) as u16;
                let h = self.map.addr(t + 1) as u16;
                (h << 8) | b
            }
            AddrMode::IndY => {
                let t = self.fetch_register() as u16;

                let b = self.map.addr(t) as u16;
                let h = self.map.addr(t + 1) as u16;
                let l = ((h << 8) | b) as u8;
                let r = self.register.y as u8;
                ex_plus_ignoring_overflow(l, r) as u16
            }
            AddrMode::Ind => {
                let b = self.fetch_register() as u16;
                let h = self.fetch_next_register() as u16;

                let t = (h << 8) | b;
                let b = self.map.addr(t) as u16;
                let h = self.map.addr(t + 1) as u16;

                (h << 8) | b
            }
        };

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

        r
    }

    pub fn run_ope(&mut self, r: u16, opekind: OpeKind) {
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
                let t = (h << 8) | b;
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
                    self.register.pc = (h << 8) | b;
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

    pub fn ex_ope(&mut self) {
        match self.read_ope() {
            Some(Operator {
                ope_kind,
                addr_mode,
            }) => {
                let ope_kind = ope_kind.clone();
                let addr_mode = addr_mode.clone();
                let reg_addr = self.ex_addr_mode(addr_mode);
                self.run_ope(reg_addr, ope_kind);
            }
            None => self.undef(),
        }
    }

    pub fn read_ope(&mut self) -> Option<&Operator> {
        let c = self.fetch_code();
        // print!("self.register.pc: {:0x?} ", self.register.pc);
        // print!(
        //     "c: {:0x?}, c1: {:0x?}, c2: {:0x?} ",
        //     c,
        //     self.fetch_next_register(),
        //     self.fetch_next_next_register()
        // );
        // print!("0x2000 {}, ", self.map.addr(0x2000));
        // print!("0x2001 {}, ", self.map.addr(0x2001));
        // print!("0x2002 {}, ", self.map.addr(0x2002));
        // print!("0x2003 {}, ", self.map.addr(0x2003));
        // print!("0x2004 {}, ", self.map.addr(0x2004));
        // print!("0x2005 {}, ", self.map.addr(0x2005));
        // print!("0x2006 {}, ", self.map.addr(0x2006));
        // println!("0x2007 {}", self.map.addr(0x2007));

        match self.operators.get_mut(&c) {
            Some(operator) => Some(operator),
            None => None,
        }
    }
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

#[cfg(test)]
mod test {
    use crate::cpu::*;
    extern crate rand;
    use rand::seq::IteratorRandom;

    impl Cpu {
        fn random_pick_operator_with_specify_ope_kind(&self, like_kind: OpeKind) -> (u8, Operator) {
            let picked_operators = self
                .operators
                .iter()
                .filter(|(_, Operator { ope_kind, .. })| *ope_kind == like_kind);

            let mut rng = rand::thread_rng();
            let (code, operator) = picked_operators.choose(&mut rng).unwrap();
            (*code, operator.clone())
        }

        fn random_pick_operator_with_specify_addr_mode(
            &self,
            like_mode: AddrMode,
        ) -> (u8, &Operator) {
            let picked_operators = self
                .operators
                .iter()
                .filter(|(_, Operator { addr_mode, .. })| *addr_mode == like_mode);

            let mut rng = rand::thread_rng();
            let (code, operator) = picked_operators.choose(&mut rng).unwrap();
            (*code, operator)
        }

        fn set_next_reg_addr(&mut self, reg_addr: &mut u16) {
            match self.read_ope() {
                Some(Operator { addr_mode, .. }) => {
                    let addr_mode = addr_mode.clone();
                    *reg_addr = self.ex_addr_mode(addr_mode);
                }
                None => (),
            };
        }

        fn insert_random_num_into_b1_b2(&mut self) {
            self.map.prg_rom1[1] = rand_u8();
            self.map.prg_rom1[2] = rand_u8();
        }
    }

    fn rand_u8() -> u8 {
        use crate::cpu::test::rand::Rng;

        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen();
        n
    }

    fn prepare_cpu_for_addr_mode_test(addr_mode: AddrMode) -> Cpu {
        let nes = Nes::new();
        let mut cpu = Cpu::default();
        cpu.init(&nes);
        cpu.reset();
        let (code, _) = cpu.random_pick_operator_with_specify_addr_mode(addr_mode);
        cpu.map.prg_rom1[0] = code;
        cpu
    }

    #[test]
    fn acc_not_specify_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Acc);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        let fetched_next_addr = cpu.fetch_next_register() as u16;
        let fetched_next_next_addr = cpu.fetch_next_next_register() as u16;

        assert_eq!(reg_addr, 0);
        assert_ne!(reg_addr, fetched_next_addr);
        assert_ne!(reg_addr, fetched_next_next_addr);
    }

    #[test]
    fn impl_not_specify_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Impl);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        let fetched_next_addr = cpu.fetch_next_register() as u16;
        let fetched_next_next_addr = cpu.fetch_next_next_register() as u16;

        assert_eq!(reg_addr, 0);
        assert_ne!(reg_addr, fetched_next_addr);
        assert_ne!(reg_addr, fetched_next_next_addr);
    }

    #[test]
    fn imm_specify_immediate_register_as_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Imm);
        cpu.insert_random_num_into_b1_b2();

        let fetched_next_addr = cpu.fetch_next_register() as u16;
        let fetched_next_next_addr = cpu.fetch_next_next_register() as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, fetched_next_addr);
        assert_ne!(reg_addr, fetched_next_next_addr);
    }

    #[test]
    fn abs_specify_b1_b2_register_as_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Abs);
        cpu.insert_random_num_into_b1_b2();

        let b = cpu.fetch_next_register() as u16;
        let h = cpu.fetch_next_next_register() as u16;
        let r = ((h << 8) | b) as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn abs_x_specify_b1_b2_register_as_indexed_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::AbsX);
        cpu.insert_random_num_into_b1_b2();

        let mut b = cpu.fetch_next_register() as u16;
        let h = cpu.fetch_next_next_register() as u16;
        cpu.register.x = (rand_u8() / 2) as i8;
        b += cpu.register.x as u16;
        let r = ((h << 8) | b) as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn abs_y_specify_b1_b2_register_as_indexed_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::AbsY);
        cpu.insert_random_num_into_b1_b2();

        let mut b = cpu.fetch_next_register() as u16;
        let h = cpu.fetch_next_next_register() as u16;
        cpu.register.y = (rand_u8() / 2) as i8;
        b += cpu.register.y as u16;
        let r = ((h << 8) | b) as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn zp_specify_zero_page_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Zp);
        cpu.insert_random_num_into_b1_b2();

        let r = cpu.fetch_next_register() as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn rel_specify_relation_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Rel);
        cpu.insert_random_num_into_b1_b2();

        let b = cpu.register.pc + 2;
        let h = cpu.fetch_next_register() as u16;
        let r = (b + h) as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn ind_x_specify_indexed_indirect_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::IndX);
        cpu.insert_random_num_into_b1_b2();
        cpu.register.x = (rand_u8()) as i8;

        let l = cpu.fetch_next_register();
        let r = cpu.register.x as u8;
        let t = ex_plus_ignoring_overflow(l, r) as u16;
        cpu.map.set(t, rand_u8());
        cpu.map.set(t + 1, rand_u8());

        let b = cpu.map.addr(t) as u16;
        let h = cpu.map.addr(t + 1) as u16;
        let r = b | (h << 8);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn ind_y_specify_indexed_indirect_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::IndY);
        cpu.insert_random_num_into_b1_b2();
        cpu.register.y = (rand_u8()) as i8;

        let t = cpu.fetch_next_register() as u16;
        cpu.map.set(t, rand_u8());
        cpu.map.set(t + 1, rand_u8());

        let b = cpu.map.addr(t) as u16;
        let h = cpu.map.addr(t + 1) as u16;
        let l = ((h << 8) | b) as u8;
        let r = cpu.register.y as u8;
        let r = ex_plus_ignoring_overflow(l, r) as u16;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }
}
