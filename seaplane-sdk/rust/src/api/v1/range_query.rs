//! This module contains types for performing range queries across metadata services that use the paging mechanism.

use crate::{base64::Base64Encoded, impl_base64};

/// The directory from which to perform a given range query, excluding the trailing slash, encoded in url-safe base64
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Directory {
    inner: Base64Encoded,
}
impl_base64!(Directory);

/// The full context with which to perform a range query
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RangeQueryContext<T> {
    /// The directory, if any, within which to perform the range query.
    dir: Option<Directory>,
    /// The lower bound on the page of results to return.
    from: Option<T>,
}

// This has to be hand implemented to avoid an incorrect Default bound on T
impl<T> Default for RangeQueryContext<T> {
    fn default() -> Self {
        Self {
            dir: None,
            from: None,
        }
    }
}

impl<T> RangeQueryContext<T> {
    /// Creates a blank context, suitable for querying the root directory at the first available key
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the directory within which to perform the query
    pub fn set_directory(&mut self, dir: Directory) {
        self.dir = Some(dir);
    }

    /// Set the key to use when beginning the next page of the query
    pub fn set_from(&mut self, next_key: T) {
        self.from = Some(next_key);
    }

    pub fn directory(&self) -> &Option<Directory> {
        &self.dir
    }

    pub fn from(&self) -> &Option<T> {
        &self.from
    }
}
