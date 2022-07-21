use std::io::Write;

use seaplane::api::v1::metadata::{Key as KeyModel, KeyValue as KeyValueModel};
use serde::Serialize;

use crate::{
    context::Ctx,
    error::Result,
    ops::EncodedString,
    printer::{printer, Output},
};

/// We use our own KeyValue instead of the models because we need to *not* enforce base64 encoding,
/// and implement a bunch of additional methods and traits that wouldn't make sense for the models
///
/// We also need to keep track if the values are encoded or not
#[derive(Debug, Default, Clone, Serialize)]
pub struct KeyValue {
    pub key: Option<EncodedString>,
    pub value: Option<EncodedString>,
}

impl KeyValue {
    /// Creates a new KeyValue from an encoded key and value. You must pinky promise the key and
    /// value are URL safe base64 encoded or Bad Things may happen.
    pub fn new<S: Into<String>>(key: S, value: S) -> Self {
        Self {
            key: Some(EncodedString::Base64(key.into())),
            value: Some(EncodedString::Base64(value.into())),
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
            key: Some(EncodedString::Base64(key.into())),
            ..Self::default()
        }
    }

    /// Sets the value to some base64 encoded value
    pub fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = Some(EncodedString::Base64(value.into()))
    }

    /// Set the key to `None` without touching the value.
    pub fn clear_key(&mut self) {
        self.key = None;
    }

    /// Set the value to `None` without touching the key.
    pub fn clear_value(&mut self) {
        self.value = None;
    }

    /// Creates a new Key from self's data.
    pub fn to_key_model(&self) -> Result<KeyModel> {
        Ok(match &self.key {
            Some(EncodedString::Base64(s)) => KeyModel::from_encoded(s.to_string()),
            Some(EncodedString::Simple(s)) => KeyModel::from_unencoded(s),
            None => unimplemented!("TODO error for no key"),
        })
    }

    /// Decodes the key and value if needed
    pub fn decode(&mut self) -> Result<()> {
        if let Some(key) = self.key.take() {
            self.key = Some(key.decode()?);
        }
        if let Some(value) = self.value.take() {
            self.value = Some(value.decode()?);
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

    pub fn to_decoded(mut self) -> Result<Self> {
        self.inner.iter_mut().try_for_each(|kv| kv.decode())?;
        Ok(self)
    }

    pub fn keys(&self) -> impl Iterator<Item = String> + '_ {
        self.inner
            .iter()
            .filter_map(|kv| kv.key.as_ref().map(|k| k.to_string()))
    }

    // print a table in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_table(&self, headers: bool) -> Result<()> {
        use EncodedString::*;
        // TODO: we may have to add ways to elide long keys or values with ... after some char count
        let mut ptr = printer();

        let mut iter = self.iter().peekable();
        while let Some(kv) = iter.next() {
            match &kv.key {
                Some(Base64(s)) => {
                    if headers {
                        write!(ptr, "KEY: ")?;
                    }
                    writeln!(ptr, "{s}")?;
                }
                Some(Simple(v)) => {
                    if headers {
                        write!(ptr, "KEY: ")?;
                    }
                    ptr.write_all(v)?;
                    writeln!(ptr)?;
                }
                None => (),
            }
            match &kv.value {
                Some(Base64(s)) => {
                    if headers {
                        writeln!(ptr, "VALUE:")?;
                    }
                    writeln!(ptr, "{s}")?;
                }
                Some(Simple(v)) => {
                    if headers {
                        writeln!(ptr, "VALUE:")?;
                    }
                    ptr.write_all(v)?;
                    writeln!(ptr)?;
                }
                None => (),
            }
            if iter.peek().is_some() {
                writeln!(ptr, "---")?;
            }
        }
        ptr.flush()?;

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
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
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
            return this.to_decoded()?.impl_print_table(!mdctx.no_header);
        }
        this.impl_print_table(!mdctx.no_header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn build_kvs() -> KeyValues {
        use EncodedString::*;
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
        use EncodedString::*;
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
    fn serialize_keyvalues_simple() {
        let kvs = build_kvs_invalid_utf8();

        assert_eq!(
            serde_json::to_string(&kvs).unwrap(),
            json!([{"key":"a2V5_zE","value":"dmFsdWX_MQ"}]).to_string()
        );
    }
}
