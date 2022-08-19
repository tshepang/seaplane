use std::io::Write;

use seaplane::api::v1::metadata::KeyValue as KeyValueModel;
use serde::Serialize;
use tabwriter::TabWriter;

use crate::{
    context::Ctx,
    error::{CliError, Result},
    ops::EncodedString,
    printer::{printer, Output},
};

/// We use our own KeyValue instead of the models because we need to *not* enforce base64 encoding,
/// and implement a bunch of additional methods and traits that wouldn't make sense for the models
///
/// We also need to keep track if the values are encoded or not
#[derive(Debug, Default, Clone, Serialize)]
pub struct KeyValue {
    pub key: EncodedString,
    pub value: EncodedString,
}

impl KeyValue {
    /// Creates a new KeyValue from an encoded key and value. You must pinky promise the key and
    /// value are URL safe base64 encoded or Bad Things may happen.
    pub fn new<S: Into<String>>(key: S, value: S) -> Self {
        Self { key: EncodedString::new(key.into()), value: EncodedString::new(value.into()) }
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
        Self { key: EncodedString::new(key.into()), ..Self::default() }
    }

    /// Sets the value to some base64 encoded value
    pub fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = EncodedString::new(value.into())
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

    pub fn iter(&self) -> impl Iterator<Item = &KeyValue> { self.inner.iter() }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut KeyValue> { self.inner.iter_mut() }

    pub fn push(&mut self, kv: KeyValue) { self.inner.push(kv) }

    pub fn keys(&self) -> impl Iterator<Item = EncodedString> + '_ {
        self.inner.iter().map(|kv| kv.key.clone())
    }

    // print a table in whatever state we happen to be in (encoded/unencoded)
    fn impl_print_table(
        &self,
        headers: bool,
        decode: bool,
        only_keys: bool,
        only_values: bool,
    ) -> Result<()> {
        // TODO: we may have to add ways to elide long keys or values with ... after some char count
        let mut tw = TabWriter::new(Vec::new());

        if headers {
            match [only_keys, only_values] {
                [true, false] => writeln!(tw, "KEY")?,
                [false, true] => writeln!(tw, "VALUE")?,
                [..] => writeln!(tw, "KEY\tVALUE")?,
            }
        }

        for kv in self.iter() {
            if !only_values {
                if decode {
                    tw.write_all(&kv.key.decoded()?)?;
                } else {
                    write!(tw, "{}", &kv.key)?;
                }

                if !only_keys {
                    tw.write_all(b"\t")?
                };
            };

            if !only_keys {
                if decode {
                    tw.write_all(&kv.value.decoded()?)?;
                } else {
                    write!(tw, "{}", &kv.value)?;
                }
            };
            writeln!(tw)?;
        }
        tw.flush()?;

        let mut ptr = printer();
        let page = tw
            .into_inner()
            .map_err(|_| CliError::bail("IO flush error writing key-values"))?;
        ptr.write_all(&page)?;
        ptr.flush()?;

        Ok(())
    }
}

impl Output for KeyValues {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    fn print_table(&self, ctx: &Ctx) -> Result<()> {
        let mdctx = ctx.md_ctx.get_or_init();
        self.impl_print_table(!mdctx.no_header, mdctx.decode, mdctx.no_values, mdctx.no_keys)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn build_kvs() -> KeyValues {
        KeyValues {
            inner: vec![
                KeyValue {
                    key: EncodedString::new("a2V5MQ".into()),
                    value: EncodedString::new("dmFsdWUx".into()),
                },
                KeyValue {
                    key: EncodedString::new("a2V5Mg".into()),
                    value: EncodedString::new("dmFsdWUy".into()),
                },
                KeyValue {
                    key: EncodedString::new("a2V5Mw".into()),
                    value: EncodedString::new("dmFsdWUz".into()),
                },
            ],
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
}
