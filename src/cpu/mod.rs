pub mod operator;
pub mod register;

use crate::bus::*;
use crate::nes::*;
use crate::util::*;
use operator::*;
use register::*;

use rustc_hash::*;

#[derive(Debug, Clone)]
pub struct CPU {
    pub register: Register,
    pub operators: FxHashMap<u8, Operator>,
    pub bus: Bus,
}

impl Default for CPU {
    fn default() -> Self {
        let mut cpu = Self::new();
        cpu.prepare_operators();
        cpu
    }
}

impl CPU {
    pub fn new() -> Self {
        let register = Register::default();
        let operators = FxHashMap::default();
        let bus = Bus::default();

        Self {
            register,
            operators,
            bus,
        }
    }

    fn prepare_operators(&mut self) {
        let mut operators = FxHashMap::default();
        let cycles = [
            7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, // 0x00
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, // 0x10
            6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, // 0x20
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, // 0x30
            6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, // 0x40
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, // 0x50
            6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, // 0x60
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7, // 0x70
            2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, // 0x80
            2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5, // 0x90
            2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, // 0xA0
            2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, // 0xB0
            2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, // 0xC0
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, // 0xD0
            2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, // 0xE0
            2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
            // 0xF0
        ];

        macro_rules! ope_reserved {
            ( $($id:expr => ($ope_kind:path, $addr_mode:path)),+ ) => {
                $(
                    operators.insert(
                        $id,
                        Operator {
                            ope_kind: $ope_kind,
                            addr_mode: $addr_mode,
                            cycle: cycles[$id]
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
                0x0000..=0x3FFF => self.bus.set((i + 0x8000) as u16, *n),
                0x4000..=0x8000 => self.bus.set((i + 0x8000) as u16, *n),
                _ => unreachable!(),
            }
        }
    }

    pub fn interrupt(&mut self, intr: Interrupt) {
        match intr {
            Interrupt::Nmi => (),
            Interrupt::Reset => self.reset(),
            Interrupt::Irq => (),
            Interrupt::Brk => (),
        }
    }

    pub fn reset(&mut self) {
        self.register.x = self.bus.addr(0xFFFC);
        self.register.y = self.bus.addr(0xFFFD);
        self.register.set_pc();
    }

    pub fn ex_plus(&mut self, l: u8, r: u8) -> u8 {
        if l.checked_add(r).is_none() {
            self.set_overflow(true);
            l + r - u8::MAX
        } else {
            self.set_overflow(false);
            l + r
        }
    }

    pub fn ex_minus(&mut self, l: u8, r: u8) -> u8 {
        if l > r {
            self.set_negative(false);
            l - r
        } else {
            self.set_negative(true);
            r - l
        }
    }

    pub fn ex_i8_plus(&mut self, l: u8, r: u8) -> u8 {
        if l & 0x80 == 0x80 {
            l - r + 0xf0
        } else {
            l + r
        }
    }

    pub fn set_break_mode(&mut self, b: bool) {
        self.register.p.break_mode = b;
    }

    pub fn get_break_mode(&mut self) -> bool {
        self.register.p.break_mode
    }

    pub fn set_interrupt(&mut self, b: bool) {
        self.register.p.interrupt = b;
    }

    pub fn get_interrupt(&mut self) -> bool {
        self.register.p.interrupt
    }

    pub fn set_negative(&mut self, b: bool) {
        self.register.p.negative = b;
    }

    pub fn get_negative(&mut self) -> bool {
        self.register.p.negative
    }

    pub fn set_overflow(&mut self, b: bool) {
        self.register.p.overflow = b;
    }

    pub fn get_overflow(&mut self) -> bool {
        self.register.p.overflow
    }

    pub fn set_zero(&mut self, b: bool) {
        self.register.p.zero = b;
    }

    pub fn get_zero(&mut self) -> bool {
        self.register.p.zero
    }

    pub fn set_nz(&mut self, n: u8) {
        self.set_negative(n >= 0x80);
        self.set_zero(n == 0);
    }

    pub fn set_carry(&mut self, b: bool) {
        self.register.p.carry = b;
    }

    pub fn get_carry(&mut self) -> bool {
        self.register.p.carry
    }

    pub fn set_x(&mut self, n: u8) {
        self.register.x = n;
    }

    pub fn set_y(&mut self, n: u8) {
        self.register.y = n;
    }

    pub fn set_a(&mut self, n: u8) {
        self.register.a = n;
    }

    pub fn fetch_code(&mut self) -> u8 {
        let pc = self.register.pc;
        self.bus.addr(pc)
    }

    pub fn fetch_register(&mut self) -> u8 {
        let pc = self.register.pc;
        self.bus.addr(pc)
    }

    pub fn fetch_lh_register(&mut self) -> (u8, u8) {
        let l = self.fetch_register();
        let h = self.fetch_next_register();
        (l, h)
    }

    pub fn fetch_next_register(&mut self) -> u8 {
        let pc = self.register.pc + 1;
        self.bus.addr(pc)
    }

    pub fn fetch_next_next_register(&mut self) -> u8 {
        let pc = self.register.pc + 2;
        self.bus.addr(pc)
    }

    pub fn undef(&mut self) {
        self.register.pc += 1;
    }

    pub fn nop(&mut self) {
        self.register.pc += 1;
    }

    pub fn push_stack(&mut self, n: u8) {
        let l = self.register.s as u16;
        self.register.s -= 1;
        let r = l + (1 << 8);
        self.bus.set(r, n);
    }

    pub fn pull_stack(&mut self) -> u8 {
        let l = self.register.s + 1;
        self.register.s += 1;
        let h = 0x100;
        let r = l + h;
        self.bus.addr(r)
    }

    pub fn ex_addr_mode(&mut self, addr_mode: &AddrMode) -> u16 {
        self.register.pc += 1;
        let r = match addr_mode {
            AddrMode::Impl => 0,
            AddrMode::Acc => {
                let l = self.register.a as u16;
                l as u16
            }
            AddrMode::Imm => {
                let l = self.fetch_register();
                l as u16
            }
            AddrMode::Zp => {
                let l = self.fetch_register() as i16;
                l as u16
            }
            AddrMode::ZpX => {
                let mut l = self.fetch_register() as u16;
                l += self.register.x as u16;
                l as u16
            }
            AddrMode::ZpY => {
                let mut l = self.fetch_register();
                l += self.register.y;
                l as u16
            }
            AddrMode::Abs => {
                let (l, h) = self.fetch_lh_register();
                combine_high_low(l, h)
            }
            AddrMode::AbsX => {
                let (l, h) = self.fetch_lh_register();
                let x = self.register.x as u16;
                (combine_high_low(l, h) + x) as u16
            }
            AddrMode::AbsY => {
                let (l, h) = self.fetch_lh_register();
                let y = self.register.y as u16;
                (combine_high_low(l, h) + y) as u16
            }
            AddrMode::Rel => {
                let l = self.register.pc + 1;
                let h = self.fetch_register() as u16;
                if h & 0x80 == 0x80 {
                    (l - h + 0xf0) as u16
                } else {
                    (l + h) as u16
                }
            }
            AddrMode::IndX => {
                let l = self.fetch_register();
                let r = self.register.x;
                let t = combine_high_low(l, r);
                let (l, h) = self.bus.cpu_bus.lh_addr(t);
                combine_high_low(l, h)
            }
            AddrMode::IndY => {
                let t = self.fetch_register() as u16;
                let (l, h) = self.bus.cpu_bus.lh_addr(t);
                let y = self.register.y as u16;
                combine_high_low(l, h) + y
            }
            AddrMode::Ind => {
                let (l, h) = self.fetch_lh_register();
                let t = combine_high_low(l, h);
                let (l, h) = self.bus.cpu_bus.lh_addr(t);
                combine_high_low(l, h)
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

    pub fn get_addr_for_mixed_imm_mode(&mut self, r: u16, addr_mode: AddrMode) -> u16 {
        let imm = matches!(addr_mode, AddrMode::Imm);
        if imm {
            r
        } else {
            self.bus.addr(r) as u16
        }
    }

    pub fn run_ope(&mut self, r: u16, opekind: OpeKind, addr_mode: AddrMode) {
        match opekind {
            OpeKind::Adc => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode);
                if self.register.a.checked_add(r as u8).is_some() {
                    self.register.a += r as u8;
                    self.set_nz(self.register.a);
                    self.set_carry(false);
                } else {
                    let s = (self.register.a as u16 + (r as u16)) - u8::MAX as u16;
                    self.register.a = s as u8;
                    self.set_carry(true);
                }
            }
            OpeKind::Sbc => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                if self.register.a.checked_sub(r).is_some() {
                    self.register.a -= r;
                    self.set_nz(self.register.a);
                    self.set_carry(true);
                } else {
                    let s = r - self.register.a;
                    self.register.a = s;
                    self.set_carry(false);
                    self.register.p.carry = false;
                }
            }
            OpeKind::And => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a &= r;
            }
            OpeKind::Ora => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a |= r;
            }
            OpeKind::Eor => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a ^= r;
            }
            OpeKind::Bcc => {
                if !self.get_carry() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bcs => {
                if self.get_carry() {
                    self.register.pc = r;
                }
            }
            OpeKind::Beq => {
                if self.get_zero() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bne => {
                if !self.get_zero() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bvc => {
                if !self.get_overflow() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bvs => {
                if self.get_overflow() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bpl => {
                if !self.get_negative() {
                    self.register.pc = r;
                }
            }
            OpeKind::Bmi => {
                if self.get_negative() {
                    self.register.pc = r;
                }
            }
            OpeKind::Cmp => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode);
                let s: i16 = (self.register.a as i16) - (r as i16);
                self.set_nz(s as u8);
                self.set_carry(s > 0);
            }
            OpeKind::Cpx => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                let v = r;
                let s = self.register.x > v;
                self.set_nz(s as u8);
                self.set_carry(s);
            }
            OpeKind::Cpy => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                let v = r;
                let s = self.register.y > v;
                self.set_nz(s as u8);
                self.set_carry(s);
            }
            OpeKind::Inc => {
                let v = self.bus.addr(r);
                let s = self.ex_plus(v, 1);
                self.bus.set(r, s);
                self.set_nz(self.bus.addr(r));
            }
            OpeKind::Dec => {
                let v = self.bus.addr(r);
                let s = self.ex_minus(v, 1);
                self.bus.set(r, s);
                self.set_nz(self.bus.addr(r));
            }
            OpeKind::Inx => {
                let x = self.register.x + 1;
                self.set_x(x);
                self.set_nz(x);
            }
            OpeKind::Dex => {
                let x = self.register.x as i8 - 1;
                self.set_x(x as u8);
                self.set_nz(x as u8);
            }
            OpeKind::Iny => {
                let y = self.register.y + 1;
                self.set_y(y);
                self.set_nz(y);
            }
            OpeKind::Dey => {
                let y = self.register.y as i8 - 1;
                self.set_y(y as u8);
                self.set_nz(y as u8);
            }
            OpeKind::Clc => self.register.p.carry = false,
            OpeKind::Sec => self.register.p.carry = true,
            OpeKind::Cli => self.register.p.interrupt = false,
            OpeKind::Sei => self.register.p.interrupt = true,
            OpeKind::Cld => self.register.p.decimal = false,
            OpeKind::Sed => self.register.p.decimal = true,
            OpeKind::Clv => self.register.p.overflow = false,
            OpeKind::Lda => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a = r;
                self.set_nz(self.register.a);
            }
            OpeKind::Ldx => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.x = r;
                self.set_nz(self.register.x);
            }
            OpeKind::Ldy => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.y = r;
                self.set_nz(self.register.y);
            }
            OpeKind::Sta => {
                self.bus.set(r, self.register.a);
            }
            OpeKind::Stx => {
                self.bus.set(r, self.register.x);
            }
            OpeKind::Sty => {
                self.bus.set(r, self.register.y);
            }
            OpeKind::Tax => {
                self.register.x = self.register.a;
                self.set_nz(self.register.x);
            }
            OpeKind::Txa => {
                self.register.a = self.register.x;
                self.set_nz(self.register.a);
            }
            OpeKind::Tay => {
                self.register.y = self.register.a;
                self.set_nz(self.register.y);
            }
            OpeKind::Tya => {
                self.register.a = self.register.y;
                self.set_nz(self.register.a);
            }
            OpeKind::Txs => {
                self.register.set_s(self.register.x);
            }
            OpeKind::Pha => {
                self.register.a = self.bus.addr(r);
                self.push_stack(self.register.a);
            }
            OpeKind::Pla => {
                self.register.a = self.pull_stack();
                self.set_nz(self.register.a);
            }
            OpeKind::Php => {
                let n = self.register.p.to_n();
                self.push_stack(n);
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
                let l = (p & 0x00FF) as u8;
                self.push_stack(h);
                self.push_stack(l);
                self.register.pc = r;
            }
            OpeKind::Rts => {
                let l = self.pull_stack();
                let h = self.pull_stack();
                let t = combine_high_low(l, h);
                self.register.pc = t;
                self.register.pc += 1;
            }
            OpeKind::Brk => {
                if !self.get_interrupt() {
                    self.register.pc -= 1;
                    let p = self.register.pc;
                    let h = ((p & 0xFF00) >> 8) as u8;
                    let l = (p & 0x00FF) as u8;
                    self.push_stack(h);
                    self.push_stack(l);
                    let n = self.register.p.to_n();
                    self.push_stack(n);
                    self.set_break_mode(true);
                    self.set_interrupt(true);
                    let (h, l) = self.bus.cpu_bus.hl_addr(0xFFFE);
                    self.register.pc = combine_high_low(l, h);
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

    pub fn ex_ope(&mut self) -> u8 {
        match self.read_ope() {
            Some(Operator {
                ope_kind,
                addr_mode,
                cycle,
            }) => {
                let ope_kind = ope_kind.clone();
                let addr_mode = addr_mode.clone();
                let cycle = cycle.clone();
                let reg_addr = self.ex_addr_mode(&addr_mode);
                self.run_ope(reg_addr, ope_kind, addr_mode);
                // println!(
                //     "self.register.pc: {:0x?}, reg_addr: {:0x?}",
                //     self.register.pc, reg_addr
                // );
                cycle
            }
            None => {
                self.undef();
                unimplemented!();
            }
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
        // print!(
        //     "self.operators.get_mut(&c): {:?} ",
        //     self.operators.get_mut(&c)
        // );
        // print!("0x2000 {}, ", self.bus.addr(0x2000));
        // print!("0x2001 {}, ", self.bus.addr(0x2001));
        // print!("0x2002 {}, ", self.bus.addr(0x2002));
        // print!("0x2003 {}, ", self.bus.addr(0x2003));
        // print!("0x2004 {}, ", self.bus.addr(0x2004));
        // print!("0x2005 {}, ", self.bus.addr(0x2005));
        // print!("0x2006 {}, ", self.bus.addr(0x2006));
        // println!("0x2007 {}, ", self.bus.addr(0x2007));

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

#[derive(Debug, Clone)]
pub enum Interrupt {
    Nmi,
    Reset,
    Irq,
    Brk,
}

#[cfg(test)]
mod test {
    use crate::cpu::*;
    extern crate rand;
    use rand::seq::IteratorRandom;

    impl CPU {
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
                    *reg_addr = self.ex_addr_mode(&addr_mode);
                }
                None => (),
            };
        }

        fn insert_random_num_into_b1_b2(&mut self) {
            self.bus.cpu_bus.prg_rom1[1] = rand_u8();
            self.bus.cpu_bus.prg_rom1[2] = rand_u8();
        }

        fn fetch_next_lh_register(&mut self) -> (u8, u8) {
            self.register.pc += 1;
            let (l, h) = self.fetch_lh_register();
            self.register.pc -= 1;
            (l, h)
        }
    }

    fn rand_u8() -> u8 {
        use crate::cpu::test::rand::Rng;

        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen();
        n
    }

    fn prepare_cpu_for_addr_mode_test(addr_mode: AddrMode) -> CPU {
        let nes = Nes::new();
        let mut cpu = CPU::default();
        cpu.init(&nes);
        cpu.interrupt(Interrupt::Reset);
        let (code, _) = cpu.random_pick_operator_with_specify_addr_mode(addr_mode);
        cpu.bus.cpu_bus.prg_rom1[0] = code;
        cpu
    }

    #[test]
    fn acc_not_specify_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Acc);
        cpu.insert_random_num_into_b1_b2();

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        let (l, h) = cpu.fetch_next_lh_register();

        assert_eq!(reg_addr, 0);
        assert_ne!(reg_addr, l as u16);
        assert_ne!(reg_addr, h as u16);
    }

