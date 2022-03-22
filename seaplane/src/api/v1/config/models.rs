use serde::{Deserialize, Serialize};

/// A single key value pair, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyValue {
    pub key: Key,
    pub value: Value,
}

/// A single key with which to access a value in the store, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Key(pub String);

/// The raw bytes stored at a given key, encoded in url-safe base64.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Value(pub String);

/// The directory from which to perform a given range query, excluding the trailing slash, encoded in url-safe base64
#[derive(Debug, PartialEq, Eq)]
pub struct Directory(pub String);

/// The full context with which to perform a range query
#[derive(Debug, PartialEq, Eq, Default)]
pub struct RangeQueryContext {
    /// The directory, if any, within which to perform the range query.
    pub dir: Option<Directory>,
    /// The key after which (lexicographically) results are returned.
    pub after: Option<Key>,
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
    /// Indicates that there are more values to be read.
    pub more: bool,
    /// The last key in the range, can be used to read more values if needed.
    pub last: Key,
    /// The range of key value pairs returned
    pub kvs: Vec<KeyValue>,
}
