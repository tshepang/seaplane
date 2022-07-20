use std::io::Write;

use seaplane::api::v1::{locks::LockName as LockNameModel, LockInfo, LockInfoInner};
use serde::Serialize;
use tabwriter::TabWriter;

use crate::{
    context::Ctx,
    error::{CliError, Result},
    ops::EncodedString,
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

    /// Creates a new LockName from an un-encoded string ref, encoding it along the way
    pub fn from_name_unencoded<S: AsRef<str>>(name: S) -> Self {
        let name = base64::encode_config(name.as_ref(), base64::URL_SAFE_NO_PAD);
        Self {
            name: EncodedString::Base64(name),
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
}

#[derive(Debug, Serialize)]
pub struct HeldLock {
    pub lock_id: String,
    pub sequencer: u32,
}

impl Output for HeldLock {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let show_headers = !ctx.locks_ctx.get_or_init().no_header;
        let mut ptr = printer();

        let id_prefix = if show_headers { "LOCK-ID: " } else { "" };
        let seq_prefix = if show_headers { "SEQUENCER: " } else { "" };
        writeln!(ptr, "{id_prefix}{}", self.lock_id)?;
        writeln!(ptr, "{seq_prefix}{}", self.sequencer)?;

        ptr.flush()?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct ListedLockInfoInner {
    pub ttl: u32,
    #[serde(rename = "client-id")]
    pub client_id: String,
    pub ip: String,
}

impl From<LockInfoInner> for ListedLockInfoInner {
    fn from(other: LockInfoInner) -> Self {
        Self {
            ttl: other.ttl,
            client_id: other.client_id,
            ip: other.ip,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListedLock {
    name: EncodedString,
    id: String,
    info: ListedLockInfoInner,
}

impl From<LockInfo> for ListedLock {
    fn from(other: LockInfo) -> Self {
        let info = other.info.into();
        Self {
            name: EncodedString::Base64(other.name.encoded().to_owned()),
            id: other.id.encoded().to_owned(),
            info,
        }
    }
}

pub fn print_lock_table<I>(headers: bool, chunk: I, ctx: &Ctx) -> Result<()>
where
    I: IntoIterator<Item = ListedLock>,
{
    let buffer = Vec::new();
    let mut tw = TabWriter::new(buffer);
    if headers {
        writeln!(tw, "LOCK-NAME\tLOCK-ID\tCLIENT-ID\tCLIENT-IP\tTTL")?;
    }

    let locksctx = ctx.locks_ctx.get_or_init();
    for l in chunk {
        let show_name = if locksctx.decode {
            l.name.clone().decode(locksctx.disp_encoding)?
        } else {
            l.name.clone()
        };

        writeln!(
            tw,
            "{}\t{}\t{}\t{}\t{}",
            show_name, l.id, l.info.client_id, l.info.ip, l.info.ttl
        )?;
    }
    tw.flush()?;

    let mut ptr = printer();
    let page = tw
        .into_inner()
        .map_err(|_| CliError::bail("IO flush error writing locks"))?;
    ptr.write_all(&page)?;
    ptr.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ops::DisplayEncodingFormat;
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
