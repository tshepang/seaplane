use clap::{Arg, ArgMatches};

const LONG_DECODE: &str = "Decode the directories before printing them

Binary values will be written directly to standard output (which may do strange
things to your terminal)";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces
/// errors in checking if values of arguments were used or not. i.e. `seaplane
/// formation create` may not have the same arguments as `seaplane account
/// token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
#[derive(Debug)]
pub struct SeaplaneRestrictCommonArgMatches<'a>(pub &'a ArgMatches);

pub fn display_args() -> Vec<Arg<'static>> {
    vec![
        arg!(--decode - ('D'))
            .help("Decode the directories before printing them")
            .long_help(LONG_DECODE)
            .overrides_with("no-decode"),
        arg!(--("no-decode"))
            .help("Print directories without decoding them")
            .overrides_with("decode"),
        arg!(--("no-header") | ("no-heading") | ("no-headers") - ('H'))
            .help("Omit the 'KEY' or 'VALUE' heading when printing with `--format=table`"),
        arg!(--("only-values") | ("only-value")).help("Only print the value"),
        arg!(--("only-keys") | ("only-key")).help("Only print the key"),
    ]
}

pub fn base64() -> Arg<'static> {
    arg!(--base64 - ('B')).help("The directory is already encoded in URL safe Base64")
}

pub fn api() -> Arg<'static> {
    arg!(api =["API"] required ).help("The API of the restricted directory")
}

pub fn directory() -> Arg<'static> {
    arg!(directory =["DIRECTORY"] required ).help("The restricted directory")
}
