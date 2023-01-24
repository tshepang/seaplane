//! An Object ID (OID) is a base32 (no padding) encoded UUIDv7 with a prefix
//! separated by a `-`.
//!
//! For example `tst-agc6amh7z527vijkv2cutplwaa`, by convention the prefix is three
//! ASCII lowercase characters, however that is a hard constraint of OIDs in
//! general. The current implementation limits prefixes to 3 characters, but prefix
//! limit could be exposed as a tunable if that need arises.
//!
//! ## The Pitch
//!
//! OIDs allow a "human readable subject line" in the form of the prefix, where
//! actual data (the UUID) is sortable by timestamp. This means while debugging or
//! reviewing a system it's trivial to determine if an incorrect OID was passed in
//! a particular location by looking at the prefix. This isn't easily achievable
//! with bare UUIDs.
//!
//! Base32 encoding the UUID also allows compressing the data into a smaller and
//! more familiar format for humans, akin to a commit hash.
//!
//! Finally, the actual data itself, UUIDv7 has the property of being sortable in
//! the database by timestamp.
//!
//! ## The Anti-Pitch
//!
//! The downside to OIDs is a layer of indirection when handling IDs and values,
//! it's not immediately obvious that the OIDs are a prefixed UUIDv7. Additionally,
//! the prefixes must be controlled in some manner including migrated on changes
//! which adds a layer of complexity at the application layer.
//!
//! There is also additional processing overhead compared to a bare UUIDv7 in order
//! to encode/decode as well as handling the appending and removing the prefixes.
//!
//! However, we believe the drawbacks to pale in comparison to the benefits derived
//! from the format.
//!
//! ## Example
//!
//! ```rust
//! use seaplane_oid::{error::*, Oid};
//! use uuid::Uuid;
//!
//! fn main() -> Result<()> {
//!     // OIDs can be created with a given prefix alone, which generates a new
//!     // UUIDv7 using the current unix timestamp
//!     let oid = Oid::new("exm")?;
//!     println!("{oid}");
//!
//!     // OIDs can be parsed from strings, however the "value" must be a valid
//!     // base32 encoded UUIDv7
//!     let oid: Oid = "tst-agc6amh7z527vijkv2cutplwaa".parse()?;
//!     println!("{oid}");
//!
//!     // OIDs can also be created from the raw parts
//!     let oid = Oid::with_uuid_v7(
//!         "exm",
//!         "0185e030-ffcf-75fa-a12a-ae8549bd7600"
//!             .parse::<Uuid>()
//!             .unwrap(),
//!     )?;
//!
//!     // One can retrieve the various parts of the OID if needed
//!     println!("Prefix: {}", oid.prefix());
//!     println!("Value: {}", oid.value());
//!     println!("UUIDv7: {}", oid.uuid_v7());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! Licensed under the Apache License, Version 2.0, Copyright 2023 Seaplane IO, Inc.
pub mod error;

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

/// An OID Prefix designed to be similar to a human readable "subject line" for the ID
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
    /// Create a Prefix from a slice of bytes. The bytes must be lowercase ASCII values of `0-9` or
    /// `a-z`, additionally the byte slice length must be equal to the prefix length.
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        // Checking for ASCII 0-9,a-z
        if !slice
            .iter()
            .all(|&c| (c > b'/' && c < b':') || (c > b'`' && c < b'{'))
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

    /// The string slice is converted to ASCII lowercase before creating the Prefix
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

/// An Object ID
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Oid {
    prefix: Prefix<3>,
    uuid: Uuid,
}

impl Oid {
    /// Create a new OID with a given [`Prefix`] and generating a new UUIDv7 with the current unix
    /// timestamp
    ///
    /// **NOTE:** The Prefix must be 3 ASCII characters (this restriction is arbitrary and could be
    /// lifted in the future by exposing an API to tune the [`Prefix`] length)
    pub fn new<P>(prefix: P) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        Self::with_uuid_v7(prefix, Uuid::now_v7())
    }

    /// Create a new OID with a given [`Prefix`] and a given UUIDv7. If the UUID is not a version 7
    /// an error isr returned.
    ///
    /// **NOTE:** The Prefix must be 3 ASCII characters (this restriction is arbitrary and could be
    /// lifted in the future by exposing an API to tune the [`Prefix`] length)
    pub fn with_uuid_v7<P>(prefix: P, uuid: Uuid) -> Result<Self>
    where
        P: TryInto<Prefix, Error = Error>,
    {
        if uuid.get_version_num() != 7 {
            return Err(Error::UnsupportedUuidVersion);
        }

        Ok(Self { prefix: prefix.try_into()?, uuid })
    }

    /// Get the [`Prefix`] of the OID
    pub fn prefix(&self) -> Prefix { self.prefix }

    /// Get the value portion of the  of the OID, which is the base32 encoded string following the
    /// `-` separator
    pub fn value(&self) -> String { base32_spec!().encode(self.uuid.as_bytes()) }

    /// Get the UUID of the OID
    pub fn uuid_v7(&self) -> &Uuid { &self.uuid }
}

impl FromStr for Oid {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some((pfx, val)) = s.split_once('-') {
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

            return Ok(Self { prefix: pfx.parse()?, uuid });
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
        S: ::serde::ser::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for Oid {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(::serde::de::Error::custom)
    }
}

#[cfg(test)]
mod oid_tests {
    use wildmatch::WildMatch;

    use super::*;

    #[test]
    fn oid_to_str() -> Result<()> {
        let oid = Oid::new("tst")?;
        dbg!(oid.to_string());
        assert!(WildMatch::new("tst-??????????????????????????").matches(&oid.to_string()));
        Ok(())
    }

    #[test]
    fn str_to_oid() {
        let res = "tst-agc6amh7z527vijkv2cutplwaa".parse::<Oid>();
        assert_eq!(
            res.unwrap(),
            Oid {
                prefix: "tst".parse().unwrap(),
                uuid: "0185e030-ffcf-75fa-a12a-ae8549bd7600".parse().unwrap(),
            }
        );
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
        let res = Oid::with_uuid_v7(
            "tst",
            "3dedbec5-dbc3-43f1-9407-2389aac1fd81"
                .parse::<Uuid>()
                .unwrap(),
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), Error::UnsupportedUuidVersion);
    }

    #[test]
    fn oid_to_uuid() {
        let oid: Oid = "tst-agc6amh7z527vijkv2cutplwaa".parse().unwrap();
        assert_eq!(
            oid.uuid_v7(),
            &"0185e030-ffcf-75fa-a12a-ae8549bd7600"
                .parse::<Uuid>()
                .unwrap()
        );
    }
}
