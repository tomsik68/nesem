use super::state::State;
use crate::instruction::operand::Operand;

/// Load 16-bit integer from zero-page
fn load_le16_zp(state: &State, addr: u8) -> u16 {
    dbg!(addr);
    dbg!(addr.wrapping_add(1));
    let lsb = state.ram_get(addr as u16) as u16;
    let msb = state.ram_get(addr.wrapping_add(1) as u16) as u16;
    (msb << 8) | lsb
}

fn load_le16(state: &State, addr: u16) -> u16 {
    dbg!(addr);
    dbg!(addr.wrapping_add(1));
    let lsb = state.ram_get(addr) as u16;
    let msb = state.ram_get(addr.wrapping_add(1)) as u16;
    (msb << 8) | lsb
}

/// For a given operand @op, return an address in memory where the value can be found
/// Example:
/// ```
/// let op = Operand::Absolute(0xFFFF);
/// let state = State::new_undefined();
/// let addr = get_pointer(&op, &state);
/// let value = addr.map(|a| state.ram_get(a));
/// ```
pub fn get_pointer(op: &Operand, state: &State) -> Option<u16> {
    use crate::instruction::operand::Operand::*;
    match op {
        Implicit | Accumulator | Immediate(_) => None,
        ZeroPage(offset) => Some(*offset as u16),
        ZeroPageX(offset) => Some(state.x.wrapping_add(*offset).into()),
        ZeroPageY(offset) => Some(state.y.wrapping_add(*offset).into()),
        Relative(offset) => Some(state.pc.wrapping_add(*offset as u16)),
        Absolute(offset) => Some(*offset),
        AbsoluteX(offset) => Some(offset.wrapping_add(state.x as u16)),
        AbsoluteY(offset) => Some(offset.wrapping_add(state.y as u16)),
        Indirect(offset) => Some(load_le16(&state, *offset)),
        IndexedIndirect(table_addr) => Some(load_le16_zp(&state, table_addr.wrapping_add(state.x))),
        IndirectIndexed(table_addr_addr) => {
            let table_addr = load_le16(&state, *table_addr_addr as u16);
            Some(table_addr + state.y as u16)
        }
    }
}

/// For a given operand @op, return its value
/// Example:
/// ```
/// let op = Operand::Absolute(0xFFFE);
/// let state = State::new_undefined();
/// state.ram_set(0xFFFE, 0xBA);
/// state.ram_set(0xFFFF, 0xBA);
/// let value = get_value(op, state);
/// assert_eq!(value, 0xBABA);
/// ```
pub fn get_value(op: &Operand, state: &State) -> Option<u16> {
    use crate::instruction::operand::Operand::*;
    match op {
        Implicit => None,
        Accumulator => Some(state.accumulator.into()),
        Immediate(x) => Some(*x as u16),
        ptr => get_pointer(ptr, state).map(|p| load_le16(state, p)),
    }
}

pub fn get_u8(op: &Operand, state: &State) -> Option<u8> {
    use crate::instruction::operand::Operand::*;
    match op {
        Implicit => None,
        Accumulator => Some(state.accumulator.into()),
        Immediate(x) => Some(*x),
        ptr => get_pointer(ptr, state).map(|p| state.ram_get(p)),
    }
}

// TODO revisit the result type
// the only error here could be that the operand is not writable (i.e. implicit or immediate)
pub fn set_u8(op: &Operand, val: u8, state: &mut State) -> Result<(), ()> {
    let ptr = get_pointer(op, state);

    match ptr {
        Some(p) => {
            state.ram_set(p, val);
            Ok(())
        }
        None => match op {
            Operand::Accumulator => {
                state.accumulator = val;
                Ok(())
            }
            _ => Err(()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{get_pointer, Operand, State};

    #[test]
    fn implicit_addr_random() {
        let op = Operand::Implicit;
        let state = State::new_undefined();
        assert_eq!(get_pointer(&op, &state), None);
    }

    #[test]
    fn accumulator() {
        let op = Operand::Accumulator;
        let state = State::new_undefined();
        assert_eq!(get_pointer(&op, &state), None);
    }

    #[test]
    fn immediate() {
        let op = Operand::Immediate(67);
        let state = State::new_undefined();
        assert_eq!(get_pointer(&op, &state), None);
    }

    #[test]
    fn zero_page() {
        let op = Operand::ZeroPage(27);
        let state = State::new_undefined();
        assert_eq!(get_pointer(&op, &state), Some(27));
    }

    #[test]
    fn zero_page_x() {
        let op = Operand::ZeroPageX(27);
        let mut state = State::new_undefined();
        state.x = 20;
        assert_eq!(get_pointer(&op, &state), Some(47));
        state.x += 5;
        assert_eq!(get_pointer(&op, &state), Some(52));
    }

    #[test]
    fn zero_page_x_wrap() {
        let op = Operand::ZeroPageX(255);
        let mut state = State::new_undefined();
        state.x = 20;
        assert_eq!(get_pointer(&op, &state), Some(19));
    }

    #[test]
    fn zero_page_y() {
        let op = Operand::ZeroPageY(27);
        let mut state = State::new_undefined();
        state.y = 20;
        assert_eq!(get_pointer(&op, &state), Some(47));
        state.y = 2;
        assert_eq!(get_pointer(&op, &state), Some(29));
    }

    #[test]
    fn relative_negative() {
        let op = Operand::Relative(-2);
        let mut state = State::new_undefined();
        state.pc = 21;
        assert_eq!(get_pointer(&op, &state), Some(19));
    }

    #[test]
    fn relative_positive() {
        let op = Operand::Relative(2);
        let mut state = State::new_undefined();
        state.pc = 21;
        assert_eq!(get_pointer(&op, &state), Some(23));
    }

    #[test]
    fn absolute() {
        let op = Operand::Absolute(50413);
        let state = State::new_undefined();
        assert_eq!(get_pointer(&op, &state), Some(50413));
    }

    #[test]
    fn absolute_x() {
        let op = Operand::AbsoluteX(50413);
        let mut state = State::new_undefined();
        state.x = 17;
        assert_eq!(get_pointer(&op, &state), Some(50413 + 17));
    }

    #[test]
    fn absolute_y() {
        let op = Operand::AbsoluteY(50413);
        let mut state = State::new_undefined();
        state.y = 200;
        assert_eq!(get_pointer(&op, &state), Some(50413 + 200));
    }

    #[test]
    fn indirect() {
        let op = Operand::Indirect(0x0120);
        let mut state = State::new_undefined();
        state.ram_set(0x0120, 0xFC);
        state.ram_set(0x0121, 0xBA);
        assert_eq!(get_pointer(&op, &state), Some(0xBAFC));
    }

    #[test]
    fn indexed_indirect() {
        let op = Operand::IndexedIndirect(10);
        let mut state = State::new_undefined();
        state.x = 17;
        state.ram_set(27, 0xFC);
        state.ram_set(28, 0xBA);
        assert_eq!(get_pointer(&op, &state), Some(0xBAFC));
    }

    #[test]
    fn indexed_indirect_wrap() {
        let op = Operand::IndexedIndirect(0xF0);
        let mut state = State::new_undefined();
        state.x = 0xF;
        state.ram_set(0xFF, 0xFC);
        state.ram_set(0x00, 0xBA);
        assert_eq!(get_pointer(&op, &state), Some(0xBAFC));
    }
}
