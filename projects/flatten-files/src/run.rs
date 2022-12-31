use std::path::Path;

use diagnostic_quick::error_3rd::WalkDir;
use diagnostic_quick::QResult;

use crate::FlattenFlies;

impl FlattenFlies {
    pub fn run<P>(&self, input: P) -> QResult where P: AsRef<Path> {
        let path = input.as_ref();
        match try_run(self, path) {
            Ok(_) => {}
            Err(e) => {
                Err(e)?
            }
        }
        Ok(())
    }
}

fn try_run(cfg: &FlattenFlies, path: &Path) -> QResult {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_file() {
            println!("file: {} -> {}", cfg.output.display(), entry.into_path().display());
        }
    }
    Ok(())
}


pub fn is_empty_directory(path: &Path) -> bool {
    match path.read_dir() {
        Ok(mut o) => {
            o.next().is_none()
        }
        Err(_) => { false }
    }
}