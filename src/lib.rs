//! \[WIP] Async-first signal handling, such as `CTRL` + `C`.
//!
//! Because this library is async-first, it should be more efficient than using
//! an alternative approach that awaits a blocking signal handler.
//!
//! # Platform Support
//!
//! Although the `signal` module is available on all platforms, signal handling
//! is not. Every signal is `#[cfg]`-ed to ensure it's only available on
//! platforms where it exists and won't cause other platforms to fail to
//! compile.
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

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

#[cfg(any(docsrs, feature = "once"))]
#[cfg_attr(docsrs, doc(cfg(feature = "once")))]
pub mod once;

pub mod signal;
pub use signal::{Signal, SignalSet};

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "windows")]
mod windows;
