use clap::{value_parser, Arg, ArgMatches};

use crate::ops::DisplayEncodingFormat;

const LONG_DECODE: &str = "Decode the keys and values before printing them

WARNING!
By default the display encoding is `simple` which if the keys or values contain binary data this
can mess with your terminal! Use `--display-encoding=hex` or `--display-encoding=utf8` if your
values may contain binary data.";

const LONG_DISP_ENCODE_FMT: &str = "What format to display the decoded (--decode) keys/values

WARNING!
If the value contains binary data using `--display-encoding=simple` can mess with your terminal!

WARNING!
When using `--display-encoding=simple` or `--display-encoding=utf8` along with `--format=json` the
result can be invalid JSON if your keys or values contain unescaped characters that are not valid
for a JSON string. In these cases, unless you're sure your keys and values only contain valid JSON
string data, you should either use `--display-encoding=hex` or leave the values in their base64
format by omitting `--decode` (or use `--no-decode`)

simple => No encoding, just display as is
utf8   => Lossily encode to UTF-8. Invalid UTF-8 sequences will be converted to U+FFFD REPLACEMENT
          CHARACTER which looks like this \u{FFFD}
hex    => Raw bytes will be hex encoded and displayed as text";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
#[derive(Debug)]
pub struct SeaplaneMetadataCommonArgMatches<'a>(pub &'a ArgMatches);

pub fn args() -> Vec<Arg<'static>> {
    vec![keys(), base64()]
}

pub fn display_args() -> Vec<Arg<'static>> {
    vec![
        arg!(--decode - ('D'))
            .help("Decode the keys and values before printing them (WARNING! See --help)")
            .long_help(LONG_DECODE)
            .overrides_with("no-decode"),
        arg!(--("display-encoding") -('E') =["KIND"=>"simple"])
            .ignore_case(true)
            .value_parser(value_parser!(DisplayEncodingFormat))
            .long_help(LONG_DISP_ENCODE_FMT)
            .help(
                "What format to display the decoded (--decode) keys/values (WARNING! See --help)",
            ),
        arg!(--("no-decode"))
            .help("Print keys and values without decoding them")
            .overrides_with("decode"),
        arg!(--("no-header") | ("no-heading") | ("no-headers") - ('H'))
            .help("Omit the 'KEY' or 'VALUE' heading when printing with `--format=table`"),
        arg!(--("only-values") | ("only-value")).help("Only print the value"),
        arg!(--("only-keys") | ("only-key")).help("Only print the key"),
    ]
}

pub fn base64() -> Arg<'static> {
    arg!(--base64 - ('B')).help("The keys/values are already encoded in URL safe Base64")
}

pub fn single_key() -> Arg<'static> {
    arg!(key =["KEY"] required ).help("The key of the metadata key-value pair")
}

pub fn keys() -> Arg<'static> {
    arg!(key =["KEY"]... required ).help("The key(s) of the metadata key-value pair")
}
