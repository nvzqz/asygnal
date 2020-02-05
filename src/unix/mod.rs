//! Unix-specific functionality.

#![cfg_attr(not(unix), allow(warnings))]

pub(crate) mod pipe;
pub(crate) mod signal_mask;
pub(crate) mod signal_set;

// Declare this after `signal_set` so that `SignalSet` methods inside can come
// after the initial `impl`.
pub(crate) mod signal;

// Required when documenting on non-Unix platforms.
#[cfg(not(unix))]
mod libc_polyfill;

pub use {
    signal::Signal,
    signal_set::{SignalSet, SignalSetIter},
};

/// An array suitable for indexing with a `Signal` without bounds checks.
pub(crate) type SignalTable<T> = [T; Signal::NUM];
