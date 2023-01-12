impl _serde::Serialize for ResourcePath {
    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
    {
        let mut __serde_state =
            match (_serde::Serializer::serialize_struct(__serializer, "ResourcePath", false as usize + 1 + 1)) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
        match (_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "network", &self.network)) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        }
        match (_serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "local", &self.local)) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        }
        _serde::ser::SerializeStruct::end(__serde_state)
    }
}
