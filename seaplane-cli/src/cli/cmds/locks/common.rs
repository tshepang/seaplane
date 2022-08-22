use clap::{value_parser, Arg, ArgMatches};

const LONG_DECODE: &str = "Decode the lock name before printing it

Binary values will be written directly to standard output (which may do strange
things to your terminal)";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
pub struct SeaplaneLocksCommonArgMatches<'a>(pub &'a ArgMatches);

pub fn display_args() -> Vec<Arg<'static>> {
    vec![
        arg!(--decode - ('D'))
            .help("Decode the lockname before printing it (WARNING! See --help)")
            .long_help(LONG_DECODE)
            .overrides_with("no-decode"),
        arg!(--("no-decode"))
            .help("Print lockname without decoding it")
            .overrides_with("decode"),
        arg!(--("no-header") | ("no-heading") | ("no-headers"))
            .help("Omit the heading when printing with `--format=table`"),
    ]
}

pub fn base64() -> Arg<'static> {
    arg!(--base64 - ('B')).help("The lockname is already encoded in URL safe Base64")
}

pub fn ttl() -> Arg<'static> {
    arg!(--ttl - ('T') =["SECS"] required)
        .value_parser(value_parser!(u32))
        .help("The TTL (Time To Live) in seconds, i.e. a positive integer")
}

pub fn lock_id() -> Arg<'static> {
    arg!(--("lock-id") - ('L') =["STRING"] required).help(
        "A valid lock-id can be obtained from a successful acquisition, or listing of the locks",
    )
}

pub fn lock_name() -> Arg<'static> {
    arg!(lock_name =["LOCK_NAME"] required ).help("The name of the lock")
}
