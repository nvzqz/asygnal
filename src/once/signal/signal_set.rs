use std::{
    future::Future,
    pin::Pin,
    sync::atomic::Ordering,
    task::{Context, Poll},
};

use super::{table::Table, Driver, RegisterOnceError, SignalOnce};
use crate::{unix::pipe, SignalSet};

/// A future that is fulfilled once upon receiving a [`Signal`] in a
/// [`SignalSet`].
///
/// After an instance is fulfilled, all subsequent polls will return [`Ready`].
///
/// [`Signal`]:    ../../unix/enum.Signal.html
/// [`SignalSet`]: ../../unix/struct.SignalSet.html
///
/// [`Ready`]: https://doc.rust-lang.org/std/task/enum.Poll.html#variant.Ready
#[derive(Debug)]
pub struct SignalSetOnce {
    signals: SignalSet,
    driver: Driver,
}

impl From<SignalOnce> for SignalSetOnce {
    #[inline]
    fn from(signal: SignalOnce) -> Self {
        let signals = SignalSet::from(signal.signal);
        let driver = signal.driver;
        Self { signals, driver }
    }
}

impl Future for SignalSetOnce {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let table = Table::global();

        if table
            .caught
            .load(Ordering::SeqCst)
            .contains_any(self.signals)
        {
            return Poll::Ready(());
        }

        self.driver.poll(cx)
    }
}

impl SignalSetOnce {
    /// Registers a handler for `signals` that will only be fulfilled once.
    pub fn register(signals: SignalSet) -> Result<Self, RegisterOnceError> {
        // TODO: Handle a signal in `signals` already being registered.

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

        let mut old_handles =
            Vec::<super::RegisteredSignal>::with_capacity(signals.len());

        for signal in signals {
            Table::global()
                .entry(signal)
                .writer_fd()
                .store(writer.0, Ordering::SeqCst);

            match super::register_signal(signal) {
                Ok(handle) => {
                    old_handles.push(handle);
                }
                Err(error) => {
                    old_handles.into_iter().for_each(|handle| {
                        handle.reset();
                    });
                    close_pipe();
                    return Err(error.into());
                }
            }
        }

        Ok(Self { signals, driver })
    }
}
