use std::path::PathBuf;

use diagnostic_quick::error_3rd::GlobSet;

mod run;

pub struct FlattenFlies {
    pub output: PathBuf,
    pub pattern: GlobSet,
    pub delete_empty: bool,
    pub execute: bool,
    pub overwrite: bool,
}
