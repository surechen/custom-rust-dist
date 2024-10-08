//! This module contains implementations of core functionalities,
//! each submodule must implement traits defined in [`core`](crate::core).
//! such as [`Installation`](crate::core::Installation).
//!
//! Note: If you add/remove sub mods here to add/remove support for certain OS,
//! make sure to update `build.rs` as well.

#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub(crate) mod windows;

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::utils;

/// Try getting the installation root judging be current executable path.
//
// This program should be installed under `{install_dir}/.cargo/bin/`,
// we should be able to track the installation dir by going up three parents.
// We should also make sure it is indeed the installation dir by checking if
// the folder fits the characteristic.
// FIXME: There might be risks involved, resulting unintended directory being removed
// after uninstallation.
pub(crate) fn install_dir_from_exe_path() -> Result<PathBuf> {
    let exe_path = std::env::current_exe().context("cannot locate current executable")?;
    let comp_count = exe_path.components().count();
    let maybe_install_dir: PathBuf = exe_path
        .components()
        .take(comp_count.saturating_sub(3))
        .collect();

    if maybe_install_dir.parent().is_none() {
        bail!("unable to uninstall as it appears that this program was mistakenly installed in root directory.");
    }
    // Make sure this directory match some signature by checking its content.
    // This is still unsafe, we're desparetely in need of an install manifest.
    let install_dir_contents = utils::walk_dir(&maybe_install_dir, false)
        .context("unable to uninstall as the installation directory cannot be determined.")?;
    if !install_dir_contents.contains(&maybe_install_dir.join(".cargo"))
        || !install_dir_contents.contains(&maybe_install_dir.join(".rustup"))
    {
        bail!(
            "unable to uninstall as the installation directory appears to be corrupted, \
            try manually remove the following directory:\n\n\
            {}\n",
            maybe_install_dir.display()
        );
    }

    Ok(maybe_install_dir)
}

pub(crate) fn add_to_path(path: &Path) -> Result<()> {
    #[cfg(windows)]
    windows::add_to_path(path)?;

    #[cfg(unix)]
    unix::add_to_path(path)?;

    Ok(())
}

pub(crate) fn remove_from_path(path: &Path) -> Result<()> {
    #[cfg(windows)]
    windows::remove_from_path(path)?;

    #[cfg(not(windows))]
    unix::remove_from_path(path)?;

    Ok(())
}
