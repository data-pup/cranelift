//! Generate sources with settings.

use cdsl;
use error;
use srcgen;

// Original Python functions and signatures.
// ----------------------------------------------------------------
// def gen_to_and_from_str(ty, values, fmt):
// # type: (str, Tuple[str, ...], srcgen.Formatter) -> None
//
// def gen_enum_types(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// def gen_getter(setting, sgrp, fmt):
// # type: (Setting, SettingGroup, srcgen.Formatter) -> None
//
// def gen_pred_getter(name, pred, sgrp, fmt):
// # type: (str, Predicate, SettingGroup, srcgen.Formatter) -> None
//
// def gen_getters(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// def gen_descriptors(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// def gen_template(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// def gen_display(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// def gen_constructor(sgrp, parent, fmt):
// # type: (SettingGroup, PredContext, srcgen.Formatter) -> None
//
// def gen_group(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
// ----------------------------------------------------------------

/// Generate a Flags struct representing `sgrp`.
fn gen_group(sgrp: &cdsl::settings::_SettingGroup, fmt: &mut srcgen::Formatter) {
    fmt.line("#[derive(Clone)]");
    fmt.doc_comment(&format!("Flags group `{}`", sgrp.name()));
    {
        let scope = fmt._indented(Some("pub struct Flags {"), Some("}"));
        scope.fmt.line(&format!("bytes: [u8; {}]", sgrp.byte_size()))
    }

    // TODO:
    // gen_constructor(sgrp, None, fmt)
    // gen_enum_types(sgrp, fmt)
    // gen_getters(sgrp, fmt)
    // gen_descriptors(sgrp, fmt)
    // gen_template(sgrp, fmt)
    // gen_display(sgrp, fmt)

    // unimplemented!(); // FIXUP: Temporarily disable.
}

/// Generate shared settings.
pub fn generate(_filename: &str, _out_dir: &str) -> Result<(), error::Error> {
    let mut _fmt = srcgen::Formatter::new();

    // Original Python:
    // ----------------------------------------------------------------------
    // settings.group.qual_mod = 'settings'
    // gen_group(settings.group, fmt)
    // ----------------------------------------------------------------------
    // Rust Equivalent:
    // ----------------------------------------------------------------------
    let _settings = cdsl::settings::_SettingGroup::new("shared", Some("settings"));
    gen_group(&_settings, &mut _fmt);
    // ----------------------------------------------------------------------

    // Proposed Rust (?) This is -very- much subject to change.
    // let mut group = settings::group::new();
    // group.set_qual_mod("settings");
    // gen_group(_group, fmt);

    // Update the file, and return a success. (This likely remains the same.)
    _fmt.update_file(_filename, _out_dir)?;
    Ok(())

    // unimplemented!(); // For now, panic, because this is unimplemented.
}
