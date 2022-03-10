// Copyright ⓒ  2022 Seaplane IO, Inc.
// Licensed under the Apache 2.0 license
// (see LICENSE or <http://opensource.org/licenses/Apache-2.0>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

#![warn(
    // TODO: we'll get to this
    //missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts
)]
#![forbid(unsafe_code)]

pub mod api;
pub mod error;

/// Allows using the exact same traits derived from these dependencies. If re-exported here, that
/// most likely means the derived trait appears in a types public API and you should `use` the
/// re-exported crate trait instead of one from your own dependencies list.
pub mod rexports {
    pub use strum;
}
