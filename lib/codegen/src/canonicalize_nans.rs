//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{Function, Inst, InstructionData, Opcode, Value};
use timing;

/// Performs the NaN-canonicalization pass by identifying floating-point
/// arithmetic operations, and adding instructions to replace the result
/// with a canonical NaN value if the result of the operation was NaN.
pub fn _do_canonicalize_nans(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    return; // FIXUP (Adding invocation to compilation context).

    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            if is_fp_arith(&mut pos, inst) {
                add_nan_canon_instrs(&mut pos, inst);
            }
        }
    }
}

/// Returns true/false based on whether the instruction is a floating-point
/// arithmetic operation. This ignores operations like `fneg`, `fabs`, or
/// `fcopysign` that only operate on the sign bit of a floating point value.
fn is_fp_arith(pos: &mut FuncCursor, inst: Inst) -> bool {
    match pos.func.dfg[inst] {
        InstructionData::Unary { opcode, .. } => {
            opcode == Opcode::Sqrt
        },
        InstructionData::Binary { opcode, .. } => {
            opcode == Opcode::Fadd
            || opcode == Opcode::Fsub
            || opcode == Opcode::Fmul
            || opcode == Opcode::Fdiv
            || opcode == Opcode::Fmin
            || opcode == Opcode::Fmax
        },
        InstructionData::Ternary { opcode, .. } => {
            opcode == Opcode::Fma
        },
        _ => false,
    }
}

/// Patch instructions that may result in a NaN result with operations to
/// identify and replace NaN's with a single canonical NaN value.
fn add_nan_canon_instrs(pos: &mut FuncCursor, inst: Inst) {
    // TODO:
    // Let x be the result to some floating point arithmetic operation.
    // Add the following instructions after `inst` : (Pseudo-code)
    // let is_nan = x != x;                          (fcmp)
    // let canonical_res = is_nan ? CANON_VALUE : x  (select)
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use canonicalize_nans::*;

    #[test]
    fn is_fp_arith_works() {
        unimplemented!();
    }
}
