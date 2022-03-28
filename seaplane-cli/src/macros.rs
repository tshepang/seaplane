macro_rules! _print {
    (@$color:ident, $ptr:ident, $($args:tt)+) => {{
        use ::std::io::Write;

        let mut ptr = $crate::printer::$ptr();
        ptr.set_color($crate::printer::Color::$color);
        let _ = ::std::write!(ptr, $($args)+);
        ptr.reset();
    }};
    ($ptr:ident, $($args:tt)+) => {{
        use ::std::io::Write;

        let _ = ::std::write!($crate::printer::$ptr(), $($args)+);
    }};
}

// Print is akin to info! level messages
macro_rules! cli_print {
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Info {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Info {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to info! level messages
macro_rules! cli_println {
    (@$color:ident, $($args:tt)+) => {{
        cli_print!(@$color, $($args)+);
        cli_print!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_print!($($args)+);
        cli_print!("\n");
    }}
}

// akin to error! level messages
macro_rules! cli_eprint {
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Error {
            _print!(@$color, eprinter, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Error {
            _print!(eprinter, $($args)+);
        }
    }}
}

// Akin to error! level messages
macro_rules! cli_eprintln {
    (@$color:ident, $($args:tt)+) => {{
        cli_eprint!(@$color, $($args)+);
        cli_eprint!("\n");
    }};
    ($($args:tt)*) => {{
        cli_eprint!($($args)+);
        cli_eprint!("\n");
    }}
}

/// Writes an error message to stderr and exits the process
#[allow(unused_macros)]
macro_rules! cli_bail {
    (@impl $prefix:expr, $status:expr, $($args:tt)*) => {{
        cli_eprint!(@Red, $prefix);
        cli_eprintln!($($args)+);
        ::std::process::exit($status);
    }};
    (@prefix $prefix:expr, @status $status:expr, $($args:tt)+) => {{
        cli_bail!(@impl $prefix, $status, $($args)+);
    }};
    (@status $status:expr, $($args:tt)+) => {{
        cli_bail!(@impl "error: ", $status, $($args)+);
    }};
    (@prefix $prefix:expr, $($args:tt)+) => {{
        cli_bail!(@impl $prefix, 1, $($args)+);
    }};
    ($($args:tt)*) => {{
        cli_bail!(@impl "error: ", 1, $($args)+);
    }};
}

// Akin to warn! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_warn {
    (@prefix, @$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(@Yellow, printer, "warn: ");
            _print!(@$color, printer, $($args)+);
        }
    }};
    (@prefix, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(printer, "warn: ");
            _print!(printer, $($args)+);
        }
    }};
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Warn {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to warn! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_warnln {
    (@noprefix, @$color:ident, $($args:tt)+) => {{
        cli_warn!(@$color, $($args)+);
        cli_warn!("\n");
    }};
    // TODO: change to zero or more (*)
    (@noprefix, $($args:tt)+) => {{
        cli_warn!($($args)+);
        cli_warn!("\n");
    }};
    (@$color:ident, $($args:tt)+) => {{
        cli_warn!(@prefix, @$color, $($args)+);
        cli_warn!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_warn!(@prefix, $($args)+);
        cli_warn!("\n");
    }}
}

// Akin to debug! level messages
//
// The *ln variants it's more common to want a oneshot message with a
// "warn: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_debug {
    (@prefix, @$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(@$color, printer, "DEBUG: ");
            _print!(@$color, printer, $($args)+);
        }
    }};
    (@prefix, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(printer, "DEBUG: ");
            _print!(printer, $($args)+);
        }
    }};
    (@$color:ident, $($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(@$color, printer, $($args)+);
        }
    }};
    ($($args:tt)+) => {{
        if $crate::log::log_level() <= &$crate::log::LogLevel::Debug {
            _print!(printer, $($args)+);
        }
    }};
}

