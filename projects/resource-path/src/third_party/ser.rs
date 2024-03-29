use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::ResourcePath;

impl Serialize for ResourcePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("ResourcePath", 2)?;
        s.serialize_field("network", &self.network)?;
        s.serialize_field("local", &self.local)?;
        s.end()
    }
}
