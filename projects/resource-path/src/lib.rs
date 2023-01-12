use std::{
    fs::create_dir_all,
    io::{Error, Result},
    path::{Path, PathBuf},
};

pub use find_dir::{find_directory, find_directory_or_create, this_directory};

mod third_party;

use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::DiffuserError;



#[derive(Clone, Serialize)]
pub struct ResourcePath {
    pub network: Url,
    pub local: PathBuf,
}

#[test]
fn test() {
    let s = "https://api.github.com/a?local=a/b/c";
    println!("{:?}", ResourcePath::from_str(s))
}
