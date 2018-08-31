//! Classes for describing settings and groups of settings.

use std::fmt;

/// A named setting variable that can be configured externally to Cranelift.
///
/// Settings are normally not named when they are created. They get their name
/// from the `extract_names` method.
#[derive(Debug)]
pub enum _Setting {
    _Bool(_BoolSetting),
    _Num(_NumSetting),
    _Enum(_EnumSetting),
}

impl _Setting {
    /// Get the name of this setting.
    fn _name(&self) -> String {
        unimplemented!();
    }

    fn _group(&self) -> _SettingGroup {
        unimplemented!();
    }
}

impl fmt::Display for _Setting {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

/// A named setting with a boolean on/off value.
#[derive(Debug)]
pub struct _BoolSetting;

/// A named setting with an integral value in the range 0--255.
#[derive(Debug)]
pub struct _NumSetting;

/// A named setting with an enumerated set of possible values.
///
/// The default value is always the first enumerator.
#[derive(Debug)]
pub struct _EnumSetting;

/// A group of settings.
///
/// Whenever a :class:`Setting` object is created, it is added to the currently
/// open group. A setting group must be closed explicitly before another can be
/// opened.
pub struct _SettingGroup {
    /// Short mnemonic name for setting group.
    _name: String,
    /// The settings in this group.
    _settings: Vec<_Setting>,
    /// Fully qualified Rust module name. See gen_settings.py.
    _qual_mod: Option<String>,
}

impl _SettingGroup {
    pub fn new(name: &str, qual_mod: Option<&str>) -> Self {
        Self {
            _name: name.to_string(),
            _settings: vec![],
            _qual_mod: qual_mod.map(|s| s.to_string()),
        }
    }

    /// Get a short mnemonic name for setting group.
    pub fn name(&self) -> &str {
        &self._name
    }

    /// Open this setting group such that future new settings are added to this
    /// group.
    fn _open(&self) {
        unimplemented!();
    }

    /// Close this setting group. This function must be called before opening
    /// another setting group.
    fn _close(&self) {
        unimplemented!();
    }

    /// Compute the number of bytes required to hold all settings and
    /// precomputed predicates.
    ///
    /// This is the size of the byte-sized settings plus all the numbered
    /// predcate bits rounded up to a whole number of bytes.
    pub fn byte_size(&self) -> u32 {
        // self.boolean_offset + (len(self.predicate_number) + 7) / 8
        unimplemented!();
    }
}

pub struct _Preset;
