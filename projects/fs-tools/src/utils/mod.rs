use std::path::{Path, PathBuf};

use diagnostic_quick::{QError, QResult};

pub fn ensure_workspace<P: AsRef<Path>>(path: P) -> QResult<PathBuf> {
    let path = path.as_ref();
    if !path.is_dir() {
        Err(QError::runtime_error(format!("workspace must a dir")))?;
    }
    Ok(path.canonicalize()?)
}

fn read_pattern(&self) -> QResult<GlobSet> {
    let a = GlobSetBuilder::new();
    a.build()?

    match self.include {
        None => {
            return GlobSet::n()
        }
        Some(s) => {s}
    }

}