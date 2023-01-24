mod error;

use std::{fmt, str::FromStr};

use uuid::Uuid;

use crate::error::{Error, Result};

macro_rules! base32_spec {
    () => {{
        static BASE32_SPEC: once_cell::sync::OnceCell<data_encoding::Encoding> =
            once_cell::sync::OnceCell::new();
        BASE32_SPEC.get_or_init(|| {
            let mut spec = data_encoding::Specification::new();
            spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz234567");
            spec.encoding().unwrap()
        })
    }};
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Prefix<const N: usize = 3> {
    bytes: [u8; N],
}

impl<const N: usize> fmt::Display for Prefix<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: self.bytes must not contain any invalid UTF-8. We don't expose the inner byte
        // array for manipulation, and the only way to construct self checks for a subset of ASCII
        // which itself is a subset of UTF-8
        unsafe { write!(f, "{}", std::str::from_utf8_unchecked(self.bytes.as_slice())) }
    }
}

impl<const N: usize> Prefix<N> {
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        // Checking for ASCII 2-7,a-z
        if !slice
            .iter()
            .all(|&c| (c > b'1' && c < b'8') || (c > b'`' && c < b'{'))
        {
            return Err(Error::InvalidPrefixChar);
        }
        if slice.len() != N {
            return Err(Error::PrefixByteLength);
        }
        let mut pfx = Prefix { bytes: [0_u8; N] };
        pfx.bytes.copy_from_slice(slice);
        Ok(pfx)
    }
}

impl<const N: usize> FromStr for Prefix<N> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::from_slice(s.to_ascii_lowercase().as_bytes())
    }
}

impl<const N: usize> From<[u8; N]> for Prefix<N> {
    fn from(arr: [u8; N]) -> Self { Self { bytes: arr } }
}

impl<const N: usize> TryFrom<&[u8]> for Prefix<N> {
    type Error = Error;
    fn try_from(slice: &[u8]) -> std::result::Result<Self, Self::Error> { Self::from_slice(slice) }
}

impl<const N: usize> TryFrom<&str> for Prefix<N> {
    type Error = Error;
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> { s.parse() }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;

    #[test]
    fn prefix_from_str() {
        let pfx = "frm".parse::<Prefix>();
        assert!(pfx.is_ok());
        assert_eq!(pfx.unwrap(), Prefix { bytes: [b'f', b'r', b'm'] });
    }

    #[test]
    fn prefix_from_str_err_len() {
        let pfx = "frmx".parse::<Prefix<3>>();
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::PrefixByteLength);
    }

    #[test]
    fn prefix_from_str_err_char() {
        let pfx = "fr[".parse::<Prefix<3>>();
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::InvalidPrefixChar);
    }

    #[test]
    fn prefix_from_str_uppercase_ok() {
        let pfx = "frM".parse::<Prefix>();
        assert!(pfx.is_ok());
        assert_eq!(pfx.unwrap(), Prefix { bytes: [b'f', b'r', b'm'] });
    }

    #[test]
    fn prefix_from_slice() {
        let arr: [u8; 3] = [b'f', b'r', b'm'];
        let pfx = Prefix::from_slice(arr.as_slice());
        assert!(pfx.is_ok());
        assert_eq!(pfx.unwrap(), Prefix { bytes: arr });
    }

    #[test]
    fn prefix_from_slice_err_len() {
        let arr: [u8; 4] = [b'f', b'r', b'm', b'x'];
        let pfx = Prefix::<3>::from_slice(arr.as_slice());
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::PrefixByteLength);
    }

    #[test]
    fn prefix_from_slice_err_char() {
        let arr: [u8; 3] = [b'f', b'r', b']'];
        let pfx = Prefix::<3>::from_slice(arr.as_slice());
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::InvalidPrefixChar);
    }

    #[test]
    fn prefix_from_slice_err_uppercase() {
        let arr: [u8; 3] = [b'f', b'r', b'M'];
        let pfx = Prefix::<3>::from_slice(arr.as_slice());
        assert!(pfx.is_err());
        assert_eq!(pfx.unwrap_err(), Error::InvalidPrefixChar);
    }

    #[test]
    fn prefix_to_string() {
        let pfx: Prefix = "frM".parse().unwrap();
        assert_eq!("frm".to_string(), pfx.to_string());
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Oid {
    prefix: Prefix<3>,
    uuid: Uuid,
}

impl Oid {
    pub fn new<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid_v7(prefix, Uuid::now_v7())
    }

    pub fn with_uuid_v7<P>(prefix: P, uuid: Uuid) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        if uuid.get_version_num() != 7 {
            return Err(Error::UnsupportedUuidVersion);
        }

        Ok(Self { prefix: prefix.try_into()?, uuid })
    }

    pub fn prefix(&self) -> Prefix { self.prefix }

    pub fn value(&self) -> String { base32_spec!().encode(self.uuid.as_bytes()) }

    pub fn uuid_v7(&self) -> &Uuid { &self.uuid }
}

