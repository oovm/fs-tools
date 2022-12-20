use fs::{remove_dir_all, rename};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use diagnostic_quick::{error_3rd::WalkDir, QError, QResult};

use crate::FlattenFlies;

impl FlattenFlies {
    pub fn run<P>(&self, input: P) -> QResult
    where
        P: AsRef<Path>,
    {
        let path = input.as_ref();
        match self.try_run(path) {
            Ok(_) => {}
            Err(e) => Err(e)?,
        }
        Ok(())
    }
    fn try_run(&self, path: &Path) -> QResult {
        let target = self.output.as_path();
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.path().is_file() {
                let source = entry.path();
                let target = match source.file_name() {
                    Some(s) => target.join(s),
                    None => {
                        eprintln!("No file name for {}", source.display());
                        continue;
                    }
                };
                if source.eq(&target) {
                    continue;
                }
                self.safe_rename(source, &target)?;
            }
        }
        for entry in WalkDir::new(path) {
            self.debug_remove(entry?.path())?;
        }
        Ok(())
    }
    fn safe_rename(&self, source: &Path, target: &Path) -> QResult {
        if self.overwrite {
            return self.debug_rename(source, target);
        }
        if !target.exists() {
            self.debug_rename(source, target)?;
        }
        let file_name = match target.file_name().and_then(OsStr::to_str) {
            None => Err(QError::runtime_error("target file name is not valid"))?,
            Some(s) => s,
        };
        let mut new_id = 1;
        while new_id < 65535 {
            let new_name = format!("{} ({})", file_name, new_id);
            let new_target = target.with_file_name(new_name);
            if !new_target.exists() {
                return self.debug_rename(source, &new_target);
            }
            new_id += 1;
        }
        Ok(())
    }
    fn debug_rename(&self, source: &Path, target: &Path) -> QResult {
        println!("[mv] {} \n    -> {}", source.display(), target.display());
        if self.execute {
            rename(source, target)?;
        }
        Ok(())
    }
    fn debug_remove(&self, path: &Path) -> QResult {
        if self.delete_empty && is_empty_directory(path) {
            println!("[rm] remove {}", path.display());
            if self.execute {
                remove_dir_all(path)?;
            }
        }
        Ok(())
    }
}

pub fn is_empty_directory(path: &Path) -> bool {
    match path.read_dir() {
        Ok(mut o) => o.next().is_none(),
        Err(e) => {
            println!("error: {}", e);
            false
        }
    }
}
