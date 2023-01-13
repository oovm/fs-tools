use std::path::{Path, PathBuf};

use url::{ParseError, Url};

mod third_party;

/// A path to a resource, either local or remote.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ResourcePath {
    /// The network path to the resource.
    pub network: Url,
    /// The local path to the resource.
    pub local: PathBuf,
}

impl ResourcePath {
    /// Creates a new resource path.
    pub fn with_local<P: AsRef<Path>>(mut self, local: P) -> Self {
        self.local = local.as_ref().to_path_buf();
        self
    }
    /// Creates a new resource path.
    pub fn with_network<P: TryInto<Url>>(mut self, network: Url) -> Result<Self, ParseError> {
        self.network = network;
        Ok(self)
    }
}