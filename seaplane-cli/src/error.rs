use std::{
    error::Error,
    io::{self, Write},
    result::Result as StdResult,
};

use seaplane::{api::v1::ImageReferenceError, error::SeaplaneError};

use crate::{
    log::{log_level, LogLevel},
    printer::{eprinter, Color},
};

pub type Result<T> = StdResult<T, CliError>;

/// A trait for adding context to an error that will be printed along with the error. Contexts are
/// useful for adding things such as hints (i.e. try --help), or additional information such as the
/// path name on a PermissionDenied error, etc.
///
/// **NOTE:** all contexts print *without* a trailing newline. This allows a context to print to
/// the same line in different formats (colors, etc.). If a trailing newline is required, you
/// should add it manually.
pub trait Context {
    /// A simple context
    fn context<S: Into<String>>(self, msg: S) -> Self;

    /// A context that is evaluated lazily when called. This is useful if building the context is
    /// expensive or allocates
    fn with_context<F, S>(self, f: F) -> Self
    where
        F: FnOnce() -> S,
        S: Into<String>;

    /// A simple context that will color the output
    ///
    /// **NOTE:** The color is reset at the end of this context even if there is no trailing
    /// newline. This allows you to chain multiple contexts on the same line where only part of the
    /// context is colored.
    fn color_context<S: Into<String>>(self, color: Color, msg: S) -> Self;

    /// A context that will color the output and that is evaluated lazily when called. This is
    /// useful if building the context is expensive or allocates
    ///
    /// **NOTE:** The color is reset at the end of this context even if there is no trailing
    /// newline. This allows you to chain multiple contexts on the same line where only part of the
    /// context is colored.
    fn with_color_context<F, S>(self, f: F) -> Self
    where
        F: FnOnce() -> (Color, S),
        S: Into<String>;
}

impl<T> Context for StdResult<T, CliError> {
    fn context<S: Into<String>>(self, msg: S) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(cli_err) => Err(cli_err.context(msg)),
        }
    }
    fn color_context<S: Into<String>>(self, color: Color, msg: S) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(cli_err) => Err(cli_err.color_context(color, msg)),
        }
    }
    fn with_context<F, S>(self, f: F) -> Self
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        match self {
            Ok(t) => Ok(t),
            Err(cli_err) => Err(cli_err.context(f())),
        }
    }

    fn with_color_context<F, S>(self, f: F) -> Self
    where
        F: FnOnce() -> (Color, S),
        S: Into<String>,
    {
        match self {
            Ok(t) => Ok(t),
            Err(cli_err) => {
                let (color, msg) = f();
                Err(cli_err.color_context(color, msg))
            }
        }
    }
}

#[derive(Debug)]
pub struct ColorString {
    msg: String,
    color: Option<Color>,
}

#[derive(Debug)]
pub struct CliError {
    kind: CliErrorKind,
    context: Vec<ColorString>,
    status: Option<i32>, // TODO: default to 1
    fatal: bool,         // TODO: default to true
}

impl CliError {
    pub fn bail(msg: &'static str) -> Self {
        Self {
            kind: CliErrorKind::UnknownWithContext(msg),
            ..Default::default()
        }
    }
}

impl Context for CliError {
    fn color_context<S: Into<String>>(mut self, color: Color, msg: S) -> Self {
        self.context.push(ColorString {
            msg: msg.into(),
            color: Some(color),
        });
        self
    }

    fn context<S: Into<String>>(mut self, msg: S) -> Self {
        self.context.push(ColorString {
            msg: msg.into(),
            color: None,
        });
        self
    }

    fn with_context<F, S>(mut self, f: F) -> Self
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        self.context.push(ColorString {
            msg: f().into(),
            color: None,
        });
        self
    }

    fn with_color_context<F, S>(mut self, f: F) -> Self
    where
        F: FnOnce() -> (Color, S),
        S: Into<String>,
    {
        let (color, msg) = f();
        self.context.push(ColorString {
            msg: msg.into(),
            color: Some(color),
        });
        self
    }
}

impl Default for CliError {
    fn default() -> Self {
        Self {
            kind: CliErrorKind::Unknown,
            context: Vec::new(),
            status: None,
            fatal: true,
        }
    }
}

// We have to impl Display so we can use the ? operator...but we don't actually want to use it's
// pipeline to do any kind of displaying because it doesn't support any sort of coloring. So we
// handle it manually.
impl std::fmt::Display for CliError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("std::fmt::Display is not actually implemented for CliError by design")
    }
}

// Just so we can us the ? operator
impl Error for CliError {}

macro_rules! impl_err {
    ($errty:ty, $variant:ident) => {
        impl From<$errty> for CliError {
            fn from(e: $errty) -> Self {
                CliError {
                    kind: CliErrorKind::$variant(e),
                    ..Default::default()
                }
            }
        }
    };
}

// These are placeholders until we get around to writing distinct errors for the cases we care
// about
impl_err!(serde_json::Error, SerdeJson);
impl_err!(toml::de::Error, TomlDe);
impl_err!(toml::ser::Error, TomlSer);
impl_err!(seaplane::error::SeaplaneError, Seaplane);
impl_err!(seaplane::api::v1::ImageReferenceError, ImageReference);

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => CliError {
                kind: CliErrorKind::MissingPath,
                ..Default::default()
            },
            io::ErrorKind::PermissionDenied => CliError {
                kind: CliErrorKind::PermissionDenied,
                ..Default::default()
            },
            _ => CliError {
                kind: CliErrorKind::Io(e),
                ..Default::default()
            },
        }
    }
}

