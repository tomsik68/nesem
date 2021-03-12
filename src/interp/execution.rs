use super::operand_decoder;
use crate::instruction::operand::Operand;
use crate::interp::state::State;

fn lda(state: &mut State, op: &Operand) {
    let value = match op {
        Operand::Immediate(x) => *x,
        Operand::Accumulator => state.accumulator,
        Operand::Implicit => panic!("lda with implicit operand impossible!"),
        _ => unimplemented!("nyi"),
    };

    state.accumulator = value;
}

/// Create a function @name which checks the flag @flag.
/// For the 's' variant: branch happens if @flag is set
/// For the 'c' variant: branch happens if @flag is clear
macro_rules! branch_inst {
    ($name:ident, $pred:expr) => {
        fn $name(state: &mut State, op: &Operand) {
            let dest = match op {
                Operand::Relative(rel) => state.pc.wrapping_add((*rel as i16) as u16),
                _ => unimplemented!("{}: operand is not Relative(i8)", stringify!($name)),
            };

            if $pred(&state) {
                state.pc = dest;
            }
        }
    };
}

branch_inst!(bcc, |s: &State| !s.get_carry());
branch_inst!(bcs, |s: &State| s.get_carry());
branch_inst!(beq, |s: &State| s.get_zero());
branch_inst!(bne, |s: &State| !s.get_zero());
branch_inst!(bmi, |s: &State| s.get_negative());
branch_inst!(bpl, |s: &State| !s.get_negative());

branch_inst!(bvc, |s: &State| !s.get_overflow());
branch_inst!(bvs, |s: &State| s.get_overflow());

fn bit(state: &mut State, op: &Operand) {
    let a = state.accumulator;
    let v = operand_decoder::get_u8(op, state).expect("bit: operand with value is required");

    let r = a & v;
    state.set_negative(r & (1 << 7) > 0);
    state.set_overflow(r & (1 << 6) > 0);
}

// TODO BRK instruction

#[cfg(test)]
mod tests {
    mod bcc {
        use crate::instruction::operand::Operand;
        use crate::interp::execution::bcc;
        use crate::interp::state::State;
        #[test]
        fn test_bcc_carry_not_clear() {
            let mut state = State::new_undefined();
            state.pc = 0;
            state.set_carry(true);
            let op = Operand::Relative(100);
            bcc(&mut state, &op);

            assert_eq!(state.pc, 0);
        }

        #[test]
        fn test_bcc_carry_clear() {
            let mut state = State::new_undefined();
            state.pc = 0;
            state.set_carry(false);
            let op = Operand::Relative(100);
            bcc(&mut state, &op);

            assert_eq!(state.pc, 100);
        }
    }

    mod bit {
        use crate::instruction::operand::Operand;
        use crate::interp::execution::bit;
        use crate::interp::state::State;

        #[test]
        fn test_bit_zeros() {
            let mut state = State::new_undefined();
            state.accumulator = 0;
            state.set_negative(true);
            state.set_overflow(true);

            let op = Operand::Immediate(0xFF);
            bit(&mut state, &op);

            assert!(!state.get_overflow());
            assert!(!state.get_negative());
        }

        #[test]
        fn test_bit_ones() {
            let mut state = State::new_undefined();
            state.accumulator = 0xFF;
            state.set_negative(false);
            state.set_overflow(false);

            let op = Operand::Immediate(0xFF);
            bit(&mut state, &op);

            assert!(state.get_overflow());
            assert!(state.get_negative());
        }

        #[test]
        fn test_bit_mixed_7() {
            let mut state = State::new_undefined();

            state.accumulator = 0b1000_0000;

            state.set_negative(false);
            state.set_overflow(true);

            let op = Operand::Immediate(0xFF);
            bit(&mut state, &op);

            assert!(!state.get_overflow());
            assert!(state.get_negative());
        }

        #[test]
        fn test_bit_mixed_6() {
            let mut state = State::new_undefined();

            state.accumulator = 0b0100_0000;

            state.set_negative(true);
            state.set_overflow(false);

            let op = Operand::Immediate(0xFF);
            bit(&mut state, &op);

            assert!(state.get_overflow());
            assert!(!state.get_negative());
        }
    }
}

pub use super::alu::adc;
