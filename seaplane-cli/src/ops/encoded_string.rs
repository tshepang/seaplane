use std::{fmt, result::Result as StdResult};

use serde::{ser::Serializer, Serialize};
use strum::{EnumString, EnumVariantNames};

use crate::error::Result;

#[derive(EnumString, strum::Display, EnumVariantNames, Copy, Clone, Debug, PartialEq)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum DisplayEncodingFormat {
    Simple,
    Utf8,
    Hex,
}

impl Default for DisplayEncodingFormat {
    fn default() -> Self {
        Self::Simple
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EncodedString {
    Base64(String),
    Utf8(String),
    Hex(String),
    Simple(Vec<u8>),
}

impl Default for EncodedString {
    fn default() -> Self {
        EncodedString::Utf8(String::new())
    }
}

impl Serialize for EncodedString {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl EncodedString {
    /// Either decodes the base64 into the specified encoding, or converts the already decoded into
    /// a different encoding
    pub fn decode(self, encoding: DisplayEncodingFormat) -> Result<Self> {
        use EncodedString::*;
        if !self.is_base64() {
            return self.convert(encoding);
        }

        let base64_str = match self {
            Base64(s) => s,
            // We already checked that we are in fact currently Base64
            _ => unreachable!(),
        };

        Ok(match encoding {
            DisplayEncodingFormat::Simple => {
                Simple(base64::decode_config(&base64_str, base64::URL_SAFE_NO_PAD)?)
            }
            DisplayEncodingFormat::Utf8 => Utf8(
                String::from_utf8_lossy(&base64::decode_config(
                    &base64_str,
                    base64::URL_SAFE_NO_PAD,
                )?)
                .to_string(),
            ),
            DisplayEncodingFormat::Hex => Hex(hex::encode(base64::decode_config(
                &base64_str,
                base64::URL_SAFE_NO_PAD,
            )?)),
        })
    }

    /// Converts from one already encoded format to another. Will recursively call (a single time)
    /// `decode` if the value is not yet decoded from Base64
    ///
    /// **Note**: Converting *from* `Utf8` does not restore the lost bytes that were replaced with
    /// U+FFFD.
    pub fn convert(mut self, encoding: DisplayEncodingFormat) -> Result<Self> {
        use EncodedString::*;
        if self.is_base64() {
            return self.decode(encoding);
        }

        self = match encoding {
            DisplayEncodingFormat::Simple => match self {
                Simple(_) => self,
                Hex(s) => Simple(hex::decode(s)?),
                Utf8(s) => Simple(s.into_bytes()),
                Base64(_) => unreachable!(),
            },
            DisplayEncodingFormat::Utf8 => match self {
                Simple(v) => Utf8(String::from_utf8_lossy(&v).to_string()),
                Hex(s) => Utf8(String::from_utf8_lossy(&hex::decode(s)?).to_string()),
                Utf8(_) => self,
                Base64(_) => unreachable!(),
            },
            DisplayEncodingFormat::Hex => match self {
                Simple(v) => Hex(hex::encode(v)),
                Hex(_) => self,
                Utf8(s) => Hex(hex::encode(s.into_bytes())),
                Base64(_) => unreachable!(),
            },
        };

        Ok(self)
    }

    /// Returns true if the data is currently Base64 encoded
    pub fn is_base64(&self) -> bool {
        matches!(self, EncodedString::Base64(_))
    }

    /// Returns true if the data is currently hex encoded
    pub fn is_hex(&self) -> bool {
        matches!(self, EncodedString::Hex(_))
    }

    /// Returns true if the data is currently decoded raw bytes
    pub fn is_simple(&self) -> bool {
        matches!(self, EncodedString::Simple(_))
    }

    /// Returns true if the data is currently decoded to a UTF-8 Lossy String
    pub fn is_utf8(&self) -> bool {
        matches!(self, EncodedString::Utf8(_))
    }
}

impl fmt::Display for EncodedString {
    /// NOTE: Displaying a Simple value will first convert it to Utf8
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncodedString::Base64(s) => write!(f, "{}", s),
            EncodedString::Utf8(s) => write!(f, "{}", s),
            EncodedString::Hex(s) => write!(f, "{}", s),
            EncodedString::Simple(v) => write!(f, "{}", String::from_utf8_lossy(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_base64() -> EncodedString {
        EncodedString::Base64("a2V5MQ".into())
    }

    fn invalid_utf8() -> EncodedString {
        EncodedString::Simple(vec![107, 101, 121, 0xFF, 49])
    }

    #[test]
    fn decode_hex() {
        assert_eq!(
            &valid_base64()
                .decode(DisplayEncodingFormat::Hex)
                .unwrap()
                .to_string(),
            "6b657931"
        );
    }

    #[test]
    fn transcode_invalid_utf8_to_hex() {
        assert_eq!(
            &invalid_utf8()
                .decode(DisplayEncodingFormat::Hex)
                .unwrap()
                .to_string(),
            "6b6579ff31"
        );
    }

    #[test]
    fn transcode_invalid_utf8_to_utf8() {
        assert_eq!(
            &invalid_utf8()
                .decode(DisplayEncodingFormat::Utf8)
                .unwrap()
                .to_string(),
            "key\u{FFFD}1"
        );
    }
}