impl From<CliErrorKind> for CliError {
    fn from(kind: CliErrorKind) -> Self {
        CliError {
            kind,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum CliErrorKind {
    DuplicateName(String),
    NoMatchingItem(String),
    AmbiguousItem(String),
    Io(io::Error),
    SerdeJson(serde_json::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    UnknownWithContext(&'static str),
    Seaplane(SeaplaneError),
    ExistingValue(&'static str),
    ImageReference(ImageReferenceError),
    MissingPath,
    Unknown,
    PermissionDenied,
    MissingApiKey,
    MultipleAtStdin,
}

impl CliErrorKind {
    fn print(&self) {
        use CliErrorKind::*;

        match &*self {
            DuplicateName(name) => {
                cli_eprint!("an item with the name '");
                cli_eprint!(@Yellow, "{}", name);
                cli_eprintln!("' already exists");
            }
            NoMatchingItem(item) => {
                cli_eprint!("the NAME or ID '");
                cli_eprint!(@Green, "{}", item);
                cli_eprintln!("' didn't match anything");
            }
            AmbiguousItem(item) => {
                cli_eprint!("the NAME or ID '");
                cli_eprint!(@Yellow, "{}", item);
                cli_eprintln!("' is ambiguous and matches more than one item");
            }
            MissingPath => {
                cli_eprintln!("missing file or directory");
            }
            PermissionDenied => {
                cli_eprintln!("permission denied when accessing file or directory");
            }
            ImageReference(e) => {
                cli_eprintln!("seaplane: {}", e)
            }
            Io(e) => {
                cli_eprintln!("io: {}", e)
            }
            SerdeJson(e) => {
                cli_eprintln!("json: {}", e)
            }
            TomlDe(e) => {
                cli_eprintln!("toml: {}", e)
            }
            TomlSer(e) => {
                cli_eprintln!("toml: {}", e)
            }
            UnknownWithContext(e) => {
                cli_eprintln!("unknown: {}", e)
            }
            Unknown => {
                cli_eprintln!("unknown")
            }
            MissingApiKey => {
                cli_eprintln!("no API key was found or provided")
            }
            MultipleAtStdin => {
                cli_eprint!("more than one '");
                cli_print!(@Yellow, "@-");
                cli_println!("' values were provided and only one is allowed");
            }
            Seaplane(e) => {
                cli_eprintln!("seaplane: {}", e)
            }
            ExistingValue(value) => {
                cli_eprintln!("{value} already exists");
            }
        }
    }

    pub fn into_err(self) -> CliError {
        CliError {
            kind: self,
            ..Default::default()
        }
    }
}

// Impl PartialEq manually so we can just match on kind, and not the associated data
impl PartialEq<Self> for CliErrorKind {
    fn eq(&self, rhs: &Self) -> bool {
        use CliErrorKind::*;

        match self {
            AmbiguousItem(_) => matches!(rhs, AmbiguousItem(_)),
            DuplicateName(_) => matches!(rhs, DuplicateName(_)),
            Io(_) => matches!(rhs, Io(_)),
            MissingApiKey => matches!(rhs, MissingApiKey),
            MissingPath => matches!(rhs, MissingPath),
            NoMatchingItem(_) => matches!(rhs, NoMatchingItem(_)),
            PermissionDenied => matches!(rhs, PermissionDenied),
            MultipleAtStdin => matches!(rhs, MultipleAtStdin),
            Seaplane(_) => matches!(rhs, Seaplane(_)),
            SerdeJson(_) => matches!(rhs, SerdeJson(_)),
            TomlSer(_) => matches!(rhs, TomlSer(_)),
            TomlDe(_) => matches!(rhs, TomlDe(_)),
            Unknown => matches!(rhs, Unknown),
            UnknownWithContext(_) => matches!(rhs, UnknownWithContext(_)),
            ExistingValue(_) => matches!(rhs, ExistingValue(_)),
            ImageReference(_) => matches!(rhs, ImageReference(_)),
        }
    }
}

impl CliError {
    /// Essentially destructure the cli_*! macros which actually also reduces the branches
    pub fn print(&self) {
        if log_level() <= &LogLevel::Error {
            // Scope for acquiring Mutex on global printer
            {
                let mut ptr = eprinter();
                ptr.set_color(Color::Red);
                let _ = write!(ptr, "error: ");
                ptr.reset();
            }

            // This function will try to reacquire the mutex
            self.kind.print();

            // Reacquire mutex lock
            let mut ptr = eprinter();
            for ColorString { color, msg } in &self.context {
                if let Some(c) = color {
                    ptr.set_color(*c);
                }
                let _ = write!(ptr, "{}", msg);
                ptr.reset();
            }
        }
    }

    pub fn exit(&self) -> ! {
        self.print();
        // TODO: solidify what should happen if an error with self.fatal = false is called here...
        std::process::exit(self.status.unwrap_or(1))
    }

    pub fn kind(&self) -> &CliErrorKind {
        &self.kind
    }
}
