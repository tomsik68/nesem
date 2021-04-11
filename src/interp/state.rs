/// Holds state of a 6502 interpreter
pub struct State {
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    /// 8-bit offset to the stack page
    // TODO should be initialized to 0xFF as stack starts at STACK_OFFSET + 0xFF,
    // decremented with push
    pub sp: u8,
    /// Status word
    /// Starting from 8th bit: `NV1BDIZC`
    /// for Ricoh CPU in the NES, there is no need to support D
    pub psw: u8,
    pub accumulator: u8,
    /// Indexing register
    pub x: u8,
    /// Indexing register
    pub y: u8,

    /// Content of ram
    ram: [u8; 0x800],
    /// Content of ppu registers
    ppu_registers: [u8; 0x8],
    /// Content of apu input
    apu_input: [u8; 0x18],
}

const PSW_CARRY_BIT: u8 = 1 << 0;
const PSW_ZERO_BIT: u8 = 1 << 1;
const PSW_INTERRUPT_BIT: u8 = 1 << 2;
const PSW_DECIMAL_BIT: u8 = 1 << 3;
const PSW_BREAK_BIT: u8 = 1 << 4;
const PSW_ONE_BIT: u8 = 1 << 5;
const PSW_OVERFLOW_BIT: u8 = 1 << 6;
const PSW_NEGATIVE_BIT: u8 = 1 << 7;

const STACK_OFFSET: u16 = 0x100;

/// generate getter and setter for a given psw bit in state
macro_rules! psw_getset {
    ($getter:ident, $setter:ident, $mask:expr) => {
        pub fn $getter(&self) -> bool {
            self.psw & $mask > 0
        }
        pub fn $setter(&mut self, v: bool) {
            self.psw &= !$mask;
            if v {
                self.psw |= $mask;
            }
        }
    };
}

impl State {
    /// create a new state with no guarantees on the setting of registers and content of ram
    /// mainly intended for testing and situations where any required properties will be
    /// set externally
    pub fn new_undefined() -> State {
        State {
            pc: 0,
            sp: 0,
            // this bit is always one
            psw: PSW_ONE_BIT,
            accumulator: 0,
            x: 0,
            y: 0,
            ram: [0; 0x800],
            ppu_registers: [0; 0x8],
            apu_input: [0; 0x18],
        }
    }

    pub fn ram_get(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn ram_set(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    /// return stack pointer
    /// the address where to store newly-pushed element of stack
    fn get_sp(&self) -> u16 {
        STACK_OFFSET + self.pc
    }

    pub fn stack_push(&mut self, val: u8) {
        self.ram_set(self.get_sp(), val);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.ram_get(self.get_sp())
    }

    pub fn push_pc(&mut self) {
        self.stack_push(self.pc as u8);
        self.stack_push((self.pc >> 8) as u8);
    }

    pub fn pop_pc(&mut self) {
        self.pc = 0;
        self.pc = ((self.stack_pop() as u16) << 8);
        self.pc |= self.stack_pop() as u16;
    }

    psw_getset!(get_carry, set_carry, PSW_CARRY_BIT);
    psw_getset!(get_zero, set_zero, PSW_ZERO_BIT);
    psw_getset!(get_interrupt, set_interrupt, PSW_INTERRUPT_BIT);
    // we don't support PSW_DECIMAL_BIT
    psw_getset!(get_break, set_break, PSW_BREAK_BIT);
    // get/set for PSW_ONE_BIT is useless
    psw_getset!(get_overflow, set_overflow, PSW_OVERFLOW_BIT);
    psw_getset!(get_negative, set_negative, PSW_NEGATIVE_BIT);
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn test_psw() {
        let mut st = State::new_undefined();
        // zero out everything initially
        st.set_carry(false);
        st.set_zero(false);
        st.set_interrupt(false);
        st.set_break(false);
        st.set_overflow(false);
        st.set_negative(false);
        assert_eq!(false, st.get_carry());
        assert_eq!(false, st.get_zero());
        assert_eq!(false, st.get_interrupt());
        assert_eq!(false, st.get_break());
        assert_eq!(false, st.get_overflow());
        assert_eq!(false, st.get_negative());
        st.set_carry(true);
        assert_eq!(true, st.get_carry());
        assert_eq!(false, st.get_zero());
        assert_eq!(false, st.get_interrupt());
        assert_eq!(false, st.get_break());
        assert_eq!(false, st.get_overflow());
        assert_eq!(false, st.get_negative());
        st.set_overflow(true);
        assert_eq!(true, st.get_carry());
        assert_eq!(false, st.get_zero());
        assert_eq!(false, st.get_interrupt());
        assert_eq!(false, st.get_break());
        assert_eq!(true, st.get_overflow());
        assert_eq!(false, st.get_negative());
        st.set_carry(false);
        assert_eq!(false, st.get_carry());
        assert_eq!(false, st.get_zero());
        assert_eq!(false, st.get_interrupt());
        assert_eq!(false, st.get_break());
        assert_eq!(true, st.get_overflow());
        assert_eq!(false, st.get_negative());
    }
}
