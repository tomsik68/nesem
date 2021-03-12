#![feature(concat_idents)]

mod instruction;
mod interp;

fn main() {
    let ty = instruction::instruction_type::InstructionType::Adc;
}
