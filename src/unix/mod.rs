//! Unix-specific functionality.

#![cfg_attr(not(unix), allow(warnings))]

use std::task::{Context, Poll};

mod signal_kind;
mod signal_set;

// Required when documenting on non-Unix platforms.
#[cfg(not(unix))]
mod libc_polyfill;

#[doc(inline)]
pub use self::{signal_kind::*, signal_set::*};

/// A future for receiving a particular signal.
#[derive(Debug)]
pub struct Signal {
    _private: (),
}

impl Signal {
    /// Receive the next signal notification event.
    #[inline]
    pub async fn recv(&mut self) -> Option<SignalKind> {
        crate::util::poll_fn(|cx| self.poll_recv(cx)).await
    }

    /// Poll to receive the next signal notification event, outside of an
    /// `async` context.
    pub fn poll_recv(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<SignalKind>> {
        unimplemented!()
    }
}

cfg_futures! {
    impl futures::stream::Stream for Signal {
        type Item = SignalKind;

        #[inline]
        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<SignalKind>> {
            self.poll_recv(cx)
        }
    }
}
