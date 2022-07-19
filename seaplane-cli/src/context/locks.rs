use crate::{
    cli::cmds::locks::SeaplaneLocksCommonArgMatches,
    error::Result,
    ops::{locks::LockName, DisplayEncodingFormat},
};
use seaplane::api::v1::LockId;

/// Represents the "Source of Truth" i.e. it combines all the CLI options, ENV vars, and config
/// values into a single structure that can be used later to build models for the API or local
/// structs for serializing
#[derive(Debug, Default, Clone)]
pub struct LocksCtx {
    pub lock_name: Option<LockName>,
    pub ttl: Option<u32>,
    pub client_id: Option<String>,
    pub lock_id: Option<LockId>,
    /// Is the lock-name already URL safe base64 encoded
    pub base64: bool,
    /// Print with decoding
    pub decode: bool,
    /// What format to display decoded values in
    pub disp_encoding: DisplayEncodingFormat,
    /// Skip the KEY or VALUE header in --format=table
    pub no_header: bool,
}

impl LocksCtx {
    /// Builds a LocksCtx from ArgMatches
    pub fn from_locks_common(matches: &SeaplaneLocksCommonArgMatches) -> Result<LocksCtx> {
        let matches = matches.0;
        let base64 = matches.is_present("base64");
        let raw_lock_name = matches.value_of("lock_name");

        let lock_name: Option<LockName> = if base64 {
            let res: Option<Result<LockName>> = raw_lock_name.map(|name| {
                // Check that what the user passed really is valid base64
                let _ = base64::decode_config(name, base64::URL_SAFE_NO_PAD)?;
                Ok::<LockName, _>(LockName::new(name))
            });
            res.transpose()?
        } else {
            raw_lock_name.map(LockName::from_name_unencoded)
        };

        Ok(LocksCtx {
            lock_name,
            base64: true, // At this point all keys and values should be encoded as base64
            ..LocksCtx::default()
        })
    }
}
