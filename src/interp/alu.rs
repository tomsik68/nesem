use super::operand_decoder::{get_pointer, get_u8, get_value};
use super::state::State;
use crate::instruction::operand::Operand;

/// Interpret @a as an 8-bit twos complement integer.
/// Return true iff @a >= 0
fn is_positive(a: u8) -> bool {
    // check the sign bit
    a & 0x80 == 0
}

/// Interpret @a as an 8-bit twos complement integer.
/// Return true iff @a < 0
fn is_negative(a: u8) -> bool {
    !is_positive(a)
}

/// Return true iff adding @a + @b causes the oVerflow flag to be set to 1
/// This happens when @a + @b > 127 or @a + @b < -128
/// @a and @b are signed twos complement integers represented by u8
fn is_add_overflow(a: u8, b: u8) -> bool {
    // adding positive and negative number decreases abs. value => no problems
    if is_positive(a) != is_positive(b) {
        return false;
    }

    let pos = is_positive(a);
    let n = a.wrapping_add(b);
    is_positive(a) != is_positive(n)
}

pub fn adc(state: &mut State, op: &Operand) {
    let value = get_value(&op, &state).unwrap() as u8;

    let (new, carry) = state.accumulator.overflowing_add(value);
    let overflow = is_add_overflow(value, state.accumulator);
    state.accumulator = new;
    state.set_carry(carry);
    state.set_overflow(carry);
    state.set_negative(new & 0b10000000 > 0);
    state.set_zero(new == 0);
}

pub fn and(state: &mut State, op: &Operand) {
    let value = get_value(&op, &state).unwrap() as u8;
    state.accumulator = state.accumulator & value;
    state.set_zero(state.accumulator == 0);
    state.set_negative(is_negative(state.accumulator));
}

pub fn asl(state: &mut State, op: &Operand) {
    use crate::instruction::operand::Operand::Accumulator;

    let ptr = get_pointer(&op, &state);
    let value = get_u8(&op, &state).unwrap();

    state.set_carry(is_negative(value));
    let value = value << 1;

    match ptr {
        None => {
            if let Accumulator = op {
                state.accumulator = value;
            } else {
                panic!("alu: asl: operand does not have an associated pointer and is not in an accumulator!");
            }
        }
        Some(p) => {
            state.ram_set(p, value);
        }
    }

    state.set_zero(state.accumulator == 0);
    state.set_negative(is_negative(value));
}

#[cfg(test)]
mod tests {

    mod asl {
        use super::super::asl;
        use crate::instruction::operand::Operand;
        use crate::interp::state::State;

        #[test]
        fn asl_accumulator() {
            let mut st = State::new_undefined();
            st.accumulator = 0x01;
            let op = Operand::Accumulator;
            asl(&mut st, &op);

            assert_eq!(st.accumulator, 0x02);
            assert!(!st.get_zero());
            assert!(!st.get_negative());
        }

        #[test]
        fn asl_flags() {
            let mut st = State::new_undefined();
            st.accumulator = 0x0;
            let op = Operand::Accumulator;
            asl(&mut st, &op);

            assert_eq!(st.accumulator, 0x0);
            assert!(st.get_zero());
            assert!(!st.get_negative());

            st.accumulator = 0xFF;
            let op = Operand::Accumulator;
            asl(&mut st, &op);

            assert_eq!(st.accumulator, 0xFE);
            assert!(!st.get_zero());
            assert!(st.get_negative());
            assert!(st.get_carry());
        }

        #[test]
        fn asl_inplace_absolute_test() {
            let mut st = State::new_undefined();
            st.ram_set(0xAA, 0x01);

            let op = Operand::Absolute(0xAA);
            asl(&mut st, &op);

            assert_eq!(st.ram_get(0xAA), 0x02);
            assert!(st.get_zero());
            assert!(!st.get_negative());
        }
    }

    mod and {
        use super::super::and;
        use crate::instruction::operand::Operand;
        use crate::interp::state::State;

        #[test]
        fn and_test() {
            let mut st = State::new_undefined();
            st.accumulator = 0xFF;
            let op = Operand::Immediate(20);
            and(&mut st, &op);

            assert_eq!(st.accumulator, 20);
            assert!(!st.get_zero());
            assert!(!st.get_negative());

            st.accumulator = 0x00;
            let op = Operand::Immediate(0xFF);
            and(&mut st, &op);

            assert_eq!(st.accumulator, 0);
            assert!(st.get_zero());
            assert!(!st.get_negative());
        }
    }

    mod is_add_overflow {
        use super::super::is_add_overflow;

        #[test]
        fn is_overflow_test_positive() {
            assert!(!is_add_overflow(0, 0));
            assert!(!is_add_overflow(17, 61));
            assert!(!is_add_overflow(64, 63));
            assert!(is_add_overflow(64, 64));
            assert!(is_add_overflow(0x7F, 0x7F));
        }

        #[test]
        fn is_overflow_test_negative() {
            assert!(!is_add_overflow(1, 0xFF));
            assert!(!is_add_overflow(2, 0xFE));
            // 0xFF == -1
            assert!(!is_add_overflow(0xFF, 0xFE));
            // 0xC0 == -64
            assert!(!is_add_overflow(0xC0, 0xC0));
            assert!(is_add_overflow(0xBF, 0xC0));
            // 0x80 == -128
            assert!(is_add_overflow(0x80, 0xFF));
            assert!(is_add_overflow(0x80, 0x80));
        }
    }

    mod adc {
        use super::super::adc;
        use crate::instruction::operand::Operand;
        use crate::interp::state::State;
        #[test]
        #[should_panic]
        fn adc_implicit_test() {
            let mut st = State::new_undefined();
            let op = Operand::Implicit;
            adc(&mut st, &op);
        }

        #[test]
        fn adc_immediate_test() {
            let mut st = State::new_undefined();
            let orig = st.accumulator;
            let op = Operand::Immediate(20);
            adc(&mut st, &op);
            assert_eq!(orig + 20, st.accumulator);
            assert!(!st.get_carry());
            assert!(!st.get_zero());
            assert!(!st.get_overflow());
            assert!(!st.get_negative());
        }

        #[test]
        fn adc_immediate_overflow_test() {
            let mut st = State::new_undefined();
            st.accumulator = 254;
            let op = Operand::Immediate(2);
            adc(&mut st, &op);
            assert_eq!(0, st.accumulator);
            assert!(st.get_carry());
            assert!(st.get_carry());
        }
    }
}
