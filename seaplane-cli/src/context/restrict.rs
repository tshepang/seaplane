use seaplane::api::v1::restrict::RestrictedDirectory;

use crate::{cli::cmds::restrict::SeaplaneRestrictCommonArgMatches, error::Result};

/// Represents the "Source of Truth" i.e. it combines all the CLI options, ENV vars, and config
/// values into a single structure that can be used later to build models for the API or local
/// structs for serializing
// TODO: we may not want to derive this we implement circular references
#[derive(Debug, Default, Clone)]
pub struct RestrictCtx {
    pub api: Option<String>,
    /// A base64 encoded directory
    pub directory: Option<RestrictedDirectory>,
    pub from_api: Option<String>,
    /// A base64 encoded key
    pub from: Option<String>,
    /// Is the directory already URL safe base64 encoded
    pub base64: bool,
    /// Print with decoding
    pub decode: bool,
    /// Skip the headers in --format=table
    pub no_header: bool,
}

impl RestrictCtx {
    /// Builds a RestictCtx from ArgMatches
    pub fn from_restrict_common(matches: &SeaplaneRestrictCommonArgMatches) -> Result<RestrictCtx> {
        let matches = matches.0;
        let base64 = matches.contains_id("base64");
        let raw_api = matches.get_one::<String>("api").unwrap();
        let raw_dir = matches.get_one::<String>("directory").unwrap();

        Ok(RestrictCtx {
            api: Some(raw_api.into()),
            directory: if base64 {
                // Check that what the user passed really is valid base64
                let _ = base64::decode_config(raw_dir, base64::URL_SAFE_NO_PAD)?;
                Some(RestrictedDirectory::from_encoded(raw_dir))
            } else {
                Some(RestrictedDirectory::from_unencoded(raw_dir))
            },
            base64: true, // At this point all keys and values should be encoded as base64
            ..RestrictCtx::default()
        })
    }
}
