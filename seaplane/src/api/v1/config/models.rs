use std::fmt;

mod encoded;

use encoded::Base64Encoded;
use serde::{Deserialize, Serialize};

/// Convenience macro for implementing encode/decode getters/setters for a struct with an inner `Base64Encoded`
//TODO: Is this actually any more "hidden" than a trait? I'm not sure
macro_rules! impl_base64 {
    ($a:ty) => {
        impl $a {
            /// Constructs from an unencoded byte array, encoding with URL-safe base64 in the process
            pub fn from_unencoded(unencoded: impl AsRef<[u8]>) -> Self {
                Self {
                    inner: Base64Encoded::from_unencoded(unencoded),
                }
            }

            /// Constructs a `Base64Encoded`, assuming the input is already encoded.
            pub fn from_encoded(encoded: impl Into<String>) -> Self {
                Self {
                    inner: Base64Encoded::from_encoded(encoded),
                }
            }

            /// Returns the inner string
            pub fn encoded(&self) -> &str {
                self.inner.encoded()
            }

            /// Returns the result of decoding the inner string.
            /// # Panics
            /// Will panic if the inner string is not correctly encoded.
            pub fn decode(&self) -> Vec<u8> {
                self.inner.decoded()
            }
        }
    };
}

/// A single key value pair, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct KeyValue {
    pub key: Key,
    pub value: Value,
}

impl KeyValue {
    pub fn into_value(self) -> Value {
        self.value
    }
}

/// A single key with which to access a value in the store, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct Key {
    inner: Base64Encoded,
}
impl_base64!(Key);

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// The raw bytes stored at a given key, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct Value {
    inner: Base64Encoded,
}
impl_base64!(Value);

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// The directory from which to perform a given range query, excluding the trailing slash, encoded in url-safe base64
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    inner: Base64Encoded,
}
impl_base64!(Directory);

/// The full context with which to perform a range query
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct RangeQueryContext {
    /// The directory, if any, within which to perform the range query.
    dir: Option<Directory>,
    /// The lower bound on the page of results to return.
    from: Option<Key>,
}

impl RangeQueryContext {
    /// Creates a blank context, suitable for querying the root directory at the first available key
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the directory within which to perform the query
    pub fn set_directory(&mut self, dir: Directory) {
        self.dir = Some(dir);
    }

    /// Set the key to use when beginning the next page of the query
    pub fn set_from(&mut self, next_key: Key) {
        self.from = Some(next_key);
    }

    pub fn directory(&self) -> &Option<Directory> {
        &self.dir
    }

    pub fn from(&self) -> &Option<Key> {
        &self.from
    }
}

/// The target of a request, representing either a single key or a range of keys.
#[derive(Debug, PartialEq, Eq)]
pub enum RequestTarget {
    Key(Key),
    Range(RangeQueryContext),
}

/// The response given from a range query
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyValueRange {
    /// A lower bound of the next page of results
    pub next_key: Option<Key>,
    /// The range of key value pairs returned
    pub kvs: Vec<KeyValue>,
}

#[cfg(test)]
mod config_models_test {
    use super::*;

    #[test]
    fn key_value_range_deserialize() {
        let deserialzied = serde_json::from_str(
            "{\"next_key\":\"bmV4dCBrZX\",\"kvs\":[{\"key\":\"aGVsbG8\",\"value\":\"dmFsdWU\"}]}",
        )
        .unwrap();

        assert_eq!(
            KeyValueRange {
                next_key: Some(Key::from_encoded("bmV4dCBrZX")),
                kvs: vec![KeyValue {
                    key: Key::from_encoded("aGVsbG8"),
                    value: Value::from_encoded("dmFsdWU"),
                },]
            },
            deserialzied
        );
    }

    #[test]
    fn key_value_range_serialize() {
        let serialized = serde_json::to_string(&KeyValueRange {
            next_key: Some(Key::from_encoded("bmV4dCBrZX")),
            kvs: vec![KeyValue {
                key: Key::from_encoded("aGVsbG8"),
                value: Value::from_encoded("dmFsdWU"),
            }],
        })
        .unwrap();

        assert_eq!(
            "{\"next_key\":\"bmV4dCBrZX\",\"kvs\":[{\"key\":\"aGVsbG8\",\"value\":\"dmFsdWU\"}]}",
            serialized
        );
    }
}
