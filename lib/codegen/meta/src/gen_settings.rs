//! Generate sources with settings.

use cdsl;
use error;
use srcgen;

// TODO: Function checklist w/ original signatures.
// ----------------------------------------------------------------
// [ ] - def gen_to_and_from_str(ty, values, fmt):
// # type: (str, Tuple[str, ...], srcgen.Formatter) -> None
//
// [ ] - def gen_enum_types(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_getter(setting, sgrp, fmt):
// # type: (Setting, SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_pred_getter(name, pred, sgrp, fmt):
// # type: (str, Predicate, SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_getters(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_descriptors(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_template(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// [ ] - def gen_display(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
//
// [.] - def gen_constructor(sgrp, parent, fmt):
// # type: (SettingGroup, PredContext, srcgen.Formatter) -> None
//
// [x] - def gen_group(sgrp, fmt):
// # type: (SettingGroup, srcgen.Formatter) -> None
// ----------------------------------------------------------------

/// Generate a Flags constructor.
fn _gen_constructor(
    _sgrp: &cdsl::settings::_SettingGroup,
    _pred: (),
    _fmt: &mut srcgen::Formatter
) {
    {
        let impl_scope = _fmt._indented(Some("impl Flags {"), Some("}"));
        let args = "builder: Builder";

        // TODO: Find the prefix of the name.
        // if let Some(parent) = _sgrp.parent() {
        //     unimplemented!();
        // }
    }

    unimplemented!();
}

/// Generate a Flags struct representing `sgrp`.
fn gen_group(sgrp: &cdsl::settings::_SettingGroup, fmt: &mut srcgen::Formatter) {
    fmt.line("#[derive(Clone)]");
    fmt.doc_comment(&format!("Flags group `{}`", sgrp.name()));
    {
        let scope = fmt._indented(Some("pub struct Flags {"), Some("}"));
        scope.fmt.line(&format!("bytes: [u8; {}]", sgrp.byte_size()))
    }

    // TODO: Implement predicate contexts.
    _gen_constructor(sgrp, (), fmt);

    // TODO:
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

    // Original Python: [DEVELOPMENT NOTE - FIXUP]
    // ----------------------------------------------------------------------
    // settings.group.qual_mod = 'settings'
    // gen_group(settings.group, fmt)
    // ----------------------------------------------------------------------
    let _settings = cdsl::settings::_SettingGroup::new("shared", Some("settings"));
    gen_group(&_settings, &mut _fmt);

    // Update the file, and return a success. (This likely remains the same.)
    _fmt.update_file(_filename, _out_dir)?;
    Ok(())

    // unimplemented!(); // For now, panic, because this is unimplemented.
}
