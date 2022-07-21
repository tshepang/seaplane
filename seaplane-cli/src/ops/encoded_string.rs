use std::{fmt, result::Result as StdResult};

use serde::{ser::Serializer, Serialize};

use crate::error::Result;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EncodedString {
    Base64(String),
    Simple(Vec<u8>),
}

impl Serialize for EncodedString {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl EncodedString {
    /// Decodes into binary format
    pub fn decode(self) -> Result<Self> {
        use EncodedString::*;

        let ret = match self {
            Base64(s) => Simple(base64::decode_config(&s, base64::URL_SAFE_NO_PAD)?),
            Simple(b) => Simple(b),
        };

        Ok(ret)
    }
}

impl Default for EncodedString {
    fn default() -> Self {
        EncodedString::Simple(vec![])
    }
}

impl fmt::Display for EncodedString {
    // Bit of a footgun here, we "display" as Base64 regardless of encoding.
    // Use direct writes for binary data.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodedString::Base64(s) => write!(f, "{}", s),
            EncodedString::Simple(v) => {
                write!(f, "{}", base64::encode_config(v, base64::URL_SAFE_NO_PAD))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bin() -> Vec<u8> {
        b"Hey\x01There".to_vec()
    }

    fn base64() -> String {
        "SGV5AVRoZXJl".to_owned()
    }

    #[test]
    fn test_decode() -> Result<()> {
        let decoded = EncodedString::Base64(base64()).decode()?;
        assert_eq!(decoded, EncodedString::Simple(bin()));
        Ok(())
    }
}
