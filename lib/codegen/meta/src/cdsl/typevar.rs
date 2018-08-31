//! Type variables for Parametric polymorphism.
//!
//! Cranelift instructions and instruction transformations can be specified to be
//! polymorphic by using type variables.

static _MAX_LANES: u32 = 256;
static _MAX_BITS: u32 = 64;
static _MAX_BITVEC: u32 = 16_384;

pub struct _TypeVar;
