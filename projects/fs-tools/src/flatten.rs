use std::env::current_dir;
use std::path::PathBuf;
use diagnostic_quick::error_3rd::{GlobSet, GlobSetBuilder};

use diagnostic_quick::QResult;

use fs_flatten::FlattenFlies;

use crate::utils::ensure_workspace;

#[derive(Debug, Clone)]
pub struct FsFlatten {
    workspace: Option<String>,
    delete_empty: bool,
    include: Option<String>,
}


impl FsFlatten {
    pub fn run(&self) -> QResult {}
    fn worker(&self) -> QResult<FlattenFlies> {
        let dir = match &self.workspace {
            Some(s) => {
                PathBuf::from(s)
            }
            None => {
                current_dir()?
            }
        };
        Ok(FlattenFlies {
            output: Default::default(),
            pattern: self.read_pattern()?,
            delete_empty: false,
        })
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
}