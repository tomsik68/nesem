use super::instruction_type::InstructionType;
use super::operand::Operand;

pub struct Instruction {
    ty: InstructionType,
    operand: Operand,
}

impl Instruction {
    pub fn without_operand(ty: InstructionType) -> Instruction {
        Instruction {
            ty,
            operand: Operand::Implicit,
        }
    }

    pub fn with_operand(ty: InstructionType, operand: Operand) -> Instruction {
        Instruction { ty, operand }
    }

    pub fn get_type(&self) -> InstructionType {
        self.ty
    }

    pub fn get_operand(&self) -> &Operand {
        &self.operand
    }
}
