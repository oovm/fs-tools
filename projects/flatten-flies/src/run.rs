use std::path::Path;

use diagnostic_quick::{FileID, QResult};
use diagnostic_quick::error_3rd::WalkDir;

use crate::FlattenFlies;

impl FlattenFlies {
    pub fn run<P>(&self, input: P) -> QResult where P: AsRef<Path> {
        let path = input.as_ref();
        match try_run(self, path) {
            Ok(_) => {}
            Err(e) => {
                Err(e.with_file(&FileID::from(path)))
            }
        }
        Ok(())
    }
}

fn try_run(cfg: &FlattenFlies, path: &Path) -> QResult {
    for entry in WalkDir::new(path) {
        let entry = entry?.path();
        if entry.is_file() {
            println!("file: {} -> {}", cfg.output.display(), entry.display());
        }
    }
    Ok(())
}