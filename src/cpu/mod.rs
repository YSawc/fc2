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
    pub cycle: u16,
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
            cycle: 0,
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
            0x02 => (OpeKind::Kil, AddrMode::Nop),
            0x03 => (OpeKind::Slo, AddrMode::ZpX),
            0x04 => (OpeKind::Dop, AddrMode::Zp),
            0x05 => (OpeKind::Ora, AddrMode::Zp),
            0x06 => (OpeKind::Asl, AddrMode::Zp),
            0x07 => (OpeKind::Slo, AddrMode::ZpX),
            0x08 => (OpeKind::Php, AddrMode::Impl),
            0x09 => (OpeKind::Ora, AddrMode::Imm),
            0x0A => (OpeKind::Asl, AddrMode::Acc),
            0x0B => (OpeKind::Aac, AddrMode::Imm),
            0x0C => (OpeKind::Top, AddrMode::Abs),
            0x0D => (OpeKind::Ora, AddrMode::Abs),
            0x0E => (OpeKind::Asl, AddrMode::Abs),
            0x0F => (OpeKind::Slo, AddrMode::Abs),

            0x10 => (OpeKind::Bpl, AddrMode::Rel),
            0x11 => (OpeKind::Ora, AddrMode::IndY),
            0x12 => (OpeKind::Kil, AddrMode::Nop),
            0x13 => (OpeKind::Slo, AddrMode::ZpY),
            0x14 => (OpeKind::Dop, AddrMode::ZpX),
            0x15 => (OpeKind::Ora, AddrMode::ZpX),
            0x16 => (OpeKind::Asl, AddrMode::ZpX),
            0x17 => (OpeKind::Slo, AddrMode::ZpX),
            0x18 => (OpeKind::Clc, AddrMode::Impl),
            0x19 => (OpeKind::Ora, AddrMode::AbsY),
            0x1A => (OpeKind::Nop, AddrMode::Nop),
            0x1B => (OpeKind::Slo, AddrMode::AbsY),
            0x1C => (OpeKind::Top, AddrMode::AbsX),
            0x1D => (OpeKind::Ora, AddrMode::AbsX),
            0x1E => (OpeKind::Asl, AddrMode::AbsX),
            0x1F => (OpeKind::Slo, AddrMode::AbsX),

            0x20 => (OpeKind::Jsr, AddrMode::Abs),
            0x21 => (OpeKind::And, AddrMode::IndX),
            0x22 => (OpeKind::Kil, AddrMode::Nop),
            0x23 => (OpeKind::Rla, AddrMode::ZpX),
            0x24 => (OpeKind::Bit, AddrMode::Zp),
            0x25 => (OpeKind::And, AddrMode::Zp),
            0x26 => (OpeKind::Rol, AddrMode::Zp),
            0x27 => (OpeKind::Nop, AddrMode::Nop),
            0x28 => (OpeKind::Plp, AddrMode::Impl),
            0x29 => (OpeKind::And, AddrMode::Imm),
            0x2A => (OpeKind::Rol, AddrMode::Acc),
            0x2B => (OpeKind::Aac, AddrMode::Imm),
            0x2C => (OpeKind::Bit, AddrMode::Abs),
            0x2D => (OpeKind::And, AddrMode::Abs),
            0x2E => (OpeKind::Rol, AddrMode::Abs),
            0x2F => (OpeKind::Rla, AddrMode::Abs),

            0x30 => (OpeKind::Bmi, AddrMode::Rel),
            0x31 => (OpeKind::And, AddrMode::IndY),
            0x32 => (OpeKind::Kil, AddrMode::Nop),
            0x33 => (OpeKind::Rla, AddrMode::Nop),
            0x34 => (OpeKind::Dop, AddrMode::ZpX),
            0x35 => (OpeKind::And, AddrMode::ZpX),
            0x36 => (OpeKind::Rol, AddrMode::ZpX),
            0x37 => (OpeKind::Nop, AddrMode::Nop),
            0x38 => (OpeKind::Sec, AddrMode::Impl),
            0x39 => (OpeKind::And, AddrMode::AbsY),
            0x3A => (OpeKind::Nop, AddrMode::Nop),
            0x3B => (OpeKind::Rla, AddrMode::AbsY),
            0x3C => (OpeKind::Top, AddrMode::AbsX),
            0x3D => (OpeKind::And, AddrMode::AbsX),
            0x3E => (OpeKind::Rol, AddrMode::AbsX),
            0x3F => (OpeKind::Rla, AddrMode::AbsX),

            0x40 => (OpeKind::Rti, AddrMode::Impl),
            0x41 => (OpeKind::Eor, AddrMode::IndX),
            0x42 => (OpeKind::Kil, AddrMode::Nop),
            0x43 => (OpeKind::Sre, AddrMode::ZpY),
            0x44 => (OpeKind::Dcp, AddrMode::Zp),
            0x45 => (OpeKind::Eor, AddrMode::Zp),
            0x46 => (OpeKind::Lsr, AddrMode::Zp),
            0x47 => (OpeKind::Nop, AddrMode::Nop),
            0x48 => (OpeKind::Pha, AddrMode::Impl),
            0x49 => (OpeKind::Eor, AddrMode::Imm),
            0x4A => (OpeKind::Lsr, AddrMode::Acc),
            0x4B => (OpeKind::Asr, AddrMode::Imm),
            0x4C => (OpeKind::Jmp, AddrMode::Abs),
            0x4D => (OpeKind::Eor, AddrMode::Abs),
            0x4E => (OpeKind::Lsr, AddrMode::Abs),
            0x4F => (OpeKind::Sre, AddrMode::Abs),

            0x50 => (OpeKind::Bvc, AddrMode::Rel),
            0x51 => (OpeKind::Eor, AddrMode::IndY),
            0x52 => (OpeKind::Kil, AddrMode::Nop),
            0x53 => (OpeKind::Sre, AddrMode::ZpY),
            0x54 => (OpeKind::Dop, AddrMode::ZpX),
            0x55 => (OpeKind::Eor, AddrMode::ZpX),
            0x56 => (OpeKind::Lsr, AddrMode::ZpX),
            0x57 => (OpeKind::Nop, AddrMode::Nop),
            0x58 => (OpeKind::Cli, AddrMode::Impl),
            0x59 => (OpeKind::Eor, AddrMode::AbsY),
            0x5A => (OpeKind::Nop, AddrMode::Nop),
            0x5B => (OpeKind::Sre, AddrMode::AbsY),
            0x5C => (OpeKind::Nop, AddrMode::AbsX),
            0x5D => (OpeKind::Eor, AddrMode::AbsX),
            0x5E => (OpeKind::Lsr, AddrMode::AbsX),
            0x5F => (OpeKind::Sre, AddrMode::AbsX),

            0x60 => (OpeKind::Rts, AddrMode::Impl),
            0x61 => (OpeKind::Adc, AddrMode::IndX),
            0x62 => (OpeKind::Kil, AddrMode::Nop),
            0x63 => (OpeKind::Rra, AddrMode::ZpX),
            0x64 => (OpeKind::Dop, AddrMode::Zp),
            0x65 => (OpeKind::Adc, AddrMode::Zp),
            0x66 => (OpeKind::Ror, AddrMode::Zp),
            0x67 => (OpeKind::Nop, AddrMode::Nop),
            0x68 => (OpeKind::Pla, AddrMode::Impl),
            0x69 => (OpeKind::Adc, AddrMode::Imm),
            0x6A => (OpeKind::Ror, AddrMode::Acc),
            0x6B => (OpeKind::Arr, AddrMode::Imm),
            0x6C => (OpeKind::Jmp, AddrMode::Ind),
            0x6D => (OpeKind::Adc, AddrMode::Abs),
            0x6E => (OpeKind::Ror, AddrMode::Abs),
            0x6F => (OpeKind::Rra, AddrMode::Abs),

            0x70 => (OpeKind::Bvs, AddrMode::Rel),
            0x71 => (OpeKind::Adc, AddrMode::IndY),
            0x72 => (OpeKind::Kil, AddrMode::Nop),
            0x73 => (OpeKind::Rra, AddrMode::ZpX),
            0x74 => (OpeKind::Dop, AddrMode::ZpX),
            0x75 => (OpeKind::Adc, AddrMode::ZpX),
            0x76 => (OpeKind::Ror, AddrMode::ZpX),
            0x77 => (OpeKind::Nop, AddrMode::Nop),
            0x78 => (OpeKind::Sei, AddrMode::Impl),
            0x79 => (OpeKind::Adc, AddrMode::AbsY),
            0x7A => (OpeKind::Nop, AddrMode::Nop),
            0x7B => (OpeKind::Rra, AddrMode::AbsY),
            0x7C => (OpeKind::Top, AddrMode::AbsX),
            0x7D => (OpeKind::Adc, AddrMode::AbsX),
            0x7E => (OpeKind::Ror, AddrMode::AbsX),
            0x7F => (OpeKind::Rra, AddrMode::AbsX),

            0x80 => (OpeKind::Dop, AddrMode::Imm),
            0x81 => (OpeKind::Sta, AddrMode::IndX),
            0x82 => (OpeKind::Dop, AddrMode::Imm),
            0x83 => (OpeKind::Aax, AddrMode::ZpX),
            0x84 => (OpeKind::Sty, AddrMode::Zp),
            0x85 => (OpeKind::Sta, AddrMode::Zp),
            0x86 => (OpeKind::Stx, AddrMode::Zp),
            0x87 => (OpeKind::Nop, AddrMode::Nop),
            0x88 => (OpeKind::Dey, AddrMode::Impl),
            0x89 => (OpeKind::Dop, AddrMode::Imm),
            0x8A => (OpeKind::Txa, AddrMode::Impl),
            0x8B => (OpeKind::Xaa, AddrMode::Imm),
            0x8C => (OpeKind::Sty, AddrMode::Abs),
            0x8D => (OpeKind::Sta, AddrMode::Abs),
            0x8E => (OpeKind::Stx, AddrMode::Abs),
            0x8F => (OpeKind::Aax, AddrMode::Abs),

            0x90 => (OpeKind::Bcc, AddrMode::Rel),
            0x91 => (OpeKind::Sta, AddrMode::IndY),
            0x92 => (OpeKind::Kil, AddrMode::Nop),
            0x93 => (OpeKind::Axa, AddrMode::ZpY),
            0x94 => (OpeKind::Sty, AddrMode::ZpX),
            0x95 => (OpeKind::Sta, AddrMode::ZpX),
            0x96 => (OpeKind::Stx, AddrMode::ZpY),
            0x97 => (OpeKind::Nop, AddrMode::Nop),
            0x98 => (OpeKind::Tya, AddrMode::Impl),
            0x99 => (OpeKind::Sta, AddrMode::AbsY),
            0x9A => (OpeKind::Txs, AddrMode::Impl),
            0x9B => (OpeKind::Xas, AddrMode::AbsY),
            0x9C => (OpeKind::Sya, AddrMode::AbsX),
            0x9D => (OpeKind::Sta, AddrMode::AbsX),
            0x9E => (OpeKind::Sxa, AddrMode::AbsX),
            0x9F => (OpeKind::Axa, AddrMode::AbsY),

            0xA0 => (OpeKind::Ldy, AddrMode::Imm),
            0xA1 => (OpeKind::Lda, AddrMode::IndX),
            0xA2 => (OpeKind::Ldx, AddrMode::Imm),
            0xA3 => (OpeKind::Lax, AddrMode::ZpX),
            0xA4 => (OpeKind::Ldy, AddrMode::Zp),
            0xA5 => (OpeKind::Lda, AddrMode::Zp),
            0xA6 => (OpeKind::Ldx, AddrMode::Zp),
            0xA7 => (OpeKind::Nop, AddrMode::Nop),
            0xA8 => (OpeKind::Tay, AddrMode::Impl),
            0xA9 => (OpeKind::Lda, AddrMode::Imm),
            0xAA => (OpeKind::Tax, AddrMode::Impl),
            0xAB => (OpeKind::Atx, AddrMode::Imm),
            0xAC => (OpeKind::Ldy, AddrMode::Abs),
            0xAD => (OpeKind::Lda, AddrMode::Abs),
            0xAE => (OpeKind::Ldx, AddrMode::Abs),
            0xAF => (OpeKind::Lax, AddrMode::Abs),

            0xB0 => (OpeKind::Bcs, AddrMode::Rel),
            0xB1 => (OpeKind::Lda, AddrMode::IndY),
            0xB2 => (OpeKind::Kil, AddrMode::Nop),
            0xB3 => (OpeKind::Lax, AddrMode::ZpY),
            0xB4 => (OpeKind::Ldy, AddrMode::ZpX),
            0xB5 => (OpeKind::Lda, AddrMode::ZpX),
            0xB6 => (OpeKind::Ldx, AddrMode::ZpY),
            0xB7 => (OpeKind::Nop, AddrMode::Nop),
            0xB8 => (OpeKind::Clv, AddrMode::Impl),
            0xB9 => (OpeKind::Lda, AddrMode::AbsY),
            0xBA => (OpeKind::Tsx, AddrMode::Impl),
            0xBB => (OpeKind::Lar, AddrMode::AbsY),
            0xBC => (OpeKind::Ldy, AddrMode::AbsX),
            0xBD => (OpeKind::Lda, AddrMode::AbsX),
            0xBE => (OpeKind::Ldx, AddrMode::AbsY),
            0xBF => (OpeKind::Lax, AddrMode::AbsY),

            0xC0 => (OpeKind::Cpy, AddrMode::Imm),
            0xC1 => (OpeKind::Cmp, AddrMode::IndX),
            0xC2 => (OpeKind::Dop, AddrMode::Imm),
            0xC3 => (OpeKind::Nop, AddrMode::Nop),
            0xC4 => (OpeKind::Cpy, AddrMode::Zp),
            0xC5 => (OpeKind::Cmp, AddrMode::Zp),
            0xC6 => (OpeKind::Dec, AddrMode::Zp),
            0xC7 => (OpeKind::Nop, AddrMode::Nop),
            0xC8 => (OpeKind::Iny, AddrMode::Impl),
            0xC9 => (OpeKind::Cmp, AddrMode::Imm),
            0xCA => (OpeKind::Dex, AddrMode::Impl),
            0xCB => (OpeKind::Axs, AddrMode::Imm),
            0xCC => (OpeKind::Cpy, AddrMode::Abs),
            0xCD => (OpeKind::Cmp, AddrMode::Abs),
            0xCE => (OpeKind::Dec, AddrMode::Abs),
            0xCF => (OpeKind::Dcp, AddrMode::Abs),

            0xD0 => (OpeKind::Bne, AddrMode::Rel),
            0xD1 => (OpeKind::Cmp, AddrMode::IndY),
            0xD2 => (OpeKind::Kil, AddrMode::Nop),
            0xD3 => (OpeKind::Dcp, AddrMode::ZpY),
            0xD4 => (OpeKind::Dop, AddrMode::ZpX),
            0xD5 => (OpeKind::Cmp, AddrMode::ZpX),
            0xD6 => (OpeKind::Dec, AddrMode::ZpX),
            0xD7 => (OpeKind::Nop, AddrMode::Nop),
            0xD8 => (OpeKind::Cld, AddrMode::Impl),
            0xD9 => (OpeKind::Cmp, AddrMode::AbsY),
            0xDA => (OpeKind::Nop, AddrMode::Nop),
            0xDB => (OpeKind::Dcp, AddrMode::AbsY),
            0xDC => (OpeKind::Top, AddrMode::AbsX),
            0xDD => (OpeKind::Cmp, AddrMode::AbsX),
            0xDE => (OpeKind::Dec, AddrMode::AbsX),
            0xDF => (OpeKind::Dcp, AddrMode::AbsX),

            0xE0 => (OpeKind::Cpx, AddrMode::Imm),
            0xE1 => (OpeKind::Sbc, AddrMode::IndX),
            0xE2 => (OpeKind::Dop, AddrMode::Imm),
            0xE3 => (OpeKind::Isc, AddrMode::ZpX),
            0xE4 => (OpeKind::Cpx, AddrMode::Zp),
            0xE5 => (OpeKind::Sbc, AddrMode::Zp),
            0xE6 => (OpeKind::Inc, AddrMode::Zp),
            0xE7 => (OpeKind::Nop, AddrMode::Nop),
            0xE8 => (OpeKind::Inx, AddrMode::Impl),
            0xE9 => (OpeKind::Sbc, AddrMode::Imm),
            0xEA => (OpeKind::Nop, AddrMode::Impl),
            0xEB => (OpeKind::Sbc, AddrMode::Imm),
            0xEC => (OpeKind::Cpx, AddrMode::Abs),
            0xED => (OpeKind::Sbc, AddrMode::Abs),
            0xEE => (OpeKind::Inc, AddrMode::Abs),
            0xEF => (OpeKind::Isc, AddrMode::Abs),

            0xF0 => (OpeKind::Beq, AddrMode::Rel),
            0xF1 => (OpeKind::Sbc, AddrMode::IndY),
            0xF2 => (OpeKind::Kil, AddrMode::Nop),
            0xF3 => (OpeKind::Isc, AddrMode::ZpY),
            0xF4 => (OpeKind::Dop, AddrMode::ZpX),
            0xF5 => (OpeKind::Sbc, AddrMode::ZpX),
            0xF6 => (OpeKind::Inc, AddrMode::ZpX),
            0xF7 => (OpeKind::Nop, AddrMode::Nop),
            0xF8 => (OpeKind::Sed, AddrMode::Impl),
            0xF9 => (OpeKind::Sbc, AddrMode::AbsY),
            0xFA => (OpeKind::Nop, AddrMode::Nop),
            0xFB => (OpeKind::Isc, AddrMode::AbsY),
            0xFC => (OpeKind::Top, AddrMode::AbsX),
            0xFD => (OpeKind::Sbc, AddrMode::AbsX),
            0xFE => (OpeKind::Inc, AddrMode::AbsX),
            0xFF => (OpeKind::Isc, AddrMode::AbsX)
        };
        self.operators = operators;
    }

    pub fn init(&mut self, nes: &Nes) {
        let prgs = &nes.header.info.prg_rom;
        match prgs.len() {
            0x8000 => {
                for (i, n) in prgs.iter().enumerate() {
                    match i {
                        0x0000..=0x3FFF => self.bus.set((i + 0x8000) as u16, *n),
                        0x4000..=0x8000 => self.bus.set((i + 0x8000) as u16, *n),
                        _ => unreachable!(),
                    }
                }
            }
            0x4000 => {
                for (i, n) in prgs.iter().enumerate() {
                    match i {
                        0x0000..=0x3FFF => {
                            self.bus.set((i + 0x8000) as u16, *n);
                            self.bus.set((i + 0xC000) as u16, *n);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unimplemented!(
                "prg_rom lengh {:0x?} detected, but not implemented!",
                prgs.len()
            ),
        }
        // println!("{:0x?}", prgs);
        // unimplemented!("!!!");
    }

    pub fn interrupt(&mut self, intr: Interrupt) {
        match intr {
            Interrupt::Nmi => {
                self.register.p.break_mode = false;
                self.register.p.interrupt = true;
                self.push_pc();
                let p = self.register.p.to_n();
                self.push_stack(p);
                let (l, h) = self.bus.cpu_bus.lh_addr(0xFFFA);
                self.register.pc = combine_high_low(l, h);
            }
            Interrupt::Reset => self.reset(),
            Interrupt::Irq => unimplemented!(),
            Interrupt::Brk => unimplemented!(),
        }
    }

    // MEMO: use for nestest.nes
    // pub fn reset(&mut self) {
    //     self.register.pc = 0 + (0xc0 << 8);
    // }

    pub fn reset(&mut self) {
        self.register.x = self.bus.addr(0xFFFC);
        self.register.y = self.bus.addr(0xFFFD);
        self.register.set_pc();
    }

    pub fn ex_plus(&mut self, l: u8, r: u8) -> u8 {
        if l.checked_add(r).is_none() {
            self.set_overflow(true);
            ((l as u16 + r as u16 - 1) - (u8::MAX as u16)) as u8
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
        if l < 0x80 {
            l + r
        } else {
            (l as i16 + r as i16) as u8
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
        let pc = self.register.pc.wrapping_add(1);
        self.bus.addr(pc)
    }

    pub fn fetch_next_next_register(&mut self) -> u8 {
        let pc = self.register.pc.wrapping_add(2);
        self.bus.addr(pc)
    }

    pub fn undef(&mut self) {
        self.register.pc += self.register.pc.wrapping_add(1);
    }

    pub fn nop(&mut self) {}

    pub fn push_stack(&mut self, n: u8) {
        let l = self.register.s as u16;
        let s = self.register.s.wrapping_sub(1);
        self.register.s = s;
        let r = l.wrapping_add(1 << 8);
        self.bus_set(r, n);
    }

    pub fn pull_stack(&mut self) -> u8 {
        let l = self.register.s.wrapping_add(1);
        self.register.s = l;
        let h = 0x100;
        let r = l.wrapping_add(h);
        self.bus.addr(r)
    }

    fn set_oam(&mut self) {
        self.cycle += 513;
        if self.cycle % 2 != 0 {
            self.cycle += 1;
        }
        let mut sprite_infos = vec![];

        let r = self.bus.addr(0x4014) as u16;
        for n in 0..0xff {
            let data = self.bus.addr((r << 8) as u16 | n as u16);
            sprite_infos.push(data);
        }
        // print!("set_oam(sprite_infos: {:0x?} r: {:0x?}), ", sprite_infos, r);
        self.bus.ppu.primary_oam.set_sprite_infos(sprite_infos);
    }

    fn bus_set(&mut self, n: u16, r: u8) {
        self.bus.set(n, r);
        match n {
            0x4014 => {
                self.set_oam();
            }
            _ => (),
        }
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
                let l = self.fetch_register() as u16;
                l
            }
            AddrMode::ZpX => {
                let l = self.fetch_register();
                let l = l.wrapping_add(self.register.x);
                l as u16
            }
            AddrMode::ZpY => {
                let l = self.fetch_register();
                let l = l.wrapping_add(self.register.y);
                l as u16
            }
            AddrMode::Abs => {
                let (l, h) = self.fetch_lh_register();
                combine_high_low(l, h)
            }
            AddrMode::AbsX => {
                let (l, h) = self.fetch_lh_register();
                let x = self.register.x as u16;
                let m = combine_high_low(l, h);
                let (m, c) = m.overflowing_add(x);
                if c {
                    self.register.p.carry = c;
                }
                m
            }
            AddrMode::AbsY => {
                let (l, h) = self.fetch_lh_register();
                let y = self.register.y as u16;
                let m = combine_high_low(l, h);
                let (m, c) = m.overflowing_add(y);
                if c {
                    self.register.p.carry = c;
                }
                m
            }
            AddrMode::Rel => {
                let l = self.register.pc + 1;
                let h = self.fetch_register() as u16;
                if h < 0x80 {
                    (l + h) as u16
                } else {
                    (l + h - 256) as u16
                }
            }
            AddrMode::IndX => {
                let l = self.fetch_register();
                let r = self.register.x;
                let t = l.wrapping_add(r);
                let (l, h) = self.bus.cpu_bus.lh_zeropage_addr(t);
                combine_high_low(l, h)
            }
            AddrMode::IndY => {
                let t = self.fetch_register();
                let (l, h) = self.bus.cpu_bus.lh_zeropage_addr(t);
                let r = self.register.y as u16;
                let l = combine_high_low(l as u8, h as u8);
                let t = l.wrapping_add(r);
                t
            }
            AddrMode::Ind => {
                let (l, h) = self.fetch_lh_register();
                let t = combine_high_low(l, h);
                let (l, h) = self.bus.cpu_bus.lh_ignore_overflowing_addr(t);
                combine_high_low(l, h)
            }
            AddrMode::Nop => 0,
        };

        match addr_mode {
            AddrMode::Acc | AddrMode::Impl | AddrMode::Nop => (),
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

    pub fn is_branch_enable(&self) -> bool {
        if self.register.p.interrupt {
            false
        } else {
            true
        }
    }

    pub fn get_addr_for_mixed_imm_mode(&mut self, r: u16, addr_mode: AddrMode) -> u16 {
        let imm = matches!(addr_mode, AddrMode::Imm);
        if imm {
            r
        } else {
            self.bus.addr(r) as u16
        }
    }

    pub fn push_pc(&mut self) {
        let p = self.register.pc;
        let h = ((p & 0xFF00) >> 8) as u8;
        let l = (p & 0x00FF) as u8;
        self.push_stack(h);
        self.push_stack(l);
    }

    pub fn sign_plus(&mut self, l: u8, r: u8) -> u8 {
        let is_l_plus = (l & 0b10000000) == 0;
        let is_r_plus = (r & 0b10000000) == 0;

        let (n, c) = l.overflowing_add(r + self.register.p.carry as u8);
        let is_n_plus = (n & 0b10000000) == 0;
        self.register.p.carry = c;
        let overflow_flag = (is_n_plus != is_l_plus) && (is_n_plus != is_r_plus);
        self.register.p.overflow = overflow_flag;
        n
    }

    pub fn sign_minus(&mut self, l: u8, r: u8) -> u8 {
        let is_l_minus = (l & 0b10000000) != 0;

        let (n, c1) = l.overflowing_sub(r);
        let l = n;
        let (n, c2) = l.overflowing_sub(1 - self.register.p.carry as u8);
        let is_n_plus = (n & 0b10000000) == 0;
        self.register.p.carry = !(c1 | c2);
        let overflow_flag = is_l_minus && is_n_plus;
        self.register.p.overflow = overflow_flag;
        n
    }

    pub fn run_ope(&mut self, r: u16, opekind: OpeKind, addr_mode: AddrMode) {
        match opekind {
            OpeKind::Adc => {
                let m = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u16;
                let n = self.sign_plus(self.register.a, m as u8);
                self.register.a = n;
                self.set_nz(self.register.a);
            }
            OpeKind::Sbc => {
                let m = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u16;
                let n = self.sign_minus(self.register.a, m as u8);
                self.register.a = n;
                self.set_nz(self.register.a);
            }
            OpeKind::And => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a &= r;
                self.set_nz(self.register.a);
            }
            OpeKind::Ora => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a |= r;
                self.set_nz(self.register.a);
            }
            OpeKind::Eor => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.register.a ^= r;
                self.set_nz(self.register.a);
            }
            OpeKind::Asl => match addr_mode {
                AddrMode::Acc => {
                    let mut a = r as u8;
                    self.set_carry((a & 0b10000000) != 0);
                    a <<= 1;
                    self.set_nz(a);
                    self.register.a = a;
                }
                _ => {
                    let mut v = self.bus.addr(r);
                    self.set_carry((v & 0b10000000) != 0);
                    v <<= 1;
                    self.set_nz(v);
                    self.bus.set(r, v);
                }
            },
            OpeKind::Lsr => match addr_mode {
                AddrMode::Acc => {
                    let mut a = r as u8;
                    self.set_carry((a & 0b00000001) != 0);
                    a >>= 1;
                    self.set_nz(a);
                    self.register.a = a;
                }
                _ => {
                    let mut v = self.bus.addr(r);
                    self.set_carry((v & 0b00000001) != 0);
                    v >>= 1;
                    self.set_nz(v);
                    self.bus.set(r, v);
                }
            },
            OpeKind::Rol => match addr_mode {
                AddrMode::Acc => {
                    let mut a = r as u8;
                    let c = self.register.p.carry;
                    self.set_carry((a & 0b10000000) != 0);
                    a <<= 1;
                    a |= c as u8;
                    self.set_nz(a);
                    self.register.a = a;
                }
                _ => {
                    let mut m = self.bus.addr(r);
                    let c = self.register.p.carry;
                    self.set_carry((m & 0b10000000) != 0);
                    m <<= 1;
                    m |= c as u8;
                    self.set_nz(m);
                    self.bus.set(r, m);
                }
            },
            OpeKind::Ror => {
                match addr_mode {
                    AddrMode::Acc => {
                        let mut a = r as u8;
                        let c = self.register.p.carry;
                        self.set_carry((a & 0b00000001) != 0);
                        a >>= 0x1;
                        a |= (c as u8) << 7;
                        self.set_nz(a);
                        self.register.a = a;
                    }
                    _ => {
                        let mut v = self.bus.addr(r);
                        let c = self.register.p.carry;
                        self.set_carry((v & 0b00000001) != 0);
                        v >>= 0x1;
                        v |= (c as u8) << 7;
                        self.set_nz(v);
                        self.bus.set(r, v);
                    }
                };
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
            OpeKind::Bit => {
                let v = self.bus.addr(r);
                self.set_zero((v & self.register.a) == 0);
                self.register.p.negative = (v & 0b10000000) != 0;
                self.register.p.overflow = (v & 0b01000000) != 0;
            }
            OpeKind::Cmp => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode);
                let (m, f) = self.register.a.overflowing_sub(r as u8);
                self.set_nz(m as u8);
                self.set_carry(!f);
            }
            OpeKind::Cpx => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.set_carry(self.register.x >= r);
                self.set_zero(self.register.x == r);
                let n = self.register.x.wrapping_sub(r);
                self.set_negative((n & 0b10000000) != 0);
            }
            OpeKind::Cpy => {
                let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
                self.set_carry(self.register.y >= r);
                self.set_zero(self.register.y == r);
                let n = self.register.y.wrapping_sub(r);
                self.set_negative((n & 0b10000000) != 0);
            }
            OpeKind::Inc => {
                let v = self.bus.addr(r);
                let s = self.ex_plus(v, 1);
                self.bus_set(r, s);
                self.set_nz(s);
            }
            OpeKind::Dec => {
                let v = self.bus.addr(r);
                let n = v.wrapping_sub(1);
                self.bus_set(r, n);
                self.set_nz(n);
            }
            OpeKind::Inx => {
                let x = self.ex_i8_plus(self.register.x, 1);
                self.set_x(x);
                self.set_nz(x);
            }
            OpeKind::Dex => {
                let x = self.register.x as i16 - 1;
                self.set_x(x as u8);
                self.set_nz(x as u8);
            }
            OpeKind::Iny => {
                let y = self.ex_i8_plus(self.register.y, 1);
                self.set_y(y as u8);
                self.set_nz(y as u8);
            }
            OpeKind::Dey => {
                let y = self.register.y as i16 - 1;
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
                self.bus_set(r, self.register.a);
            }
            OpeKind::Stx => {
                self.bus_set(r, self.register.x);
            }
            OpeKind::Sty => {
                self.bus_set(r, self.register.y);
            }
            OpeKind::Tax => {
                self.register.x = self.register.a;
                self.set_nz(self.register.x);
            }
            OpeKind::Txa => {
                self.register.a = self.register.x;
                self.set_nz(self.register.a);
            }
            OpeKind::Tsx => {
                let s = self.register.s as u8;
                self.register.x = s;
                self.set_nz(s);
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
                // self.register.set_s(self.register.x);
                self.register.set_s(self.register.x);
            }
            OpeKind::Pha => {
                self.push_stack(self.register.a);
            }
            OpeKind::Pla => {
                self.register.a = self.pull_stack();
                self.set_nz(self.register.a);
            }
            OpeKind::Php => {
                let mut n = self.register.p.to_n();
                n |= 0b00010000;
                self.push_stack(n);
            }
            OpeKind::Plp => {
                let mut n = self.pull_stack();
                n &= 0b11101111;
                self.register.p.set(n);
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
                self.register.pc = self.register.pc.wrapping_add(1);
            }
            OpeKind::Brk => {
                if !self.get_interrupt() {
                    self.register.pc -= 1;
                    self.push_pc();
                    let mut n = self.register.p.to_n();
                    n |= 0b00010000;
                    self.push_stack(n);
                    self.set_break_mode(true);
                    self.set_interrupt(true);
                    let (h, l) = self.bus.cpu_bus.hl_addr(0xFFFE);
                    self.register.pc = combine_high_low(l, h);
                }
            }
            OpeKind::Rti => {
                let p = self.pull_stack();
                let l = self.pull_stack();
                let h = self.pull_stack();
                self.register.pc = combine_high_low(l, h);
                self.register.p.set(p);
            }
            OpeKind::Nop => {
                self.nop();
            }
            _ => self.nop(),
        }
    }

    pub fn ex_ope(&mut self) {
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
                self.cycle += cycle as u16;
                // println!("pc: {:0x?}, reg_addr: {:0x?}", self.register.pc, reg_addr);
            }
            None => {
                self.undef();
                self.cycle += 1;
            }
        }
    }

    pub fn clear_cycle(&mut self) {
        self.cycle = 0;
    }

    pub fn read_ope(&mut self) -> Option<&Operator> {
        let c = self.fetch_code();
        // print!("\n{:0x?} ", self.register.pc);
        // print!(
        //     "{:>02x} {:>02x} {:>02x} ",
        //     c,
        //     self.fetch_next_register(),
        //     self.fetch_next_next_register(),
        // );
        // // print!("{:?}  ", self.operators.get_mut(&c).unwrap().ope_kind);
        // print!(
        //     "{} {}  ",
        //     format!("{:?}", self.operators.get_mut(&c).unwrap().ope_kind).to_uppercase(),
        //     format!("{:?}", self.operators.get_mut(&c).unwrap().addr_mode).to_uppercase()
        // );
        // print!(
        //     "A:{:>02x} X:{:>02x} Y:{:>02x} P:{:>02x} S:{:>02x}",
        //     self.register.a,
        //     self.register.x,
        //     self.register.y,
        //     self.register.p.to_n(),
        //     self.register.s,
        // );

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
    use std::fs::File;
    use std::io::Read;

    impl Nes {
        pub fn new_for_test() -> Self {
            let mut f = File::open("roms/hello-world.nes").unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).unwrap();
            let header = Header::new(&buffer);
            Self { header }
        }
    }

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
        let nes = Nes::new_for_test();
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
        let r = if h < 0x80 {
            (l + h) as u16
        } else {
            (l + h - 256) as u16
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

        let mut l = cpu.fetch_next_register();
        let h = cpu.register.x;
        l = l.wrapping_add(h);
        cpu.bus_set(h as u16, rand_u8());
        cpu.bus_set((h + 1) as u16, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(l as u16);
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
        cpu.bus_set(t, rand_u8());
        cpu.bus_set(t + 1, rand_u8());

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

        cpu.bus_set(t, rand_u8());
        cpu.bus_set(t + 1, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(t);
        let r = combine_high_low(l, h);

        let mut reg_addr = u16::MAX;
        cpu.set_next_reg_addr(&mut reg_addr);

        assert_eq!(reg_addr, r);
    }
}
