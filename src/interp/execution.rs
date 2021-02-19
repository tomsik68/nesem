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

fn bcc(state: &mut State, op: &Operand) {
    /*
     * Questions:
     * FF F0: BCC +64   // where does this take me?
     * 00 00: BCC -64   // where does this take me?
     *
     * Let's assume signed numbers are two-complement and they "wrap around"
     * 00 00: BCC -64 --> 0xFFC0
     */

    let dest = match op {
        Operand::Relative(rel) => state.pc.wrapping_add((*rel as u16).into()),
        _ => unimplemented!("bcc: operand is not Relative(i8)"),
    };

    if !state.get_carry() {
        state.pc = dest;
    }
}

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
