use std::env::current_dir;
use std::path::{Path, PathBuf};

use diagnostic_quick::{QError, QResult};
use diagnostic_quick::error_3rd::{Glob, GlobSet, GlobSetBuilder};

pub fn ensure_workspace<P: AsRef<Path>>(path: P) -> QResult<PathBuf> {
    let path = path.as_ref();
    if !path.is_dir() {
        Err(QError::runtime_error(format!("workspace must a dir")))?;
    }
    Ok(path.canonicalize()?)
}

pub fn resolve_workspace(path: &Option<String>) -> QResult<PathBuf> {
    let path = match path {
        None => { current_dir()? }
        Some(s) => { PathBuf::from(s) }
    };
    ensure_workspace(path)
}


pub fn build_glob_set(include: &Option<String>) -> QResult<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    match include {
        None => {
            builder.add(Glob::new("*")?);
        }
        Some(s) => {
            for line in s.trim().lines() {
                builder.add(Glob::new(line)?);
            }
        }
    }
    Ok(builder.build()?)
}