use std::{fmt, result::Result as StdResult};

use serde::{ser::Serializer, Serialize};

use crate::error::Result;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EncodedString(String);

impl Serialize for EncodedString {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl EncodedString {
    pub fn new(s: String) -> Self { EncodedString(s) }

    /// Decodes into binary format
    pub fn decoded(&self) -> Result<Vec<u8>> {
        let ret = base64::decode_config(&self.0, base64::URL_SAFE_NO_PAD)?;
        Ok(ret)
    }
}

impl Default for EncodedString {
    fn default() -> Self { EncodedString("".to_owned()) }
}

impl fmt::Display for EncodedString {
    // Bit of a footgun here, we "display" as Base64 regardless of encoding.
    // Use direct writes for binary data.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bin() -> Vec<u8> { b"Hey\x01There".to_vec() }

    fn base64() -> String { "SGV5AVRoZXJl".to_owned() }

    #[test]
    fn test_decode() -> Result<()> {
        let decoded = EncodedString(base64()).decoded()?;
        assert_eq!(decoded, bin());
        Ok(())
    }
}
