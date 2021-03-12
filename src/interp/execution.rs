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
    ($name:ident, $flag:ident, s) => {
        fn $name(state: &mut State, op: &Operand) {
            let dest = match op {
                Operand::Relative(rel) => state.pc.wrapping_add((*rel as i16) as u16),
                _ => unimplemented!("{}: operand is not Relative(i8)", stringify!($name)),
            };

            if state.get_$flag() {
                state.pc = dest;
            }
        }
    };

    ($name:ident, $flag:ident, c) => {
        fn $name(state: &mut State, op: &Operand) {
            let dest = match op {
                Operand::Relative(rel) => state.pc.wrapping_add((*rel as i16) as u16),
                _ => unimplemented!("{}: operand is not Relative(i8)", stringify!($name)),
            };

            if !state.get_$flag() {
                state.pc = dest;
            }
        }
    };
}

branch_inst!(bcc, get_carry, c);
branch_inst!(bcs, get_carry, s);
branch_inst!(beq, get_zero, s);
branch_inst!(bne, get_zero, c);
branch_inst!(bmi, get_minus, s);
branch_inst!(bpl, get_minus, c);
branch_inst!(bvc, get_overflow, c);
branch_inst!(bvs, get_overflow, s);

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
}

pub use super::alu::adc;
