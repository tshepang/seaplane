use std::{
    fmt,
    io::{self, Write},
    result::Result as StdResult,
};

use seaplane::api::v1::config::{Key as KeyModel, KeyValue as KeyValueModel};
use serde::{ser::Serializer, Serialize};

use crate::{
    context::{metadata::DisplayEncodingFormat, Ctx},
    error::Result,
    printer::Output,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KeyValueInner {
    Base64(String),
    Utf8(String),
    Hex(String),
    Simple(Vec<u8>),
}

impl Serialize for KeyValueInner {
    fn serialize<S: Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl KeyValueInner {
    /// Either decodes the base64 into the specified encoding, or converts the already decoded into
    /// a different encoding
    pub fn decode(self, encoding: DisplayEncodingFormat) -> Result<Self> {
        use KeyValueInner::*;
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
        use KeyValueInner::*;
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
        matches!(self, KeyValueInner::Base64(_))
    }

    /// Returns true if the data is currently hex encoded
    pub fn is_hex(&self) -> bool {
        matches!(self, KeyValueInner::Hex(_))
    }

    /// Returns true if the data is currently decoded raw bytes
    pub fn is_simple(&self) -> bool {
        matches!(self, KeyValueInner::Simple(_))
    }

    /// Returns true if the data is currently decoded to a UTF-8 Lossy String
    pub fn is_utf8(&self) -> bool {
        matches!(self, KeyValueInner::Utf8(_))
    }

    /// Creates a new Key from self's data.
    ///
    /// NOTE: If self is currently Utf8, the encoded Base64 value may not match the original if
    /// invalid UTF-8 bytes were lost and replaced with U+FFFD
    pub fn to_key_model(&self) -> Result<KeyModel> {
        Ok(match self {
            KeyValueInner::Base64(s) => KeyModel::from_encoded(s.to_string()),
            KeyValueInner::Utf8(s) => KeyModel::from_unencoded(s),
            KeyValueInner::Hex(s) => KeyModel::from_unencoded(hex::decode(s)?),
            KeyValueInner::Simple(s) => KeyModel::from_unencoded(s),
        })
    }
}

impl fmt::Display for KeyValueInner {
    /// NOTE: Displaying a Simple value will first convert it to Utf8
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KeyValueInner::Base64(s) => write!(f, "{}", s),
            KeyValueInner::Utf8(s) => write!(f, "{}", s),
            KeyValueInner::Hex(s) => write!(f, "{}", s),
            KeyValueInner::Simple(v) => write!(f, "{}", String::from_utf8_lossy(v)),
        }
    }
}

/// We use our own KeyValue instead of the models because we need to *not* enforce base64 encoding,
/// and implement a bunch of additional methods and traits that wouldn't make sense for the models
///
/// We also need to keep track if the values are encoded or not
#[derive(Debug, Default, Clone, Serialize)]
pub struct KeyValue {
    pub key: Option<KeyValueInner>,
    pub value: Option<KeyValueInner>,
}

impl KeyValue {
    /// Creates a new KeyValue from an encoded key and value. You must pinky promise the key and
    /// value are URL safe base64 encoded or Bad Things may happen.
    pub fn new<S: Into<String>>(key: S, value: S) -> Self {
        Self {
            key: Some(KeyValueInner::Base64(key.into())),
            value: Some(KeyValueInner::Base64(value.into())),
        }
    }

    /// Creates a new KeyValue from an un-encoded key and value, encoding them along the way
    pub fn new_unencoded<S: AsRef<str>>(key: S, value: S) -> Self {
        Self::new(
            base64::encode_config(key.as_ref(), base64::URL_SAFE_NO_PAD),
            base64::encode_config(value.as_ref(), base64::URL_SAFE_NO_PAD),
        )
    }

    /// Creates a new KeyValue from an un-encoded string ref, encoding it along the way
    pub fn from_key_unencoded<S: AsRef<str>>(key: S) -> Self {
        Self::from_key(base64::encode_config(key.as_ref(), base64::URL_SAFE_NO_PAD))
    }

