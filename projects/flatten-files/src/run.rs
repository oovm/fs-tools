use std::fs;
use std::path::Path;

use diagnostic_quick::error_3rd::WalkDir;
use diagnostic_quick::QResult;

use crate::FlattenFlies;

impl FlattenFlies {
    pub fn run<P>(&self, input: P) -> QResult where P: AsRef<Path> {
        let path = input.as_ref();
        match try_run(self, path, true) {
            Ok(_) => {}
            Err(e) => {
                Err(e)?
            }
        }
        Ok(())
    }
    pub fn dry_run<P>(&self, input: P) -> QResult where P: AsRef<Path> {
        let path = input.as_ref();
        match try_run(self, path, false) {
            Ok(_) => {}
            Err(e) => {
                Err(e)?
            }
        }
        Ok(())
    }
}

fn try_run(cfg: &FlattenFlies, path: &Path, execute: bool) -> QResult {
    let target = cfg.output.as_path();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_file() {
            let source = entry.path();
            let target = target.join(source.file_name().unwrap());
            println!("{} -> {}", entry.path().display(), target.display());
            if execute {
                fs::rename(source, target)?;
            }
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