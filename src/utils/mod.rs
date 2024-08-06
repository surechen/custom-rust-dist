//! Utility functions/types to use across the whole crate.
//!
//! NOTE: Most of these are moved from the `experimental` branch,
//! some of them might turns out to be unused, so remember to clean those after version `1.0`.

mod download;
mod extraction;
mod file_system;
mod process;
mod progress_bar;
mod triple;

use std::path::Path;

pub use download::download_from_start;
pub use extraction::{Extractable, ExtractableKind};
pub use file_system::*;
pub use process::*;
pub use triple::HostTriple;

use anyhow::{Context, Result};
use url::Url;

pub fn parse_url(url: &str) -> Result<Url> {
    Url::parse(url).with_context(|| format!("failed to parse url: {url}"))
}

/// Flip `Option<Result<T, E>>` to `Result<Option<T>, E>`
pub fn flip_option_result<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

pub fn path_to_str(path: &Path) -> Result<&str> {
    path.to_str().ok_or_else(|| {
        anyhow::anyhow!(
            "path '{}' cannot be convert to str as it may contains invalid unicode characters.",
            path.display()
        )
    })
}
