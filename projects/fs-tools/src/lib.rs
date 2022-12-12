use clap::{Args, Parser, Subcommand};
use diagnostic_quick::{QError, QResult};

use self::cmds::FsFlatten;

mod cmds;
pub mod utils;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct FSTools {
    #[command(subcommand)]
    cmds: Option<FSCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum FSCommands {
    Flatten(Box<FsFlatten>)
}

#[derive(Args, Debug, Clone)]
pub struct SharedArgs {
    #[arg(long)]
    dry_run: bool,
}

impl FSTools {
    pub fn run(&self) -> QResult<()> {
        match &self.cmds {
            None => {
                return Err(QError::runtime_error("No command specified"));
            }
            Some(cmd) => {
                match cmd {
                    FSCommands::Flatten(cmd) => {
                        cmd.run()
                    }
                }
            }
        }
    }
}