// Akin to the debug! level messages.
//
// The *ln variants it's more common to want a oneshot message with a
// "DEBUG: " prefix, so that's the default. You opt out of the prefix with `@noprefix`. The non-line
// versions are the opposite, because it's more common to *not* want a prefix i.e. you're writing
// multiple portions of the same line.
macro_rules! cli_debugln {
    (@noprefix, @$color:ident, $($args:tt)+) => {{
        cli_debug!(@$color, $($args)+);
        cli_debug!("\n");
    }};
    // TODO: change to zero or more (*)
    (@noprefix, $($args:tt)+) => {{
        cli_debug!($($args)+);
        cli_debug!("\n");
    }};
    (@$color:ident, $($args:tt)+) => {{
        cli_debug!(@prefix, @$color, $($args)+);
        cli_debug!("\n");
    }};
    // TODO: change to zero or more (*)
    ($($args:tt)+) => {{
        cli_debug!(@prefix, $($args)+);
        cli_debug!("\n");
    }}
}

// Helper macros to make some CLI aspects a little less verbose

/// Converts a value from a clap::ArgMatches into some Result<T, CliError>
///
/// This macro uses clap's [`clap::ArgMatches::value_of_t`] but matches the error if any and
/// converts to our own [`crate::error::CliError`]
macro_rules! value_t {
    ($m:ident, $v:expr, $ty:ty) => {{
        match $m.value_of_t::<$ty>($v) {
            Ok(val) => Ok(val),
            Err(e) => match e.kind() {
                ::clap::ErrorKind::ValueValidation => {
                    if let ::clap::error::ContextValue::String(s) = e.context().nth(0).unwrap().1 {
                        Err(
                            $crate::error::CliErrorKind::InvalidCliValue(Some($v), s.to_string())
                                .into_err(),
                        )
                    } else {
                        // clap only returns a single string context value for ErrorKind::ValueValidation
                        // the pattern is actually irrefutable
                        unreachable!()
                    }
                }
                ::clap::ErrorKind::ArgumentNotFound => {
                    Err($crate::error::CliErrorKind::CliArgNotUsed($v).into_err())
                }
                // `clap::ArgMatches::value_of_t` does not return any other type of error but the
                // above two
                _ => unreachable!(),
            },
        }
    }};
}

/// Collects the values out of an ArgMatches parsing them into some T or exiting if any value fails
/// to parse. Unlike ArgMatches::values_of_t_or_exit this macro does not require the argument to
/// have been used.
macro_rules! values_t_or_exit {
    (@into_model $m:ident, $v:expr, $ty:ty) => {{
        $m.values_of($v)
            .map(|vals| {
                vals.filter_map(|s| {
                    // clap already validates that these values are valid
                    let p = &s.parse::<$ty>().unwrap();
                    <$ty>::into_model(p)
                })
                .collect()
            })
            .unwrap_or_default()
    }};
    ($m:ident, $v:expr, $ty:ty) => {{
        $m.values_of($v)
            .map(|vals| {
                vals.map(|s| {
                    // clap already validates that these values are valid
                    s.parse::<$ty>().unwrap()
                })
                .collect()
            })
            .unwrap_or_default()
    }};
}

