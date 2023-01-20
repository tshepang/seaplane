/// Implements Deserialize using FromStr
macro_rules! impl_deser_from_str {
    ($t:ty) => {
        impl<'de> ::serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                s.parse().map_err(::serde::de::Error::custom)
            }
        }
    };
}
