use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("Wrong number of bytes to construct Prefix")]
    PrefixByteLength,
    #[error("Prefix characters may only be ASCII values of 2-7,a-z")]
    InvalidPrefixChar,
    #[error("Only UUIDv7 is supported, but another UUID version was given")]
    UnsupportedUuidVersion,
    #[error("Attempted to deserialize OID without a prefix")]
    MissingPrefix,
    #[error("deserialize OID without a separator")]
    MissingSeparator,
    #[error("Attempted to deserialize OID without a value")]
    MissingValue,
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
    #[error("Base32 Decode error: {0}")]
    Base32Decode(#[from] data_encoding::DecodeError),
}
