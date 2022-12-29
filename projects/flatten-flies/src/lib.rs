use std::path::{Path, PathBuf};

mod run;

pub struct FlattenFlies {
    pub output: PathBuf,
    pub delete_empty: bool,
    pub pattern: String,
}

impl FlattenFlies {
    #[inline]
    pub fn new<P: AsRef<Path>>(output: P) -> Self {
        Self {
            output: output.as_ref().to_path_buf(),
            delete_empty: false,
            pattern: "*".to_string(),
        }
    }
}
