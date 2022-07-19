use clap::{Arg, ArgMatches};
use strum::VariantNames;

use crate::ops::DisplayEncodingFormat;

const LONG_DECODE: &str = "Decode the lock-id before printing it

WARNING!
By default the display encoding is `simple` which if the lock-id contains binary data this
can mess with your terminal! Use `--display-encoding=hex` or `--display-encoding=utf8` if your
lock-id may contain binary data.";

const LONG_DISP_ENCODE_FMT: &str = "What format to display the decoded (--decode) lock-id

WARNING!
If the value contains binary data using `--display-encoding=simple` can mess with your terminal!

WARNING!
When using `--display-encoding=simple` or `--display-encoding=utf8` along with `--format=json` the
result can be invalid JSON if your lock-id contains unescaped characters that are not valid for a
JSON string. In these cases, unless you're sure your keys and values only contain valid JSON string
data, you should either use `--display-encoding=hex` or leave the values in their base64 format by
omitting `--decode` (or use `--no-decode`)

simple => No encoding, just display as is
utf8   => Lossily encode to UTF-8. Invalid UTF-8 sequences will be converted to U+FFFD REPLACEMENT
          CHARACTER which looks like this \u{FFFD}
hex    => Raw bytes will be hex encoded and displayed as text";

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
        arg!(--("display-encoding") -('E') =["KIND"=>"simple"])
            .ignore_case(true)
            .possible_values(DisplayEncodingFormat::VARIANTS)
            .long_help(LONG_DISP_ENCODE_FMT)
            .help("What format to display the decoded (--decode) lockname (WARNING! See --help)"),
        arg!(--("no-decode"))
            .help("Print lockname without decoding it")
            .overrides_with("decode"),
        arg!(--("no-header") | ("no-heading") | ("no-headers") - ('H'))
            .help("Omit the heading when printing with `--format=table`"),
    ]
}

pub fn base64() -> Arg<'static> {
    arg!(--base64 - ('B')).help("The lockname is already encoded in URL safe Base64")
}

pub fn ttl() -> Arg<'static> {
    arg!(--ttl - ('T') =["SECS"] required)
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
