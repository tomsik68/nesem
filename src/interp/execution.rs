use super::alu::is_negative;
use super::operand_decoder;
use super::operand_decoder::{get_pointer, get_u8};
use crate::instruction::operand::Operand;
use crate::interp::state::State;

/// Create a function @name which checks the flag @flag.
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

// TODO test this after MMU is done
// since interrupt vector at 0xFFFE is outside ram, it doesn't work without MMU
fn brk(state: &mut State, op: &Operand) {
    match op {
        Operand::Implicit => {}
        _ => panic!("brk: there must be no operand!"),
    };

    // push lower bits then higher bits
    // TODO the stack order
    state.push_pc();
    state.stack_push(state.psw);
    state.set_break(true);
    state.pc = get_pointer(&Operand::Indirect(0xFFFE), &state).unwrap();
}

fn rti(state: &mut State, op: &Operand) {
    match op {
        Operand::Implicit => {}
        _ => panic!("brk: there must be no operand!"),
    };

    // TODO the stack order
    // pop psw
    state.psw = state.stack_pop();
    // pop pc
    state.pop_pc();
}

macro_rules! flag {
    ($clear:ident, $setter:ident) => {
        fn $clear(state: &mut State, op: &Operand) {
            state.$setter(false);
        }
    };

    ($clear:ident, $set:ident, $setter:ident) => {
        fn $clear(state: &mut State, op: &Operand) {
            state.$setter(false);
        }

        fn $set(state: &mut State, op: &Operand) {
            state.$setter(true);
        }
    };
}

flag!(clc, sec, set_carry);
flag!(cli, sei, set_interrupt);
flag!(clv, set_interrupt);

fn jmp(state: &mut State, op: &Operand) {
    let d = get_pointer(&op, &state).expect("jmp: operand is required");
    state.pc = d;
}

fn jsr(state: &mut State, op: &Operand) {
    let d = get_pointer(&op, &state).expect("jsr: operand is required");
    state.push_pc();
    state.pc = d;
}

macro_rules! load {
    ($inst:ident, $dst:ident) => {
        fn $inst(state: &mut State, op: &Operand) {
            let v = get_u8(&op, &state).expect("lda: operand is required");
            state.$dst = v;
            state.set_zero(v == 0);
            state.set_negative(is_negative(v));
        }
    };
}

load!(lda, accumulator);
load!(ldx, x);
load!(ldy, y);

fn nop(_state: &mut State, _op: &Operand) {}

fn pha(state: &mut State, _op: &Operand) {
    state.stack_push(state.accumulator);
}

fn php(state: &mut State, _op: &Operand) {
    state.stack_push(state.psw);
}

fn pla(state: &mut State, _op: &Operand) {
    state.accumulator = state.stack_pop();
    state.set_zero(state.accumulator == 0);
    state.set_negative(is_negative(state.accumulator));
}

fn plp(state: &mut State, _op: &Operand) {
    state.psw = state.stack_pop();
}

fn rts(state: &mut State, op: &Operand) {
    // complement of jsr
    state.pop_pc();
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

    // mod brk {
    //     use crate::instruction::operand::Operand;
    //     use crate::interp::execution::brk;
    //     use crate::interp::state::State;

    //     #[test]
    //     fn brk_test() {
    //         let mut state = State::new_undefined();
    //         state.set_break(false);
    //         state.set_overflow(true);
    //         state.set_negative(true);
    //         state.set_carry(true);

    //         let op = Operand::Implicit;
    //         brk(&mut state, &op);

    //         assert!(state.get_break());
    //     }
    // }
}

pub use super::alu::adc;
