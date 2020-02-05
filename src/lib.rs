//! \[WIP] Async-first signal handling, such as `CTRL` + `C`.
//!
//! Because this library is async-first, it should be more efficient than using
//! an alternative approach that awaits a blocking signal handler.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// mod ctrlc;

#[cfg(any(docsrs, feature = "once"))]
#[cfg_attr(docsrs, doc(cfg(feature = "once")))]
pub mod once;

#[cfg(any(unix, docsrs))]
#[cfg_attr(docsrs, doc(cfg(unix)))]
pub mod unix;

#[cfg(windows)]
mod windows;
