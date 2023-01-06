use clap::{builder::ArgPredicate, Arg, ArgGroup, ArgMatches};

const LONG_DECODE: &str = "Decode the keys and values before printing them

Binary values will be written directly to standard output (which may do strange
things to your terminal)";

const LONG_HUMAN_READABLE: &str = "Safely decode and truncate output for human readability

Implies --decode-safe --values-width-limit 256";

/// A newtype wrapper to enforce where the ArgMatches came from which reduces errors in checking if
/// values of arguments were used or not. i.e. `seaplane formation create` may not have the same
/// arguments as `seaplane account token` even though both produce an `ArgMatches`.
#[allow(missing_debug_implementations)]
#[derive(Debug)]
pub struct SeaplaneMetadataCommonArgMatches<'a>(pub &'a ArgMatches);

pub fn args() -> Vec<Arg> { vec![keys(), base64()] }

pub fn display_args() -> Vec<Arg> {
    vec![
        arg!(--("human-readable") - ('H'))
            .help("Safely decode and truncate output for human readability")
            .long_help(LONG_HUMAN_READABLE),
        arg!(--decode - ('D'))
            .help("Decode the keys and values before printing them")
            .long_help(LONG_DECODE)
            .overrides_with_all(["no-decode", "decode-safe"]),
        arg!(--("decode-safe"))
            .help("Decode the keys and values in a terminal-friendly way")
            .overrides_with_all(["decode", "no-decode"]),
        arg!(--("no-decode"))
            .help("Print keys and values without decoding them")
            .overrides_with_all(["decode", "decode-safe"]),
        arg!(--("no-header") | ("no-heading") | ("no-headers"))
            .help("Omit the 'KEY' or 'VALUE' heading when printing with `--format=table`"),
        arg!(--("only-values") | ("only-value")).help("Only print the value"),
        arg!(--("only-keys") | ("only-key")).help("Only print the key"),
        arg!(--("keys-width-limit") = ["LIMIT"])
            .help("Limit the width of the keys when using `--format=table` (0 means unlimited)")
            .value_parser(clap::value_parser!(usize)),
        arg!(--("values-width-limit") = ["LIMIT"])
            .default_value_if("human-readable", ArgPredicate::IsPresent, Some("256"))
            .help("Limit the width of the values when using `--format=table` (0 means unlimited)")
            .value_parser(clap::value_parser!(usize)),
    ]
}

pub fn base64() -> Arg {
    arg!(--base64 - ('B')).help("The keys/values are already encoded in URL safe Base64")
}

pub fn single_key() -> Arg {
    arg!(key =["KEY"] required ).help("The key of the metadata key-value pair")
}

pub fn keys() -> Arg {
    arg!(key =["KEY"]... required ).help("The key(s) of the metadata key-value pair")
}

pub fn keys_or_values() -> ArgGroup {
    ArgGroup::new("keys_or_values")
        .args(["only-keys", "only-values"])
        .multiple(false)
        .required(false)
}
