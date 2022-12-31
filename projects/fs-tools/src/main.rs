use clap::Parser;
use diagnostic_quick::QResult;

use fs_tool::FSTools;

fn main() -> QResult {
    let app = FSTools::parse();
    app.run()
}