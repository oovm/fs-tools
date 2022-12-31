use clap::Parser;
use diagnostic_quick::QResult;

use fs_flatten::FlattenFlies;

use crate::utils::{build_glob_set, resolve_workspace};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct FsFlatten {
    /// Folder that needs to be flattened
    workspace: Option<String>,
    /// Delete all empty folders
    #[arg(short, long)]
    delete_empty: bool,
    /// Glob pattern of files to move
    #[arg(short, long)]
    pattern: Option<String>,
}


impl FsFlatten {
    pub fn run(&self) -> QResult {
        let ws = resolve_workspace(&self.workspace)?;
        let worker = FlattenFlies {
            output: ws,
            pattern: build_glob_set(&self.pattern)?,
            delete_empty: self.delete_empty,
        };
        worker.run(&worker.output)
    }
}