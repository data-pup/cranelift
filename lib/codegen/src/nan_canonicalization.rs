//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{Function, Inst, InstBuilder, InstructionData, Opcode, Value};
use timing;

/// Performs the NaN-canonicalization pass by identifying floating-point
/// arithmetic operations, and adding instructions to replace the result
/// with a canonical NaN value if the result of the operation was NaN.
pub fn do_nan_canonicalization(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
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
    // FIXUP: Should `ceil`, `floor`, `trunc`, `nearest`, and
    // immediate constants be considered as well?
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
    // ----------------------------------------------------------------
    // TODO:
    // Let x be the result to some floating point arithmetic operation.
    // Add the following instructions after `inst` : (Pseudo-code)
    // let is_nan = x != x;                          (fcmp)
    // let canonical_res = is_nan ? CANON_VALUE : x  (select)
    // ----------------------------------------------------------------
    // FIXUP: Verbose type annotations are here purely for the sake of
    // helping myself learn more about the Cretonne API.

    let orig_pos: Inst = inst; // Store the original position of the cursor.

    // Select the operation's result.
    let inst_res: Value = pos.func.dfg.first_result(orig_pos);

    // Move to the next instruction. (FIXUP: Is this completely safe to unwrap?)
    let next_inst: Inst = pos.next_inst().unwrap();

    // Insert a comparison to check if the result of the instruction was NaN.
    let is_nan: Value = pos.ins().ffcmp(inst_res, inst_res);

    // Select a canonical value if the result was NaN, or the original result otherwise.
    // FIXUP: How/Where to define the constant canonical value?
    let new_res: Value = pos.ins().select(is_nan, inst_res, inst_res);

    // Move backwards to the last instruction we inserted, so that we can
    // replace the results of the original instruction with aliases to the
    // results of the select instruction.
    let select_inst: Inst = pos.prev_inst().unwrap();

    // Replace the results of the original floating point arithmetic operation
    // with aliases to the results of the new instruction.
    // FIXUP: Is this backwards? I'd like to double check this.
    // FIXUP: Comments for `replace_with_aliases` mention that `dest_inst` may
    // need to be removed from the graph. Does this apply in this case?
    // pos.func.dfg.replace_with_aliases(orig_pos, select_inst);
}