    #[test]
    fn impl_not_specify_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Impl);
        cpu.insert_random_num_into_b1_b2();

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        let (l, h) = cpu.fetch_next_lh_register();

        assert_eq!(reg_addr, 0);
        assert_ne!(reg_addr, l as u16);
        assert_ne!(reg_addr, h as u16);
    }

    #[test]
    fn imm_specify_immediate_register_as_addressing_register() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Imm);
        cpu.insert_random_num_into_b1_b2();

        let (l, h) = cpu.fetch_next_lh_register();

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, l as u16);
        assert_ne!(reg_addr, h as u16);
    }

    #[test]
    fn abs_specify_b1_b2_register_as_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Abs);
        cpu.insert_random_num_into_b1_b2();

        let (l, h) = cpu.fetch_next_lh_register();
        let r = combine_high_low(l, h);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn abs_x_specify_b1_b2_register_as_indexed_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::AbsX);
        cpu.insert_random_num_into_b1_b2();

        let (l, h) = cpu.fetch_next_lh_register();
        cpu.register.x = rand_u8();
        let x = cpu.register.x as u16;
        let r = combine_high_low(l, h) + x;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn abs_y_specify_b1_b2_register_as_indexed_absolute_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::AbsY);
        cpu.insert_random_num_into_b1_b2();

        let (l, h) = cpu.fetch_next_lh_register();
        cpu.register.y = rand_u8();
        let y = cpu.register.y as u16;
        let r = combine_high_low(l, h) + y;

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

        let l = cpu.register.pc + 2;
        let h = cpu.fetch_next_register() as u16;
        let r = if h & 0x80 == 0x80 {
            (l - h + 0xf0) as u16
        } else {
            (l + h) as u16
        };

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn ind_x_specify_indexed_indirect_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::IndX);
        cpu.insert_random_num_into_b1_b2();
        cpu.register.x = rand_u8();

        let l = cpu.fetch_next_register();
        let h = cpu.register.x;
        let t = combine_high_low(l, h);
        cpu.bus.set(t, rand_u8());
        cpu.bus.set(t + 1, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(t);
        let r = combine_high_low(l, h);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn ind_y_specify_indexed_indirect_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::IndY);
        cpu.insert_random_num_into_b1_b2();
        cpu.register.y = rand_u8();

        let t = cpu.fetch_next_register() as u16;
        cpu.bus.set(t, rand_u8());
        cpu.bus.set(t + 1, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(t);
        let y = cpu.register.y as u16;
        let r = combine_high_low(l, h) + y;

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }

    #[test]
    fn ind_specify_indexed_indirect_address() {
        let mut cpu = prepare_cpu_for_addr_mode_test(AddrMode::Ind);
        cpu.insert_random_num_into_b1_b2();

        let (l, h) = cpu.fetch_next_lh_register();
        let t = combine_high_low(l, h);

        cpu.bus.set(t, rand_u8());
        cpu.bus.set(t + 1, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(t);
        let r = combine_high_low(l, h);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }
}
