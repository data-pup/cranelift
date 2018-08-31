//! Cranelift predicates that consider `Function` fields.

// FIXUP: These will be used eventually.
// use cdsl::predicates::_FieldPredicate;

/// An instruction predicate that checks the referenced function is colocated.
struct _IsColocatedFunc;

/// An instruction predicate that checks the referenced data object is
/// colocated.
struct _IsColocatedData;

/// TODO: Document this predicate?
struct _LengthEquals;
