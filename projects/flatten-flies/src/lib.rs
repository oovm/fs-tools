use std::path::{Path, PathBuf};

mod run;

pub struct FlattenFlies {
    pub output: PathBuf,
    pub pattern: String,
    pub delete_empty: bool,
}

impl FlattenFlies {
    #[inline]
    pub fn new<P: AsRef<Path>>(output: P) -> Self {
        debug_assert!(output.as_ref().is_dir());
        Self {
            output: output.as_ref().to_path_buf(),
            pattern: "*".to_string(),
            delete_empty: false,
        }
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