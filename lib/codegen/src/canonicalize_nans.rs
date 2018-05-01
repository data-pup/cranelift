//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::{Function, Opcode};
use timing;

// Canonical NaN values.

// TODO: Helper functions should go here.

/// Returns true/false based on whether the instruction is a floating-point
/// arithmetic operation.
fn is_fp_arith(opcode: Opcode) -> bool {
    unimplemented!()
}

/// The main NaN-canonicalization pass.
pub fn _do_canonicalize_nans(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(inst) = pos.next_inst() {
            // TODO: NaN Canonicalization should go here.

            let data = &pos.func.dfg[inst];
            let opcode = data.opcode();
            if is_fp_arith(opcode) {
                unimplemented!();
            }

        }
    }
}
