//! Cretonne ValueType hierarchy

// Temporary disabled: Unused at the moment.
// use std::collections::HashMap;

use base::types as base_types;

static _RUST_NAME_PREFIX: &'static str = "ir::types::";

// ValueType variants (i8, i32, ...) are provided in `base::types.rs`.

/// A concrete SSA value type.
///
/// All SSA values have a type that is described by an instance of `ValueType`
/// or one of its subclasses.
pub enum ValueType {
    BV(BVType),
    Lane(LaneType),
    Special(SpecialType),
    Vector(VectorType),
}

impl ValueType {
    /// Iterate through all of the special types (neither lanes nor vectors).
    pub fn all_special_types() -> SpecialTypeIterator {
        SpecialTypeIterator::new()
    }

    /// Iterate through all of the lane types.
    pub fn all_lane_types() -> LaneTypeIterator {
        LaneTypeIterator::new()
    }

    /// Get the name of this type.
    pub fn name(&self) -> String {
        match self {
            ValueType::BV(_) => unimplemented!(),
            ValueType::Lane(l) => l.name(),
            ValueType::Special(s) => s.name(),
            ValueType::Vector(v) => v.name(),
        }
    }

    /// Return a string containing the documentation comment for this type.
    pub fn doc(&self) -> String {
        match self {
            ValueType::BV(_) => unimplemented!(),
            ValueType::Lane(l) => l.doc(),
            ValueType::Special(s) => s.doc(),
            ValueType::Vector(v) => v.doc(),
        }
    }

    /// Find the unique number associated with this type.
    pub fn number(&self) -> u8 {
        match self {
            ValueType::BV(_) => unimplemented!(),
            ValueType::Lane(l) => l.number(),
            ValueType::Special(s) => s.number(),
            ValueType::Vector(v) => v.number(),
        }
    }

    /// Return the name of this type for other Rust source files.
    pub fn _rust_name(&self) -> String {
        format!("{}{}", _RUST_NAME_PREFIX, self.name().to_uppercase())
    }
}

impl From<BVType> for ValueType {
    fn from(bv: BVType) -> Self {
        ValueType::BV(bv)
    }
}

impl From<LaneType> for ValueType {
    fn from(lane: LaneType) -> Self {
        ValueType::Lane(lane)
    }
}

impl From<SpecialType> for ValueType {
    fn from(spec: SpecialType) -> Self {
        ValueType::Special(spec)
    }
}

impl From<VectorType> for ValueType {
    fn from(vector: VectorType) -> Self {
        ValueType::Vector(vector)
    }
}

/// A concrete scalar type that can appear as a vector lane too.
#[derive(Debug, Clone, Copy)]
pub struct LaneType {
    bits: u64,
    tag: LaneTypeTag,
}

impl LaneType {
    /// Get the name of this type.
    pub fn name(&self) -> String {
        self.tag.name()
    }

    pub fn doc(&self) -> String {
        match self.tag {
            LaneTypeTag::BoolType(_) => format!("A boolean type with {} bits.", self.bits),
            LaneTypeTag::IntType(_) if self.bits < 32 => format!(
                "An integer type with {} bits.
                WARNING: arithmetic on {}bit integers is incomplete.",
                self.bits, self.bits
            ),
            LaneTypeTag::IntType(_) => format!("An integer type with {} bits.", self.bits),
            LaneTypeTag::FloatType(base_types::Float::F32) => String::from(
                "A 32-bit floating point type represented in the IEEE 754-2008
                *binary32* interchange format. This corresponds to the :c:type:`float`
                type in most C implementations.",
            ),
            LaneTypeTag::FloatType(base_types::Float::F64) => String::from(
                "A 64-bit floating point type represented in the IEEE 754-2008
                *binary64* interchange format. This corresponds to the :c:type:`double`
                type in most C implementations.",
            ),
        }
    }

    pub fn number(&self) -> u8 {
        self.tag.number()
    }

    /// Find the number of bytes that this type occupies in memory.
    pub fn membytes(&self) -> u64 {
        self._lane_bits() / 8
    }

    /// Return the number of bits in a lane.
    fn _lane_count(&self) -> u64 {
        1
    }

    /// Return the number of bits in a lane.
    pub fn _lane_bits(&self) -> u64 {
        self.bits
    }

    /// Return the total number of bits of an instance of this type.
    fn _width(&self) -> u64 {
        self._lane_count() * self._lane_bits()
    }

    /// Return true iff:
    ///     1. self and other have equal number of lanes
    ///     2. each lane in self has at least as many bits as a lane in other
    fn _wider_or_equal(&self, rhs: &LaneType) -> bool {
        (self._lane_count() == rhs._lane_count()) && (self._lane_bits() >= rhs._lane_bits())
    }
}

