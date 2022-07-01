use std::io::Write;

use seaplane::api::v1::locks::LockName as LockNameModel;
use serde::Serialize;

use crate::{
    context::Ctx,
    error::Result,
    ops::{DisplayEncodingFormat, EncodedString},
    printer::{printer, Output},
};

/// We use our own LockName instead of the models because we need to *not* enforce base64 encoding,
/// and implement a bunch of additional methods and traits that wouldn't make sense for the models
///
/// We also need to keep track if the values are encoded or not
#[derive(Debug, Default, Clone, Serialize)]
pub struct LockName {
    pub name: EncodedString,
}

impl LockName {
    /// Creates a new LockName from an encoded name. You must pinky promise the name
    /// is URL safe base64 encoded or Bad Things may happen.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: EncodedString::Base64(name.into()),
        }
    }

    /// Creates a new LockName from an un-encoded name, encoding it along the way
    pub fn new_unencoded<S: AsRef<str>>(name: S) -> Self {
        Self::new(base64::encode_config(
            name.as_ref(),
            base64::URL_SAFE_NO_PAD,
        ))
    }

    /// Creates a new LockName from an un-encoded string ref, encoding it along the way
    pub fn from_name_unencoded<S: AsRef<str>>(name: S) -> Self {
        Self::from_name(base64::encode_config(
            name.as_ref(),
            base64::URL_SAFE_NO_PAD,
        ))
    }

    /// Creates a new LockName from an already encoded string ref. You must pinky promise the name
    /// is URL safe base64 encoded or Bad Things may happen.
    pub fn from_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: EncodedString::Base64(name.into()),
        }
    }

    /// Creates a new LockName from self's data.
    ///
    /// NOTE: If self is currently Utf8, the encoded Base64 value may not match the original if
    /// invalid UTF-8 bytes were lost and replaced with U+FFFD
    pub fn to_model(&self) -> Result<LockNameModel> {
        Ok(match &self.name {
            EncodedString::Base64(s) => LockNameModel::from_encoded(s.to_string()),
            EncodedString::Utf8(s) => LockNameModel::from_unencoded(s),
            EncodedString::Hex(s) => LockNameModel::from_unencoded(hex::decode(s)?),
            EncodedString::Simple(s) => LockNameModel::from_unencoded(s),
        })
    }

    /// Decodes the name if needed
    pub fn decode(mut self, encoding: DisplayEncodingFormat) -> Result<Self> {
        self.name = self.name.decode(encoding)?;

        Ok(self)
    }

    // print JSON in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_json(&self) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    // print a table in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_table(&self, headers: bool) -> Result<()> {
        use EncodedString::*;

        let mut ptr = printer();

        match &self.name {
            Hex(s) | Utf8(s) | Base64(s) => {
                if headers {
                    write!(ptr, "LOCK-NAME: ")?;
                }
                writeln!(ptr, "{s}")?;
            }
            Simple(v) => {
                if headers {
                    write!(ptr, "LOCK-NAME: ")?;
                }
                ptr.write_all(v)?;
                writeln!(ptr)?;
            }
        }

        ptr.flush()?;

        Ok(())
    }
}

impl Output for LockName {
    fn print_json(&self, ctx: &Ctx) -> Result<()> {
        let this = self.clone();
        let locksctx = ctx.locks_ctx.get_or_init();
        if locksctx.decode {
            return this.decode(locksctx.disp_encoding)?.impl_print_json();
        }
        self.impl_print_json()
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let this = self.clone();
        let locksctx = ctx.locks_ctx.get_or_init();
        if locksctx.decode {
            return this
                .decode(locksctx.disp_encoding)?
                .impl_print_table(!locksctx.no_header);
        }
        self.impl_print_table(!locksctx.no_header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_base64() -> EncodedString {
        EncodedString::Base64("a2V5MQ".into())
    }

    fn invalid_utf8() -> EncodedString {
        EncodedString::Simple(vec![107, 101, 121, 0xFF, 49])
    }

    #[test]
    fn serialize_locknameinner_base64() {
        let name = valid_base64();

        assert_eq!(
            serde_json::to_string(&name).unwrap(),
            json!("a2V5MQ").to_string()
        );
    }

    #[test]
    fn serialize_lockname_hex() {
        let lock_name = valid_base64().decode(DisplayEncodingFormat::Hex).unwrap();

        assert_eq!(
            serde_json::to_string(&lock_name).unwrap(),
            json!("6b657931").to_string()
        );
    }

    #[test]
    fn serialize_lockname_hex_invalid_utf8() {
        let lock_name = invalid_utf8().decode(DisplayEncodingFormat::Hex).unwrap();

        assert_eq!(
            serde_json::to_string(&lock_name).unwrap(),
            json!("6b6579ff31").to_string()
        );
    }

    #[test]
    fn serialize_lockname_utf8() {
        let lock_name = invalid_utf8().decode(DisplayEncodingFormat::Utf8).unwrap();

        assert_eq!(
            serde_json::to_string(&lock_name).unwrap(),
            json!("key\u{FFFD}1").to_string()
        );
    }

    #[test]
    fn serialize_lockname_simple() {
        let lock_name = invalid_utf8();

        assert_eq!(
            serde_json::to_string(&lock_name).unwrap(),
            json!("key\u{FFFD}1").to_string()
        );
    }
}
