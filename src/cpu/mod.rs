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
    fn new() -> Self {
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
                self.set_break_mode(false);
                self.set_interrupt(true);
                self.push_pc();
                let p = self.get_p();
                self.push_stack(p);
                let (l, h) = self.bus.cpu_bus.lh_addr(0xFFFA);
                self.register.set_pc(combine_high_low(l, h));
            }
            Interrupt::Reset => self.reset(),
            Interrupt::Irq => unimplemented!(),
            Interrupt::Brk => unimplemented!(),
        }
    }

    // MEMO: use for nestest.nes
    // pub fn reset(&mut self) {
    //     self.register.set_pc(0, 0xc0);
    // }

    pub fn reset(&mut self) {
        let l = self.bus.addr(0xFFFC);
        let h = self.bus.addr(0xFFFD);
        self.set_x(l);
        self.set_y(h);
        self.register
            .set_pc(combine_high_low(self.get_x(), self.get_y()));
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

    fn ex_i8_plus(&mut self, l: u8, r: u8) -> u8 {
        if l < 0x80 {
            l + r
        } else {
            (l as i16 + r as i16) as u8
        }
    }

    pub fn set_break_mode(&mut self, b: bool) {
        self.register.mut_access_p().set_break_mode(b);
    }

    pub fn get_break_mode(&self) -> bool {
        self.register.access_p().get_break_mode()
    }

    pub fn set_interrupt(&mut self, b: bool) {
        self.register.mut_access_p().set_interrupt(b);
    }

    pub fn set_decimal(&mut self, b: bool) {
        self.register.mut_access_p().set_decimal(b);
    }

    pub fn get_interrupt(&self) -> bool {
        self.register.access_p().get_interrupt()
    }

    pub fn set_negative(&mut self, b: bool) {
        self.register.mut_access_p().set_negative(b);
    }

    pub fn get_negative(&self) -> bool {
        self.register.access_p().get_negative()
    }

    pub fn set_overflow(&mut self, b: bool) {
        self.register.mut_access_p().set_overflow(b);
    }

    pub fn get_overflow(&self) -> bool {
        self.register.access_p().get_overflow()
    }

    pub fn set_zero(&mut self, b: bool) {
        self.register.mut_access_p().set_zero(b);
    }

    pub fn get_zero(&self) -> bool {
        self.register.access_p().get_zero()
    }

    pub fn set_nz(&mut self, n: u8) {
        self.set_negative(n >= 0x80);
        self.set_zero(n == 0);
    }

    pub fn set_carry(&mut self, b: bool) {
        self.register.mut_access_p().set_carry(b)
    }

    pub fn get_carry(&self) -> bool {
        self.register.access_p().get_carry()
    }

    pub fn set_a(&mut self, n: u8) {
        self.register.set_a(n);
    }

    pub fn get_a(&self) -> u8 {
        self.register.get_a()
    }

    pub fn set_x(&mut self, n: u8) {
        self.register.set_x(n);
    }

    pub fn get_x(&self) -> u8 {
        self.register.get_x()
    }

    pub fn set_y(&mut self, n: u8) {
        self.register.set_y(n);
    }

    pub fn get_y(&self) -> u8 {
        self.register.get_y()
    }

    pub fn set_s(&mut self, n: u8) {
        self.register.set_s(n);
    }

    pub fn get_s(&self) -> u8 {
        self.register.get_s()
    }

    pub fn set_p(&mut self, n: u8) {
        self.register.set_p(n);
    }

    pub fn get_p(&self) -> u8 {
        self.register.get_p()
    }

    pub fn mut_access_p(&mut self, b: bool) {
        self.register.mut_access_p().set_break_mode(b);
    }

    pub fn set_pc(&mut self, n: u16) {
        self.register.set_pc(n);
    }

    pub fn get_pc(&self) -> u16 {
        self.register.get_pc()
    }

    pub fn inc_pc(&mut self, n: u16) {
        self.register.inc_pc(n);
    }

    pub fn dec_pc(&mut self, n: u16) {
        self.register.dec_pc(n);
    }

    pub fn fetch_register(&mut self) -> u8 {
        self.bus.addr(self.get_pc())
    }

    fn fetch_lh_register(&mut self) -> (u8, u8) {
        let l = self.fetch_register();
        let h = self.fetch_next_register();
        (l, h)
    }

    fn fetch_next_register(&mut self) -> u8 {
        let pc = self.get_pc().wrapping_add(1);
        self.bus.addr(pc)
    }

    fn undef(&mut self) {
        let pc = self.get_pc();
        self.inc_pc(pc.wrapping_add(1))
    }

    pub fn push_stack(&mut self, n: u8) {
        let l = self.get_s() as u16;
        let s = self.get_s().wrapping_sub(1);
        self.set_s(s);
        let r = l.wrapping_add(1 << 8);
        self.bus_set(r, n);
    }

    fn pull_stack(&mut self) -> u8 {
        let l = self.get_s().wrapping_add(1);
        self.set_s(l);
        let h = 0x100;
        let r = (l as u16).wrapping_add(h);
        self.bus.addr(r)
    }

    fn set_oam(&mut self) {
        self.cycle += 513;
        if self.cycle % 2 != 0 {
            self.cycle += 1;
        }
        let mut sprite_infos = vec![];

        let r = self.bus.addr(0x4014);
        for l in 0..0xff {
            let data = self.bus.addr(combine_high_low(l, r));
            sprite_infos.push(data);
        }
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

    fn acc(&mut self) -> u16 {
        let l = self.get_a() as u16;
        l as u16
    }

    fn imm(&mut self) -> u16 {
        let l = self.fetch_register();
        l as u16
    }

    fn zp(&mut self) -> u16 {
        let l = self.fetch_register() as u16;
        l
    }

    fn zpx(&mut self) -> u16 {
        let l = self.fetch_register();
        let l = l.wrapping_add(self.get_x());
        l as u16
    }

    fn zpy(&mut self) -> u16 {
        let l = self.fetch_register();
        let l = l.wrapping_add(self.get_y());
        l as u16
    }

    fn abs(&mut self) -> u16 {
        let (l, h) = self.fetch_lh_register();
        combine_high_low(l, h)
    }

    fn abs_x(&mut self) -> u16 {
        let (l, h) = self.fetch_lh_register();
        let x = self.get_x() as u16;
        let m = combine_high_low(l, h);
        let (m, c) = m.overflowing_add(x);
        if c {
            self.set_carry(c);
        }
        m
    }

    fn abs_y(&mut self) -> u16 {
        let (l, h) = self.fetch_lh_register();
        let y = self.get_y() as u16;
        let m = combine_high_low(l, h);
        let (m, c) = m.overflowing_add(y);
        if c {
            self.set_carry(c);
        }
        m
    }

    fn rel(&mut self) -> u16 {
        let l = self.get_pc() + 1;
        let h = self.fetch_register() as u16;
        if h < 0x80 {
            (l + h) as u16
        } else {
            (l + h - 256) as u16
        }
    }

    fn ind_x(&mut self) -> u16 {
        let l = self.fetch_register();
        let r = self.get_x();
        let t = l.wrapping_add(r);
        let (l, h) = self.bus.cpu_bus.lh_zeropage_addr(t);
        combine_high_low(l, h)
    }

    fn ind_y(&mut self) -> u16 {
        let t = self.fetch_register();
        let (l, h) = self.bus.cpu_bus.lh_zeropage_addr(t);
        let r = self.get_y() as u16;
        let l = combine_high_low(l as u8, h as u8);
        let t = l.wrapping_add(r);
        t
    }

    fn ind(&mut self) -> u16 {
        let (l, h) = self.fetch_lh_register();
        let t = combine_high_low(l, h);
        let (l, h) = self.bus.cpu_bus.lh_ignore_overflowing_addr(t);
        combine_high_low(l, h)
    }

    fn nop(&mut self) -> u16 {
        0
    }

    fn ex_addr_mode(&mut self, addr_mode: &AddrMode) -> u16 {
        self.inc_pc(1);
        let r = match addr_mode {
            AddrMode::Impl => 0,
            AddrMode::Acc => self.acc(),
            AddrMode::Imm => self.imm(),
            AddrMode::Zp => self.zp(),
            AddrMode::ZpX => self.zpx(),
            AddrMode::ZpY => self.zpy(),
            AddrMode::Abs => self.abs(),
            AddrMode::AbsX => self.abs_x(),
            AddrMode::AbsY => self.abs_y(),
            AddrMode::Rel => self.rel(),
            AddrMode::IndX => self.ind_x(),
            AddrMode::IndY => self.ind_y(),
            AddrMode::Ind => self.ind(),
            AddrMode::Nop => self.nop(),
        };

        match addr_mode {
            AddrMode::Acc | AddrMode::Impl | AddrMode::Nop => (),
            AddrMode::Imm
            | AddrMode::Zp
            | AddrMode::ZpX
            | AddrMode::ZpY
            | AddrMode::Rel
            | AddrMode::IndX
            | AddrMode::IndY => self.inc_pc(1),
            AddrMode::Abs | AddrMode::AbsX | AddrMode::AbsY | AddrMode::Ind => self.inc_pc(2),
        }

        r
    }

    fn is_branch_enable(&self) -> bool {
        if self.get_interrupt() {
            false
        } else {
            true
        }
    }

    fn get_addr_for_mixed_imm_mode(&mut self, r: u16, addr_mode: AddrMode) -> u16 {
        let imm = matches!(addr_mode, AddrMode::Imm);
        if imm {
            r
        } else {
            self.bus.addr(r) as u16
        }
    }

    fn push_pc(&mut self) {
        let p = self.get_pc();
        let h = ((p & 0xFF00) >> 8) as u8;
        let l = (p & 0x00FF) as u8;
        self.push_stack(h);
        self.push_stack(l);
    }

    fn sign_plus(&mut self, l: u8, r: u8) -> u8 {
        let is_l_plus = (l & 0b10000000) == 0;
        let is_r_plus = (r & 0b10000000) == 0;

        let (n, c1) = l.overflowing_add(r);
        let (n, c2) = n.overflowing_add(self.get_carry() as u8);

        let is_n_plus = (n & 0b10000000) == 0;
        self.set_carry(c1 || c2);
        let overflow_flag = (is_n_plus != is_l_plus) && (is_n_plus != is_r_plus);
        self.set_overflow(overflow_flag);
        n
    }

    fn sign_minus(&mut self, l: u8, r: u8) -> u8 {
        let is_l_minus = (l & 0b10000000) != 0;

        let (n, c1) = l.overflowing_sub(r);
        let l = n;
        let (n, c2) = l.overflowing_sub(1 - self.get_carry() as u8);
        let is_n_plus = (n & 0b10000000) == 0;
        self.set_carry(!(c1 | c2));
        let overflow_flag = is_l_minus && is_n_plus;
        self.set_overflow(overflow_flag);
        n
    }

    fn adc(&mut self, r: u16, addr_mode: AddrMode) {
        let m = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u16;
        let n = self.sign_plus(self.get_a(), m as u8);
        self.set_a(n);
        self.set_nz(self.get_a());
    }

    fn sbc(&mut self, r: u16, addr_mode: AddrMode) {
        let m = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u16;
        let n = self.sign_minus(self.get_a(), m as u8);
        self.set_a(n);
        self.set_nz(self.get_a());
    }

    fn and(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        let mut a = self.get_a();
        a &= r;
        self.set_a(a);
        self.set_nz(a);
    }

    fn ora(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        let mut a = self.get_a();
        a |= r;
        self.set_a(a);
        self.set_nz(a);
    }

    fn eor(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        let mut a = self.get_a();
        a ^= r;
        self.set_a(a);
        self.set_nz(a);
    }

    fn asl(&mut self, r: u16, addr_mode: AddrMode) {
        match addr_mode {
            AddrMode::Acc => {
                let mut a = r as u8;
                self.set_carry((a & 0b10000000) != 0);
                a <<= 1;
                self.set_nz(a);
                self.set_a(a);
            }
            _ => {
                let mut v = self.bus.addr(r);
                self.set_carry((v & 0b10000000) != 0);
                v <<= 1;
                self.set_nz(v);
                self.bus.set(r, v);
            }
        }
    }

    fn lsr(&mut self, r: u16, addr_mode: AddrMode) {
        match addr_mode {
            AddrMode::Acc => {
                let mut a = r as u8;
                self.set_carry((a & 0b00000001) != 0);
                a >>= 1;
                self.set_nz(a);
                self.set_a(a);
            }
            _ => {
                let mut v = self.bus.addr(r);
                self.set_carry((v & 0b00000001) != 0);
                v >>= 1;
                self.set_nz(v);
                self.bus.set(r, v);
            }
        }
    }

    fn rol(&mut self, r: u16, addr_mode: AddrMode) {
        match addr_mode {
            AddrMode::Acc => {
                let mut a = r as u8;
                let c = self.get_carry();
                self.set_carry((a & 0b10000000) != 0);
                a <<= 1;
                a |= c as u8;
                self.set_nz(a);
                self.set_a(a);
            }
            _ => {
                let mut m = self.bus.addr(r);
                let c = self.get_carry();
                self.set_carry((m & 0b10000000) != 0);
                m <<= 1;
                m |= c as u8;
                self.set_nz(m);
                self.bus.set(r, m);
            }
        }
    }

    fn ror(&mut self, r: u16, addr_mode: AddrMode) {
        match addr_mode {
            AddrMode::Acc => {
                let mut a = r as u8;
                let c = self.get_carry();
                self.set_carry((a & 0b00000001) != 0);
                a >>= 0x1;
                a |= (c as u8) << 7;
                self.set_nz(a);
                self.set_a(a);
            }
            _ => {
                let mut v = self.bus.addr(r);
                let c = self.get_carry();
                self.set_carry((v & 0b00000001) != 0);
                v >>= 0x1;
                v |= (c as u8) << 7;
                self.set_nz(v);
                self.bus.set(r, v);
            }
        };
    }

    fn bcc(&mut self, r: u16) {
        if !self.get_carry() {
            self.set_pc(r);
        }
    }

    fn bcs(&mut self, r: u16) {
        if self.get_carry() {
            self.set_pc(r);
        }
    }

    fn beq(&mut self, r: u16) {
        if self.get_zero() {
            self.set_pc(r);
        }
    }

    fn bne(&mut self, r: u16) {
        if !self.get_zero() {
            self.set_pc(r);
        }
    }

    fn bvc(&mut self, r: u16) {
        if !self.get_overflow() {
            self.set_pc(r);
        }
    }

    fn bvs(&mut self, r: u16) {
        if self.get_overflow() {
            self.set_pc(r);
        }
    }

    fn bpl(&mut self, r: u16) {
        if !self.get_negative() {
            self.set_pc(r);
        }
    }

    fn bmi(&mut self, r: u16) {
        if self.get_negative() {
            self.set_pc(r);
        }
    }

    fn bit(&mut self, r: u16) {
        let v = self.bus.addr(r);
        self.set_zero((v & self.get_a()) == 0);
        self.set_negative((v & 0b10000000) != 0);
        self.set_overflow((v & 0b01000000) != 0);
    }

    fn cmp(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode);
        let (m, f) = self.get_a().overflowing_sub(r as u8);
        self.set_nz(m as u8);
        self.set_carry(!f);
    }

    fn cpx(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        let x = self.get_x();
        self.set_carry(x >= r);
        self.set_zero(x == r);
        let n = self.get_x().wrapping_sub(r);
        self.set_negative((n & 0b10000000) != 0);
    }
    fn cpy(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        let y = self.get_y();
        self.set_carry(y >= r);
        self.set_zero(y == r);
        let n = self.get_y().wrapping_sub(r);
        self.set_negative((n & 0b10000000) != 0);
    }
    fn inc(&mut self, r: u16) {
        let v = self.bus.addr(r);
        let s = self.ex_plus(v, 1);
        self.bus_set(r, s);
        self.set_nz(s);
    }
    fn dec(&mut self, r: u16) {
        let v = self.bus.addr(r);
        let n = v.wrapping_sub(1);
        self.bus_set(r, n);
        self.set_nz(n);
    }
    fn inx(&mut self) {
        let x = self.ex_i8_plus(self.get_x(), 1);
        self.set_x(x);
        self.set_nz(x);
    }
    fn dex(&mut self) {
        let x = self.get_x() as i16 - 1;
        self.set_x(x as u8);
        self.set_nz(x as u8);
    }
    fn iny(&mut self) {
        let y = self.ex_i8_plus(self.get_y(), 1);
        self.set_y(y as u8);
        self.set_nz(y as u8);
    }
    fn dey(&mut self) {
        let y = self.get_y() as i16 - 1;
        self.set_y(y as u8);
        self.set_nz(y as u8);
    }
    fn clc(&mut self) {
        self.set_carry(false)
    }
    fn sec(&mut self) {
        self.set_carry(true)
    }
    fn cli(&mut self) {
        self.set_interrupt(false)
    }
    fn sei(&mut self) {
        self.set_interrupt(true)
    }
    fn cld(&mut self) {
        self.set_decimal(false)
    }
    fn sed(&mut self) {
        self.set_decimal(true)
    }
    fn clv(&mut self) {
        self.set_overflow(false)
    }
    fn lda(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        self.set_a(r);
        self.set_nz(self.get_a());
    }
    fn ldx(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        self.set_x(r);
        self.set_nz(self.get_x());
    }
    fn ldy(&mut self, r: u16, addr_mode: AddrMode) {
        let r = self.get_addr_for_mixed_imm_mode(r, addr_mode) as u8;
        self.set_y(r);
        self.set_nz(self.get_y());
    }
    fn sta(&mut self, r: u16) {
        self.bus_set(r, self.get_a());
    }
    fn stx(&mut self, r: u16) {
        self.bus_set(r, self.get_x());
    }
    fn sty(&mut self, r: u16) {
        self.bus_set(r, self.get_y());
    }
    fn tax(&mut self) {
        self.set_x(self.get_a());
        self.set_nz(self.get_x());
    }
    fn txa(&mut self) {
        self.set_a(self.get_x());
        self.set_nz(self.get_a());
    }
    fn tsx(&mut self) {
        let s = self.get_s() as u8;
        self.set_x(s);
        self.set_nz(s);
    }
    fn tay(&mut self) {
        self.set_y(self.get_a());
        self.set_nz(self.get_y());
    }
    fn tya(&mut self) {
        self.set_a(self.get_y());
        self.set_nz(self.get_a());
    }
    fn txs(&mut self) {
        self.set_s(self.get_x());
    }
    fn pha(&mut self) {
        self.push_stack(self.get_a());
    }
    fn pla(&mut self) {
        let m = self.pull_stack();
        self.set_a(m);
        self.set_nz(self.get_a());
    }
    fn php(&mut self) {
        let mut n = self.get_p();
        n |= 0b00100000;
        self.push_stack(n);
    }
    fn plp(&mut self) {
        let mut n = self.pull_stack();
        n &= 0b11011111;
        self.set_p(n);
    }
    fn jmp(&mut self, r: u16) {
        self.set_pc(r);
    }
    fn jsr(&mut self, r: u16) {
        let p = self.get_pc() - 1;
        let h = ((p & 0xFF00) >> 8) as u8;
        let l = (p & 0x00FF) as u8;
        self.push_stack(h);
        self.push_stack(l);
        self.set_pc(r);
    }
    fn rts(&mut self) {
        let l = self.pull_stack();
        let h = self.pull_stack();
        let t = combine_high_low(l, h);
        self.set_pc(t);
        self.set_pc(self.get_pc().wrapping_add(1));
    }
    fn brk(&mut self) {
        if self.is_branch_enable() {
            self.dec_pc(1);
            self.push_pc();
            let n = self.get_p();
            self.push_stack(n);
            self.set_break_mode(true);
            self.set_interrupt(true);
            let (h, l) = self.bus.cpu_bus.hl_addr(0xFFFE);
            self.set_pc(combine_high_low(l, h));
        }
    }
    fn rti(&mut self) {
        let p = self.pull_stack();
        let l = self.pull_stack();
        let h = self.pull_stack();
        self.set_pc(combine_high_low(l, h));
        self.set_p(p);
    }

    fn run_ope(&mut self, r: u16, opekind: OpeKind, addr_mode: AddrMode) {
        match opekind {
            OpeKind::Adc => self.adc(r, addr_mode),
            OpeKind::Sbc => self.sbc(r, addr_mode),
            OpeKind::And => self.and(r, addr_mode),
            OpeKind::Ora => self.ora(r, addr_mode),
            OpeKind::Eor => self.eor(r, addr_mode),
            OpeKind::Asl => self.asl(r, addr_mode),
            OpeKind::Lsr => self.lsr(r, addr_mode),
            OpeKind::Rol => self.rol(r, addr_mode),
            OpeKind::Ror => self.ror(r, addr_mode),
            OpeKind::Bcc => self.bcc(r),
            OpeKind::Bcs => self.bcs(r),
            OpeKind::Beq => self.beq(r),
            OpeKind::Bne => self.bne(r),
            OpeKind::Bvc => self.bvc(r),
            OpeKind::Bvs => self.bvs(r),
            OpeKind::Bpl => self.bpl(r),
            OpeKind::Bmi => self.bmi(r),
            OpeKind::Bit => self.bit(r),
            OpeKind::Cmp => self.cmp(r, addr_mode),
            OpeKind::Cpx => self.cpx(r, addr_mode),
            OpeKind::Cpy => self.cpy(r, addr_mode),
            OpeKind::Inc => self.inc(r),
            OpeKind::Dec => self.dec(r),
            OpeKind::Inx => self.inx(),
            OpeKind::Dex => self.dex(),
            OpeKind::Iny => self.iny(),
            OpeKind::Dey => self.dey(),
            OpeKind::Clc => self.clc(),
            OpeKind::Sec => self.sec(),
            OpeKind::Cli => self.cli(),
            OpeKind::Sei => self.sei(),
            OpeKind::Cld => self.cld(),
            OpeKind::Sed => self.sed(),
            OpeKind::Clv => self.clv(),
            OpeKind::Lda => self.lda(r, addr_mode),
            OpeKind::Ldx => self.ldx(r, addr_mode),
            OpeKind::Ldy => self.ldy(r, addr_mode),
            OpeKind::Sta => self.sta(r),
            OpeKind::Stx => self.stx(r),
            OpeKind::Sty => self.sty(r),
            OpeKind::Tax => self.tax(),
            OpeKind::Txa => self.txa(),
            OpeKind::Tsx => self.tsx(),
            OpeKind::Tay => self.tay(),
            OpeKind::Tya => self.tya(),
            OpeKind::Txs => self.txs(),
            OpeKind::Pha => self.pha(),
            OpeKind::Pla => self.pla(),
            OpeKind::Php => self.php(),
            OpeKind::Plp => self.plp(),
            OpeKind::Jmp => self.jmp(r),
            OpeKind::Jsr => self.jsr(r),
            OpeKind::Rts => self.rts(),
            OpeKind::Brk => self.brk(),
            OpeKind::Rti => self.rti(),
            OpeKind::Nop => (),
            _ => (),
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

    fn read_ope(&mut self) -> Option<&Operator> {
        let c = self.fetch_register();
        // print!("\n{:0x?} ", self.get_pc());
        // print!(
        //     "{:>02x} {:>02x} {:>02x} ",
        //     c,
        //     self.fetch_next_register(),
        //     self.fetch_next_next_register(),
        // );
        // print!(
        //     "{} {}  ",
        //     format!("{:?}", self.operators.get_mut(&c).unwrap().ope_kind).to_uppercase(),
        //     format!("{:?}", self.operators.get_mut(&c).unwrap().addr_mode).to_uppercase()
        // );
        // print!(
        //     "A:{:>02x} X:{:>02x} Y:{:>02x} P:{:>02x} S:{:>02x}",
        //     self.get_a(),
        //     self.get_x(),
        //     self.get_y(),
        //     self.get_p(),
        //     self.get_s(),
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
                    let addr_mode = &addr_mode.clone();
                    *reg_addr = self.ex_addr_mode(addr_mode);
                }
                None => (),
            };
        }

        fn insert_random_num_into_b1_b2(&mut self) {
            self.bus.cpu_bus.prg_rom1[1] = rand_u8();
            self.bus.cpu_bus.prg_rom1[2] = rand_u8();
        }

        fn fetch_next_lh_register(&mut self) -> (u8, u8) {
            self.inc_pc(1);
            let (l, h) = self.fetch_lh_register();
            self.dec_pc(1);
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
        cpu.set_x(rand_u8());
        let x = cpu.get_x() as u16;
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
        cpu.set_y(rand_u8());
        let y = cpu.get_y() as u16;
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

        let l = cpu.get_pc() + 2;
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
        cpu.set_x(rand_u8());

        let mut l = cpu.fetch_next_register();
        let h = cpu.get_x();
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
        cpu.set_y(rand_u8());

        let t = cpu.fetch_next_register() as u16;
        cpu.bus_set(t, rand_u8());
        cpu.bus_set(t + 1, rand_u8());

        let (l, h) = cpu.bus.cpu_bus.lh_addr(t);
        let y = cpu.get_y() as u16;
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
