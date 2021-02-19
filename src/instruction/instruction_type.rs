/// all register flags: NV#BDIZC

/// Type of instruction
/// See http://6502.org/tutorials/6502opcodes.html
/// See http://obelisk.me.uk/6502/reference.html
#[derive(Copy, Clone)]
pub enum InstructionType {
    /// Add with carry
    /// Affects: `NVZC`
    Adc,
    /// Bitwise and
    /// Affects: `NZ`
    And,
    /// Arithmetic shift left
    /// Affects: `NZC`
    Asl,
    /// sets the Z flag as though the value in the address tested were ANDed with the accumulator. The N and V flags are set to match bits 7 and 6 respectively in the value stored at the tested address.
    /// Affects: `NVZ`
    Bit,
    /// Branch on plus
    Bpl,
    /// Branch on minus
    Bmi,
    /// Branch on overflow clear
    Bvc,
    /// Branch on overflow set
    Bvs,
    /// Branch on carry clear
    Bcc,
    /// Branch on carry set
    Bcs,
    /// Branch on not equal
    Bne,
    /// Branch on equal
    Beq,
    /// Break
    /// Affects: `B`
    Brk,
    /// Compare accumulator
    /// Affects: `NZC`
    Cmp,
    /// Compare the X register
    /// Affects: `NZC`
    Cpx,
    /// Compare the Y register
    /// Affects: `NZC`
    Cpy,
    /// Decrement memory
    /// Affects: `NZ`
    Dec,
    /// Bitwise exclusive or
    /// Affects: `NZ`
    Eor,
    /// Clear carry
    Clc,
    /// Set carry
    Sec,
    /// Clear interrupt
    Cli,
    /// Set interrupt
    Sei,
    /// Clear overflow
    Clv,
    /// Clear decimal
    Cld,
    /// Set decimal
    Sed,
    /// Increment memory
    /// Affects: `NZ`
    Inc,
    /// Jump
    Jmp,
    /// Jump to subroutine
    Jsr,
    /// Load accumulator
    /// Affects: `NZ`
    Lda,
    /// Load X register
    /// Affects: `NZ`
    Ldx,
    /// Load Y register
    /// Affects: `NZ`
    Ldy,
    /// Logical shift right
    /// Affects: `NZC`
    Lsr,
    /// No operation
    Nop,
    /// Bitwise or with accumulator
    /// Affects: `NZ`
    Ora,
    /// Transfer A to X
    /// Affects: `NZ`
    Tax,
    /// Transfer X to A
    /// Affects: `NZ`
    Txa,
    /// Decrement X
    /// Affects: `NZ`
    Dex,
    /// Increment X
    /// Affects: `NZ`
    Inx,
    /// Transfer A to Y
    /// Affects: `NZ`
    Tay,
    /// Transfer Y to A
    /// Affects: `NZ`
    Tya,
    /// Decrement Y
    /// Affects: `NZ`
    Dey,
    /// Increment Y
    /// Affects: `NZ`
    Iny,
    /// Rotate left
    /// Affects: `NZC`
    Rol,
    /// Rotate right
    /// Affects: `NZC`
    Ror,
    /// Return from interrupt
    /// Affects: `NV#BDIZC`
    Rti,
    /// Return from subroutine
    Rts,
    /// Subtract with carry
    /// Affects: `NVZC`
    Sbc,
    /// Store accumulator
    Sta,
    /// Transfer X to stack ptr
    Txs,
    /// Transfer stack ptr to X
    Tsx,
    /// Push A
    Pha,
    /// Pull A
    Pla,
    /// Push processor status
    Php,
    /// Pull processor status
    Plp,
    /// Store X register
    Stx,
    /// Store Y register
    Sty,
}
