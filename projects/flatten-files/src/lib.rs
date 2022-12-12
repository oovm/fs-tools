use std::path::{Path, PathBuf};

use diagnostic_quick::{QError, QResult};

mod run;

pub struct FlattenFlies {
    pub output: PathBuf,
    pub pattern: String,
    pub delete_empty: bool,
}

impl FlattenFlies {
    #[inline]
    pub fn new<P: AsRef<Path>>(output: P) -> QResult<Self> {
        let path = output.as_ref();
        if !path.is_dir() {
            Err(QError::runtime_error(format!("workspace must a dir")))?;
        }
        Ok(Self {
            output: path.canonicalize()?,
            pattern: "*".to_string(),
            delete_empty: false,
        })
    }
}

pub fn is_empty_directory(path: &Path) -> bool {
    match path.read_dir() {
        Ok(mut o) => {
            o.next().is_none()
        }
        Err(_) => { false }
    }
}