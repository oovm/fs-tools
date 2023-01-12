

impl Debug for ResourcePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcePath").field("network", &self.network.as_str()).field("local", &self.local).finish()
    }
}

impl Display for ResourcePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}?local={}", self.network, self.local.display())
    }
}

impl FromStr for ResourcePath {
    type Err = DiffuserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut url = Url::parse(s)?;
        let mut local = PathBuf::new();
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "local" | "relative" => local = PathBuf::from(value.as_ref()),
                _ => {}
            }
        }
        url.set_query(None);
        match local.to_string_lossy().eq("") {
            true => Err(DiffuserError::decode_error("Missing query in resource url.")),
            false => Ok(Self { network: url, local }),
        }
    }
}
