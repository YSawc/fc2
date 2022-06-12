#[derive(Debug, Clone)]
pub struct Operator {
    pub ope_kind: OpeKind,
    pub addr_mode: AddrMode,
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
