//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{Function, Inst, InstructionData, Opcode};
use timing;

/// Performs the NaN-canonicalization pass by identifying floating-point
/// arithmetic operations, and adding instructions to replace the result
/// with a canonical NaN value if the result of the operation was NaN.
pub fn _do_canonicalize_nans(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            // Determine if `inst` is a floating-point arithmetic operation.
            if is_fp_arith(&mut pos, inst) {
                add_nan_canon_instrs(&mut pos);
            } else {
                unimplemented!(); // FIXUP: Do nothing if not fp arithmetic?
            }
        }
    }
}

/// Returns true/false based on whether the instruction is a floating-point
/// arithmetic operation.
fn is_fp_arith(pos: &mut FuncCursor, inst: Inst) -> bool {
    match pos.func.dfg[inst] {
        InstructionData::Unary { opcode, .. } => {
            if opcode == Opcode::Sqrt {
                true
            } else {
                false
            }
        },
        InstructionData::Binary { opcode, .. } => {
            if opcode == Opcode::Fadd
            || opcode == Opcode::Fsub
            || opcode == Opcode::Fmul
            || opcode == Opcode::Fdiv
            || opcode == Opcode::Fmin
            || opcode == Opcode::Fmax
            {
                true
            } else {
                false
            }
        },
        InstructionData::Ternary { opcode, .. } => {
            if opcode == Opcode::Fma {
                true
            } else {
                false
            }
        },
        _ => unimplemented!(), // FIXUP: Return false in this case?
    }
}

/// Patch instructions that may result in a NaN result with operations to
/// identify and replace NaN's with a single canonical NaN value.
fn add_nan_canon_instrs(pos: &mut FuncCursor) {
    unimplemented!();
}
