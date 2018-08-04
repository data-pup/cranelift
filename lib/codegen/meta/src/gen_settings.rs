//! Generate sources with settings.

// Disabled import for now.
// use base::settings;

use error;
use srcgen;

/// Generate shared settings.
pub fn generate(_filename: &str, _out_dir: &str) -> Result<(), error::Error> {
    let mut _fmt = srcgen::Formatter::new();

    // Original Python:
    // settings.group.qual_mod = 'settings'
    // gen_group(settings.group, fmt)

    // Proposed Rust (?) This is -very- much subject to change.
    // let mut group = settings::group::new();
    // group.set_qual_mod("settings");
    // gen_group(_group, fmt);

    // Update the file, and return a success. (This likely remains the same.)
    // fmt.update_file(filename, out_dir)?;
    // Ok(())

    unimplemented!(); // For now, panic, because this is unimplemented.
}
