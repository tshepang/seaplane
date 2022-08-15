// Common errors and their messages/hints

use crate::{
    error::{CliError, CliErrorKind, Context, Result},
    printer::Color,
};

pub fn wrap_cli_context(err: CliError, has_exact: bool, has_all: bool) -> Result<()> {
    use CliErrorKind::*;

    let e = match err.kind() {
        NoMatchingItem(_) => {
            if has_exact {
                err.context("(hint: remove '")
                    .color_context(Color::Yellow, "--exact")
                    .context("' to allow partial matches)\n")
                    .context("(hint: try '")
                    .color_context(Color::Yellow, "seaplane formation fetch-remote")
                    .context("' to update remote Formation definitions)\n")
            } else if has_all {
                err.context("(hint: try adding '")
                    .color_context(Color::Yellow, "--all")
                    .context("' to allow partial matches)\n")
                    .context("(hint: try '")
                    .color_context(Color::Yellow, "seaplane formation fetch-remote")
                    .context("' to sync local definitions)\n")
            } else {
                err
            }
        }
        AmbiguousItem(_) => {
            if has_all {
                err.context("(hint: add '")
                    .color_context(Color::Yellow, "--all")
                    .context("' operation on every match)\n")
            } else {
                err
            }
        }
        _ => err,
    };

    Err(e)
}

pub fn no_matching_item(item: String, has_exact: bool, has_all: bool) -> Result<()> {
    wrap_cli_context(CliErrorKind::NoMatchingItem(item).into_err(), has_exact, has_all)
}

pub fn ambiguous_item(item: String, has_all: bool) -> Result<()> {
    wrap_cli_context(CliErrorKind::AmbiguousItem(item).into_err(), false, has_all)
}