    /// Creates a new KeyValue from an already encoded string ref. You must pinky promise the key
    /// is URL safe base64 encoded or Bad Things may happen.
    pub fn from_key<S: Into<String>>(key: S) -> Self {
        Self {
            key: Some(KeyValueInner::Base64(key.into())),
            ..Self::default()
        }
    }

    /// Sets the value to some base64 encoded value
    pub fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = Some(KeyValueInner::Base64(value.into()))
    }

    /// Set the key to `None` without touching the value.
    pub fn clear_key(&mut self) {
        self.key = None;
    }

    /// Set the value to `None` without touching the key.
    pub fn clear_value(&mut self) {
        self.value = None;
    }

    /// Decodes the key and value if needed
    pub fn decode(&mut self, encoding: DisplayEncodingFormat) -> Result<()> {
        if let Some(key) = self.key.take() {
            self.key = Some(key.decode(encoding)?);
        }
        if let Some(value) = self.value.take() {
            self.value = Some(value.decode(encoding)?);
        }

        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(transparent)]
pub struct KeyValues {
    inner: Vec<KeyValue>,
}

impl KeyValues {
    pub fn from_model(model: Vec<KeyValueModel>) -> Self {
        Self {
            inner: model
                .iter()
                .map(|kv| KeyValue::new(kv.key.as_ref(), kv.value.as_ref()))
                .collect(),
        }
    }

    /// Inserts an already base64 encoded key and value
    pub fn insert<S: Into<String>>(&mut self, key: S, value: S) {
        self.inner.push(KeyValue::new(key, value));
    }

    pub fn iter(&self) -> impl Iterator<Item = &KeyValue> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut KeyValue> {
        self.inner.iter_mut()
    }

    pub fn push(&mut self, kv: KeyValue) {
        self.inner.push(kv)
    }

    pub fn to_decoded(mut self, encoding: DisplayEncodingFormat) -> Result<Self> {
        self.inner
            .iter_mut()
            .try_for_each(|kv| kv.decode(encoding))?;
        Ok(self)
    }

    pub fn keys(&self) -> impl Iterator<Item = String> + '_ {
        self.inner
            .iter()
            .filter_map(|kv| kv.key.as_ref().map(|k| k.to_string()))
    }

    // print JSON in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_json(&self) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    // print a table in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_table(&self, headers: bool) -> Result<()> {
        use KeyValueInner::*;
        // TODO: we may have to add ways to elide long keys or values with ... after some char count
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        let mut iter = self.iter().peekable();
        while let Some(kv) = iter.next() {
            match &kv.key {
                Some(Hex(s)) | Some(Utf8(s)) | Some(Base64(s)) => {
                    if headers {
                        write!(&mut lock, "KEY: ")?;
                    }
                    writeln!(&mut lock, "{s}")?;
                }
                Some(Simple(v)) => {
                    if headers {
                        write!(&mut lock, "KEY: ")?;
                    }
                    lock.write_all(v)?;
                    writeln!(&mut lock)?;
                }
                None => (),
            }
            match &kv.value {
                Some(Hex(s)) | Some(Utf8(s)) | Some(Base64(s)) => {
                    if headers {
                        writeln!(&mut lock, "VALUE:")?;
                    }
                    writeln!(&mut lock, "{s}")?;
                }
                Some(Simple(v)) => {
                    if headers {
                        writeln!(&mut lock, "VALUE:")?;
                    }
                    lock.write_all(v)?;
                    writeln!(&mut lock)?;
                }
                None => (),
            }
            if iter.peek().is_some() {
                writeln!(&mut lock, "---")?;
            }
        }
        lock.flush()?;

        Ok(())
    }
}

