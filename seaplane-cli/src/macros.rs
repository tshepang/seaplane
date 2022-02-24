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

macro_rules! cli_print {
    (@$color:ident, $($args:tt)+) => {{
        _print!(@$color, printer, $($args)+)
    }};
    ($($args:tt)+) => {{
        _print!(printer, $($args)+)
    }};
}

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

macro_rules! cli_eprint {
    (@$color:ident, $($args:tt)+) => {{
        _print!(@$color, eprinter, $($args)+)
    }};
    ($($args:tt)+) => {{
        _print!(eprinter, $($args)+)
    }}
}

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
