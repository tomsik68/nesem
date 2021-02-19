use super::instruction_type::InstructionType;
use super::operand::Operand;

pub struct Instruction {
    ty: InstructionType,
    operand: Option<Operand>,
}

impl Instruction {
    pub fn without_operand(ty: InstructionType) -> Instruction {
        Instruction { ty, operand: None }
    }

    pub fn with_operand(ty: InstructionType, op: Operand) -> Instruction {
        Instruction {
            ty,
            operand: Some(op),
        }
    }

    pub fn get_type(&self) -> InstructionType {
        self.ty
    }

    pub fn get_operand(&self) -> &Option<Operand> {
        &self.operand
    }
}
