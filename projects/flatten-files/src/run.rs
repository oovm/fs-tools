use fs::rename;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use diagnostic_quick::{QError, QResult};
use diagnostic_quick::error_3rd::WalkDir;

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
    let target = cfg.output.as_path();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_file() {
            let source = entry.path();
            let target = target.join(source.file_name().unwrap());
            println!("{} -> {}", entry.path().display(), target.display());
            safe_rename(source, &target, cfg.execute)?;
        }
    }
    Ok(())
}

pub fn safe_rename(source: &Path, target: &Path, execute: bool) -> QResult {
    if !target.exists() {
        print_rename(source, target, execute)?;
    }
    let file_name = match target.file_name().and_then(OsStr::to_str) {
        None => {
            Err(QError::runtime_error("target file name is not valid"))?
        }
        Some(s) => {
            s
        }
    };
    let mut new_id = 1;
    while new_id < 65535 {
        let new_name = format!("{} ({})", file_name, new_id);
        let new_target = target.with_file_name(new_name);
        if !new_target.exists() {
            return print_rename(source, &new_target, execute);
        }
        new_id += 1;
    }
    Ok(())
}

fn print_rename(source: &Path, target: &Path, execute: bool) -> QResult {
    println!("{} -> {}", source.display(), target.display());
    if execute {
        rename(source, target)?;
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