/// Makes declaring *consistent* arguments less verbose and less tedious.
///
/// The available syntax is:
///
/// - `--STRING` or `--("STRING-WITH-HYPHENS")` will make an `Arg` where *both* the name and long
/// are the same. Due to Rust syntax, if the argument should have hyphens, one must use
/// `--("foo-bar-baz")`
/// - `-('f')` sets the Short value. (Due to Rust syntax rules)
/// - Visible aliases can be set with using `|` along with the similar Long value rules. I.e. `|foo` or
/// `|("foo-with-hyphens"). When combined the Long/name it actually looks good `--foo|bar`, etc.
/// - A value name can be set with `=["STRING"]` optionally also setting a default value `=["STRING"=>"default"]`
/// - Setting multiple values can be done with `...` Note that this sets multiple
/// values/occurrences in a consistent manner for this application. If you need arguments with
/// different semantics you'll have to set those manually. `...` is equivalent to setting
/// `Arg::new("foo").multiple_values(true).multiple_occurrences(true).number_of_values(1).value_delimiter(',')`
/// - Setting any boolean value to `true` can be done by just the function name i.e. `required`
/// - Setting any boolean value to `false` can be done by prefixing the function with `!` i.e.
/// `!required`
///
/// ```rust
/// # use clap::Arg;
/// # use seaplane_cli::macros::arg;
/// # let _ =
/// arg!(--foo|foos =["NUM"=>"2"]... global !allow_hyphen_values);
///
/// // is equivalent to (with the macro syntax in the comment to the right)...
///# let _ =
/// Arg::new("foo")                // --foo
///   .long("foo")                 // --foo
///   .visible_alias("foos")       // |foos
///   .value_name("NUM")           // =["NUM"]
///   .default_value("2")          // =[..=>"2"]
///   .multiple_values(true)       // ...
///   .multiple_occurrences(true)  // ...
///   .value_delimiter(',')        // ...
///   .number_of_values(1)         // ...
///   .global(true)                // global
///   .allow_hyphen_values(false); // !allow_hyphen_values
/// ```
macro_rules! arg {
    (@arg ($arg:expr) ) => { $arg };
    (@arg ($arg:expr) --$long:ident $($tail:tt)*) => {
        arg!(@arg ($arg.long(stringify!($long))) $($tail)* )
    };
    (@arg ($arg:expr) -($short:expr) $($tail:tt)*) => {
        arg!(@arg ($arg.short($short)) $($tail)* )
    };
    (@arg ($arg:expr) | ($alias:expr) $($tail:tt)*) => {
        arg!(@arg ($arg.visible_alias($alias)) $($tail)* )
    };
    (@arg ($arg:expr) | $alias:ident $($tail:tt)*) => {
        arg!(@arg ($arg.visible_alias(stringify!($alias))) $($tail)* )
    };
    (@arg ($arg:expr) ... $($tail:tt)*) => {
        arg!(@arg ($arg.number_of_values(1).value_delimiter(',')) multiple_values multiple_occurrences $($tail)* )
    };
    (@arg ($arg:expr) =[$var:expr$(=>$default:expr)?] $($tail:tt)*) => {
        arg!(@arg ({
            #[allow(unused_mut)]
            let mut a = $arg.value_name($var);
            $(
                a = a.default_value($default);
            )?
            a
            }) $($tail)*)
    };
    // !foo -> .foo(false)
    (@arg ($arg:expr) !$ident:ident $($tail:tt)*) => {
        arg!(@arg ($arg.$ident(false)) $($tail)*)
    };
    // +foo -> .foo(true)
    (@arg ($arg:expr) $ident:ident $($tail:tt)*) => {
        arg!(@arg ($arg.$ident(true)) $($tail)*)
    };
    ($name:ident $($tail:tt)*) => {
        arg!(@arg (::clap::Arg::new(stringify!($name))) $($tail)* )
    };
    (--($name:expr) $($tail:tt)*) => {
        arg!(@arg (::clap::Arg::new($name).long($name)) $($tail)* )
    };
    (--$name:ident $($tail:tt)*) => {
        arg!(@arg (::clap::Arg::new(stringify!($name)).long(stringify!($name))) $($tail)* )
    };
}

/// Shorthand for checking if an argument in the key-value commands was base64 or not, and doing
/// the conversion if necessary
macro_rules! maybe_base64_arg {
    ($m:expr, $arg:expr, $is_base64:expr) => {
        if let Some(raw_key) = $m.value_of($arg) {
            if $is_base64 {
                let _ = ::base64::decode_config(raw_key, ::base64::URL_SAFE_NO_PAD)?;
                Some(raw_key.to_owned())
            } else {
                Some(::base64::encode_config(raw_key, ::base64::URL_SAFE_NO_PAD))
            }
        } else {
            None
        }
    };
}
