use std::env::current_dir;
use std::path::PathBuf;

use diagnostic_quick::QResult;

use flatten_files::FlattenFlies;

#[derive(Debug, Copy, Clone)]
pub struct FsFlatten {
    workspace: Option<String>,
}


impl FsFlatten {
    pub fn run(&self) -> QResult {}

    fn workspace(&self) {
        let dir = match &self.workspace {
            Some(s) => {
                PathBuf::from(s)
            }
            None => {
                current_dir()?
            }
        };
        FlattenFlies::new(dir).run(dir)
    }
}