/// The kinds of elements in a lane.
#[derive(Debug, Clone, Copy)]
pub enum LaneTypeTag {
    BoolType(base_types::Bool),
    IntType(base_types::Int),
    FloatType(base_types::Float),
}

impl LaneTypeTag {
    /// Get the name of a lane type.
    fn name(self) -> String {
        match self {
            LaneTypeTag::BoolType(b) => format!("{}", b),
            LaneTypeTag::IntType(i) => format!("{}", i),
            LaneTypeTag::FloatType(f) => format!("{}", f),
        }
    }

    fn number(self) -> u8 {
        match self {
            LaneTypeTag::BoolType(b) => b.number(),
            LaneTypeTag::IntType(i) => i.number(),
            LaneTypeTag::FloatType(f) => f.number(),
        }
    }
}

pub struct LaneTypeIterator {
    bool_iter: base_types::BoolIterator,
    int_iter: base_types::IntIterator,
    float_iter: base_types::FloatIterator,
}

impl LaneTypeIterator {
    fn new() -> Self {
        Self {
            bool_iter: base_types::BoolIterator::new(),
            int_iter: base_types::IntIterator::new(),
            float_iter: base_types::FloatIterator::new(),
        }
    }
}

impl Iterator for LaneTypeIterator {
    type Item = LaneType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.bool_iter.next() {
            let next = LaneType {
                bits: b as u64,
                tag: LaneTypeTag::BoolType(b),
            };
            Some(next)
        } else if let Some(i) = self.int_iter.next() {
            let next = LaneType {
                bits: i as u64,
                tag: LaneTypeTag::IntType(i),
            };
            Some(next)
        } else if let Some(f) = self.float_iter.next() {
            let next = LaneType {
                bits: f as u64,
                tag: LaneTypeTag::FloatType(f),
            };
            Some(next)
        } else {
            None
        }
    }
}

/// A concrete SIMD vector type.
///
/// A vector type has a lane type which is an instance of `LaneType`,
/// and a positive number of lanes.
pub struct VectorType {
    base: LaneType,
    lanes: u64,
}

impl VectorType {
    /// Initialize a new integer type with `n` bits.
    pub fn new(base: LaneType, lanes: u64) -> VectorType {
        VectorType { base, lanes }
    }

    /// Get the name of this type.
    pub fn name(&self) -> String {
        format!("{}X{}", self.base.name(), self.lanes,)
    }

    pub fn doc(&self) -> String {
        format!(
            "A SIMD vector with {} lanes containing a '{}' each.",
            self.lanes,
            self.base.name()
        )
    }

    /// Find the unique number associated with this type.
    pub fn number(&self) -> u8 {
        let b = f64::from(self.base.number());
        let l = (self.lanes as f64).log2();
        let num = 16_f64 * l + b;
        num as u8
    }

    /// Return the number of lanes.
    pub fn _lane_count(&self) -> u64 {
        self.lanes
    }

    /// Return the number of bits in a lane.
    pub fn _lane_bits(&self) -> u64 {
        self.base._lane_bits()
    }
}

/// A flat bitvector type. Used for semantics description only.
pub struct BVType;

impl BVType {
    /// Initialize a new bitvector type with `n` bits.
    pub fn _new() -> Self {
        Self {}
    }
}

/// A concrete scalar type that is neither a vector nor a lane type.
///
/// Special types cannot be used to form vectors.
pub struct SpecialType {
    tag: SpecialTypeTag,
}

impl SpecialType {
    pub fn name(&self) -> String {
        self.tag.name()
    }

    pub fn doc(&self) -> String {
        match self.tag {
            SpecialTypeTag::Flag(base_types::Flag::IFlags) => String::from(
                "CPU flags representing the result of an integer comparison. These flags
                can be tested with an :type:`intcc` condition code.",
            ),
            SpecialTypeTag::Flag(base_types::Flag::FFlags) => String::from(
                "CPU flags representing the result of a floating point comparison. These flags
                can be tested with a :type:`floatcc` condition code.",
            ),
        }
    }

    pub fn number(&self) -> u8 {
        self.tag.number()
    }
}

pub enum SpecialTypeTag {
    Flag(base_types::Flag),
}

impl SpecialTypeTag {
    pub fn name(&self) -> String {
        match self {
            SpecialTypeTag::Flag(f) => format!("{}", f),
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            SpecialTypeTag::Flag(f) => f.number(),
        }
    }
}

pub struct SpecialTypeIterator {
    flag_iter: base_types::FlagIterator,
}

impl SpecialTypeIterator {
    fn new() -> Self {
        Self {
            flag_iter: base_types::FlagIterator::new(),
        }
    }
}

impl Iterator for SpecialTypeIterator {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.flag_iter.next() {
            let next = SpecialType {
                tag: SpecialTypeTag::Flag(f),
            };
            Some(ValueType::Special(next))
        } else {
            None
        }
    }
}
