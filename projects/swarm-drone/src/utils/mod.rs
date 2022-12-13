use std::fs::{metadata, Permissions, set_permissions};
use std::path::Path;

pub fn check_permissions(path: &Path) -> QResult<Permissions> {
    Ok(metadata(path)?.permissions().mode())
}
