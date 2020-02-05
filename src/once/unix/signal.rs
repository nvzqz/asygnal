use std::{
    future::Future,
    pin::Pin,
    sync::atomic::Ordering,
    task::{Context, Poll},
};

use super::{table::Table, Driver, RegisterOnceError};
use crate::unix::{pipe, Signal};

/// A future that is fulfilled once upon receiving a [`Signal`].
///
/// After an instance is fulfilled, all subsequent polls will return [`Ready`].
///
/// [`Signal`]: ../../unix/enum.Signal.html
///
/// [`Ready`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready
#[derive(Debug)]
pub struct SignalOnce {
    pub(super) signal: Signal,
    pub(super) driver: Driver,
}

impl Future for SignalOnce {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let table = Table::global();

        if table.caught.contains(self.signal, Ordering::SeqCst) {
            return Poll::Ready(());
        }

        self.driver.poll(cx)
    }
}

impl SignalOnce {
    /// Registers a handler for `signal` that will only be fulfilled once.
    pub fn register(signal: Signal) -> Result<Self, RegisterOnceError> {
        // TODO: Handle `signal` already being registered.

        let (reader, writer) = pipe::pipe()?;

        let close_pipe = || unsafe {
            libc::close(reader.0);
            libc::close(writer.0);
        };

        let driver = match Driver::new(reader) {
            Ok(d) => d,
            Err(error) => {
                close_pipe();
                return Err(error.into());
            }
        };

        Table::global()
            .entry(signal)
            .writer_fd()
            .store(writer.0, Ordering::SeqCst);

        match super::register_signal(signal) {
            Ok(_) => Ok(Self { signal, driver }),
            Err(error) => {
                close_pipe();
                Err(error.into())
            }
        }
    }
}
