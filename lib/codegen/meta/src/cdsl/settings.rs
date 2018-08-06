//! Classes for describing settings and groups of settings.

use std::fmt;

/// A named setting variable that can be configured externally to Cranelift.
///
/// Settings are normally not named when they are created. They get their name
/// from the `extract_names` method.
#[derive(Debug)]
pub enum Setting {}

impl Setting {
    /// Get the name of this setting.
    fn _name(&self) -> String {
        unimplemented!();
    }

    fn _group(&self) -> _SettingGroup {
        unimplemented!();
    }
}

impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

/// A group of settings.
///
/// Whenever a :class:`Setting` object is created, it is added to the currently
/// open group. A setting group must be closed explicitly before another can be
/// opened.
pub struct _SettingGroup;

impl _SettingGroup {
    /// Get a short mnemonic name for setting group.
    fn _name(&self) -> String {
        unimplemented!();
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
}
