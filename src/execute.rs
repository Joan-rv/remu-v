use crate::instruction::Instruction;
use crate::state::State;

pub fn execute(instruction: Instruction, state: &mut State) {
    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            let v = state.get(rs1) + state.get(rs2);
            state.set(rd, v);
        }
        Instruction::Addi { rd, rs1, imm } => {
            let v = state.get(rs1) + imm as u32;
            state.set(rd, v);
        }
    }
}
