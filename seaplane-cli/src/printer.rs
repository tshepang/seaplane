use std::{
    io,
    sync::{Mutex, MutexGuard, PoisonError},
};

use clap::ArgEnum;
use once_cell::sync::OnceCell;

use crate::{error::Result, Ctx};

static GLOBAL_PRINTER: OnceCell<Mutex<self::_printer::Printer>> = OnceCell::new();
static GLOBAL_EPRINTER: OnceCell<Mutex<self::_printer::Printer>> = OnceCell::new();

pub use self::_printer::{eprinter, printer, Printer};

#[derive(ArgEnum, Copy, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Table
    }
}

#[derive(ArgEnum, Copy, Clone, Debug, PartialEq)]
pub enum ColorChoice {
    Always,
    Ansi,
    Auto,
    Never,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
}

pub trait Output {
    fn print_json(&self, _ctx: &Ctx) -> Result<()> {
        cli_eprint!("--format=");
        cli_eprint!(@Yellow, "{}", "json");
        cli_eprintln!(" is not supported by this object");

        Ok(())
    }

    fn print_table(&self, _ctx: &Ctx) -> Result<()> {
        cli_eprint!("--format=");
        cli_eprint!(@Yellow, "{}", "table");
        cli_eprintln!(" is not supported by this object");

        Ok(())
    }
}

#[cfg(feature = "color")]
mod _printer {
    use super::*;

    use atty::Stream;
    use termcolor::{
        Color as TermColorColor, ColorChoice as TermColorChoice, ColorSpec, StandardStream,
        WriteColor,
    };

    fn detect_tty(stream: Stream) -> TermColorChoice {
        if atty::is(stream) {
            TermColorChoice::Auto
        } else {
            TermColorChoice::Never
        }
    }

    pub struct Printer(StandardStream);

    pub fn printer() -> MutexGuard<'static, Printer> {
        GLOBAL_PRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::stdout(TermColorChoice::Auto))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn eprinter() -> MutexGuard<'static, Printer> {
        GLOBAL_EPRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::stderr(TermColorChoice::Auto))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    impl Printer {
        pub fn init(color: ColorChoice) {
            use TermColorChoice::*;
            let (choice, echoice) = match color {
                ColorChoice::Always => (Always, Always),
                ColorChoice::Auto => (detect_tty(Stream::Stdout), detect_tty(Stream::Stderr)),
                ColorChoice::Ansi => (AlwaysAnsi, AlwaysAnsi),
                ColorChoice::Never => (Never, Never),
            };

            printer().set_stream(StandardStream::stdout(choice));
            eprinter().set_stream(StandardStream::stderr(echoice));
        }

        fn set_stream(&mut self, stream: StandardStream) {
            self.0 = stream;
        }

        pub fn set_color(&mut self, color: Color) {
            let _ = self
                .0
                .set_color(ColorSpec::new().set_fg(Some(color.into_termcolor())));
        }

        pub fn reset(&mut self) {
            let _ = self.0.reset();
        }
    }

    impl io::Write for Printer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.0.flush()
        }
    }

    impl Color {
        fn into_termcolor(self) -> TermColorColor {
            match self {
                Color::Black => TermColorColor::Black,
                Color::Blue => TermColorColor::Blue,
                Color::Green => TermColorColor::Green,
                Color::Red => TermColorColor::Red,
                Color::Cyan => TermColorColor::Cyan,
                Color::Magenta => TermColorColor::Magenta,
                Color::Yellow => TermColorColor::Yellow,
                Color::White => TermColorColor::White,
            }
        }
    }
}

#[cfg(not(feature = "color"))]
mod _printer {
    use super::*;

    enum StandardStream {
        Stdout(io::Stdout),
        Stderr(io::Stderr),
    }

    pub struct Printer(StandardStream);

    pub fn printer() -> MutexGuard<'static, Printer> {
        GLOBAL_PRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::Stdout(io::stdout()))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn eprinter() -> MutexGuard<'static, Printer> {
        GLOBAL_EPRINTER
            .get_or_init(|| Mutex::new(Printer(StandardStream::Stderr(io::stderr()))))
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    impl Printer {
        pub fn init(color: ColorChoice) {
            printer();
            eprinter();
        }

        pub fn set_color(&mut self, color: Color) {}

        pub fn reset(&mut self) {}
    }

    impl io::Write for Printer {
        fn write(&mut self, mut buf: &[u8]) -> io::Result<usize> {
            match self.0 {
                StandardStream::Stdout(ref mut s) => s.write(buf),
                StandardStream::Stderr(ref mut s) => s.write(buf),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            match self.0 {
                StandardStream::Stdout(ref mut s) => s.flush(),
                StandardStream::Stderr(ref mut s) => s.flush(),
            }
        }
    }
}