impl FromStr for Oid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((pfx, val))= s.split_once('-') {
            if pfx.is_empty() {
                return Err(Error::MissingPrefix);
            }
            if val.is_empty() {
                return Err(Error::MissingValue);
            }

            let uuid = Uuid::from_slice(&base32_spec!().decode(val.as_bytes())?)?;
            if uuid.get_version_num() != 7 {
                return Err(Error::UnsupportedUuidVersion);
            }

            return Ok(Self {
                prefix: pfx.parse()?,
                uuid
            });

        }

        Err(Error::MissingSeparator)
    }

}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.prefix, self.value())
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for Oid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::ser::Serializer {
            serializer.collect_str(self)
        }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?.parse().map_err(::serde::de::Error::custom)
    }
}

#[cfg(test)]
mod oid_tests {
    use super::*;
    use wildmatch::WildMatch;

    #[test]
    fn oid_to_str() -> Result<()>{
        let oid = Oid::new("tst")?;
        dbg!(oid.to_string());
        assert!(WildMatch::new("tst-??????????????????????????").matches(&oid.to_string()));
        Ok(())
    }

    #[test]
    fn str_to_oid() {
        let res = "tst-agc6amh7z527vijkv2cutplwaa".parse::<Oid>();
        assert_eq!(res.unwrap(), Oid {
            prefix: "tst".parse().unwrap(),
            uuid: "0185e030-ffcf-75fa-a12a-ae8549bd7600".parse().unwrap(),
        });
    }

    #[test]
    fn str_to_oid_err_prefix() {
        let res = "-agc6amh7z527vijkv2cutplwaa".parse::<Oid>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingPrefix);
    }

    #[test]
    fn str_to_oid_err_value() {
        let res = "tst-".parse::<Oid>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingValue);
    }

    #[test]
    fn str_to_oid_err_decode() {
        let res = "tst-&gc6amh7z527vijkv2ctplwaa".parse::<Oid>();
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::Base32Decode(_)));
    }

    #[test]
    fn str_to_oid_err_no_sep() {
        let res = "agc6amh7z527vijkv2cutplwaa".parse::<Oid>();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::MissingSeparator);
    }

    #[test]
    fn str_to_oid_err_two_sep() {
        let res = "tst-agc6amh7z-527vijkv2ctplwaa".parse::<Oid>();
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), Error::Base32Decode(_)));
    }

    #[test]
    fn str_to_oid_err_wrong_uuid_ver() {
        let res = Oid::with_uuid_v7("tst", "3dedbec5-dbc3-43f1-9407-2389aac1fd81".parse::<Uuid>().unwrap());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::UnsupportedUuidVersion);
    }

    #[test]
    fn oid_to_uuid() {
        let oid: Oid = "tst-agc6amh7z527vijkv2cutplwaa".parse().unwrap();
        assert_eq!(oid.uuid_v7(), &"0185e030-ffcf-75fa-a12a-ae8549bd7600".parse::<Uuid>().unwrap());
    }
}


