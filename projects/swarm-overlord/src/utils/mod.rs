use std::fs::metadata;
use std::path::Path;

use diagnostic_quick::QResult;

#[allow(unused_variables)]
pub fn set_executable(path: &Path) -> QResult<()> {
    let mut permissions = metadata(path)?.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        permissions.set_mode(0o755);
    }
    Ok(())
}