impl Output for KeyValues {
    fn print_json(&self, ctx: &Ctx) -> Result<()> {
        let mut this = self.clone();
        let mdctx = ctx.md_ctx.get_or_init();
        if mdctx.no_keys {
            this.inner.iter_mut().for_each(|kv| {
                kv.key.take();
            });
        }
        if mdctx.no_values {
            this.inner.iter_mut().for_each(|kv| {
                kv.value.take();
            });
        }
        if mdctx.decode {
            // TODO: for lots of keys or lots of big keys this may need improved performance?
            return this.to_decoded(mdctx.disp_encoding)?.impl_print_json();
        }
        this.impl_print_json()
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let mut this = self.clone();
        let mdctx = ctx.md_ctx.get_or_init();
        if mdctx.no_keys {
            this.inner.iter_mut().for_each(|kv| {
                kv.key.take();
            });
        }
        if mdctx.no_values {
            this.inner.iter_mut().for_each(|kv| {
                kv.value.take();
            });
        }
        if mdctx.decode {
            // TODO: for lots of keys or lots of big keys this may need improved performance?
            return this
                .to_decoded(mdctx.disp_encoding)?
                .impl_print_table(!mdctx.no_header);
        }
        this.impl_print_table(!mdctx.no_header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn build_kvs() -> KeyValues {
        use KeyValueInner::*;
        KeyValues {
            inner: vec![
                KeyValue {
                    key: Some(Base64("a2V5MQ".into())),
                    value: Some(Base64("dmFsdWUx".into())),
                },
                KeyValue {
                    key: Some(Base64("a2V5Mg".into())),
                    value: Some(Base64("dmFsdWUy".into())),
                },
                KeyValue {
                    key: Some(Base64("a2V5Mw".into())),
                    value: Some(Base64("dmFsdWUz".into())),
                },
            ],
        }
    }

    fn build_kvs_invalid_utf8() -> KeyValues {
        use KeyValueInner::*;
        KeyValues {
            inner: vec![KeyValue {
                key: Some(Simple(vec![107, 101, 121, 0xFF, 49])),
                value: Some(Simple(vec![118, 97, 108, 117, 101, 0xFF, 49])),
            }],
        }
    }

    #[test]
    fn serialize_keyvalues_base64() {
        let kvs = build_kvs();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "a2V5MQ", "value": "dmFsdWUx"}, {"key": "a2V5Mg", "value": "dmFsdWUy"}, {"key": "a2V5Mw", "value": "dmFsdWUz"}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_base64_no_keys() {
        let mut kvs = build_kvs();
        kvs.inner.iter_mut().for_each(|kv| {
            kv.key.take();
        });

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": null, "value": "dmFsdWUx"}, {"key": null, "value": "dmFsdWUy"}, {"key": null, "value": "dmFsdWUz"}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_base64_no_values() {
        let mut kvs = build_kvs();
        kvs.inner.iter_mut().for_each(|kv| {
            kv.value.take();
        });

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "a2V5MQ", "value": null}, {"key": "a2V5Mg", "value": null}, {"key": "a2V5Mw", "value": null}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_hex() {
        let kvs = build_kvs().to_decoded(DisplayEncodingFormat::Hex).unwrap();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "6b657931", "value": "76616c756531"}, {"key": "6b657932", "value": "76616c756532"}, {"key": "6b657933", "value": "76616c756533"}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_hex_invalid_utf8() {
        let kvs = build_kvs_invalid_utf8()
            .to_decoded(DisplayEncodingFormat::Hex)
            .unwrap();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "6b6579ff31", "value": "76616c7565ff31"}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_utf8() {
        let kvs = build_kvs_invalid_utf8()
            .to_decoded(DisplayEncodingFormat::Utf8)
            .unwrap();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "key\u{FFFD}1", "value": "value\u{FFFD}1"}]).to_string()
        );
    }

    #[test]
    fn serialize_keyvalues_simple() {
        let kvs = build_kvs_invalid_utf8();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key": "key\u{FFFD}1", "value": "value\u{FFFD}1"}]).to_string()
        );
    }
}
