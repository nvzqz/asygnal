//! POSIX-style signals.
//!
//! # Platform Support
//!
//! Although this module is available on all platforms, signal handling is not.
//! Every signal is `#[cfg]`-ed to ensure it's only available on platforms where
//! it exists and won't cause other platforms to fail to compile.
//!
//! - `target_env`: `uclibc`
//!
//! - `target_os`: `android`, `dragonfly`, `emscripten`, `freebsd`, `fuchsia`,
//!   `haiku`, `hermit`, `illumos`, `ios`, `linux`, `macos`, `netbsd`,
//!   `openbsd`, `redox`, `solaris`, `vxworks`
//!
//! - `target_family`: `windows`
//!
//! See [`Signal`](enum.Signal.html#variants) for more info.
//!
//! Please [submit an issue] (or better, a [pull request]!) for any signals or
//! configurations missing in [`asygnal`].
//!
//! [`asygnal`]:       https://github.com/nvzqz/asygnal
//! [pull request]:    https://github.com/nvzqz/asygnal/pulls
//! [submit an issue]: https://github.com/nvzqz/asygnal/issues

#![cfg_attr(not(unix), allow(warnings))]

mod set;

// Declare this after `set` so that `SignalSet` methods inside can come after
// the initial `impl`.
mod signal;

pub use {
    set::{AtomicSignalSet, SignalSet, SignalSetIter},
    signal::Signal,
};

/// An array suitable for indexing with a [`Signal`] without bounds checks.
///
/// The size of this is exempted from [semantic versioning](https://semver.org)
/// because it may change in the future as more signals are added.
///
/// # Examples
///
/// Because the size varies between platforms, [`Signal::NUM`] must be used to
/// initialize the array in a cross-platform way:
///
/// ```
/// use asygnal::signal::{Signal, SignalArray};
///
/// let mut raw_values: SignalArray<libc::c_int> = [0; Signal::NUM];
///
/// for signal in Signal::all() {
///     raw_values[signal as usize] = signal.into_raw();
/// }
/// ```
///
/// [`Signal`]:      enum.Signal.html
/// [`Signal::NUM`]: enum.Signal.html#associatedconstant.NUM
pub type SignalArray<T> = [T; Signal::NUM];
