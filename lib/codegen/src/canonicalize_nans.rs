//! A NaN-canonicalizing rewriting pass.

use cursor::{Cursor, FuncCursor};
use ir::Function;
use timing;

// Canonical NaN values.


// TODO: Helper functions should go here.

/// The main NaN-canonicalization pass.
pub fn _do_canonicalize_nans(func: &mut Function) {
    let _tt = timing::canonicalize_nans();
    let mut pos = FuncCursor::new(func);
    while let Some(_ebb) = pos.next_ebb() {
        while let Some(_inst) = pos.next_inst() {
            // TODO: NaN Canonicalization should go here.
            unimplemented!();
        }
    }
}
