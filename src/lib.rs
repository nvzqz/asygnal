//! [WIP] Async-first signal handling, such as `CTRL` + `C`.
//!
//! Because this library is async-first, it should be more efficient than using
//! an alternative approach that awaits a blocking signal handler.

#![deny(missing_docs)]
#![cfg_attr(feature = "_docs", feature(doc_cfg))]

#[macro_use]
mod macros;

cfg_unix! {
    pub mod unix;
}

#[cfg(windows)]
mod windows;

/// A future for `CTRL` + `C` signals.
#[derive(Debug)]
pub struct CtrlC {
    _private: (),
}
