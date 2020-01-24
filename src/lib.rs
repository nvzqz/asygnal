//! \[WIP] Async-first signal handling, such as `CTRL` + `C`.
//!
//! Because this library is async-first, it should be more efficient than using
//! an alternative approach that awaits a blocking signal handler.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::task::{Context, Poll};

#[macro_use]
mod macros;

mod util;

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

impl CtrlC {
    /// Receive the next signal notification event.
    #[inline]
    pub async fn recv(&mut self) -> Option<()> {
        util::poll_fn(|cx| self.poll_recv(cx)).await
    }

    /// Poll to receive the next signal notification event, outside of an
    /// `async` context.
    pub fn poll_recv(&mut self, _cx: &mut Context<'_>) -> Poll<Option<()>> {
        unimplemented!()
    }
}

cfg_futures! {
    impl futures::stream::Stream for CtrlC {
        type Item = ();

        #[inline]
        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut Context<'_>
        ) -> Poll<Option<()>> {
            self.poll_recv(cx)
        }
    }
}
