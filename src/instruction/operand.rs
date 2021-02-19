/// Represents an operand of an instruction
/// http://obelisk.me.uk/6502/addressing.html
pub enum Operand {
    /// Operand is well defined for the given instruction
    Implicit,
    /// Operate on Accumulator
    Accumulator,
    /// Immediate, in instruction
    Immediate(u8),
    /// 8 bit address, operand is on the first page in memory
    ZeroPage(u8),
    /// `address = (X + value) % 256`
    ZeroPageX(u8),
    /// `address = (Y + value) % 256`
    ZeroPageY(u8),
    /// `address = PC + offset`
    Relative(i8),
    /// full 16-bit address
    Absolute(u16),
    /// `address = X + offset`
    AbsoluteX(u16),
    /// `address = Y + offset`
    AbsoluteY(u16),
    /// `address = *indirect`
    Indirect(u16),
    /// Offset is an address of a table
    /// `address = *(X + offset)`
    IndexedIndirect(u8),
    /// Offset is an address of a table
    /// `address = *(Y + offset)`
    IndirectIndexed(u8),
}
