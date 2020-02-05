//! Futures that are fulfilled once.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(any(unix, docsrs))]
#[cfg_attr(docsrs, doc(cfg(unix)))]
pub mod unix;

#[cfg(unix)]
type CtrlCOnceInner = unix::SignalSetOnce;

/// A future that is fulfilled once upon receiving `CTRL` + `C`.
///
/// After an instance is fulfilled, all subsequent polls will return `Ready`.
#[derive(Debug)]
pub struct CtrlCOnce(CtrlCOnceInner);

impl Future for CtrlCOnce {
    type Output = ();

    #[inline]
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}

impl CtrlCOnce {
    /// Registers the `CTRL` + `C` handler.
    #[inline]
    pub fn register() -> Result<Self, RegisterCtrlCOnceError> {
        // Register via `Signal` instead of `SignalSet` since it's slightly more
        // efficient.
        #[cfg(unix)]
        let inner = unix::SignalSetOnce::from(
            crate::unix::Signal::Interrupt.register_once()?,
        );

        Ok(Self(inner))
    }

    /// Registers the handler for all signals that would otherwise terminate.
    ///
    /// # Unix Behavior
    ///
    /// On Unix-like systems, this corresponds to: [`Alarm`], [`Hangup`],
    /// [`Interrupt`], [`Pipe`], [`Quit`], [`Terminate`], [`UserDefined1`], and
    /// [`UserDefined2`].
    ///
    /// [`Alarm`]:        ../unix/enum.Signal.html#variant.Alarm
    /// [`Hangup`]:       ../unix/enum.Signal.html#variant.Hangup
    /// [`Interrupt`]:    ../unix/enum.Signal.html#variant.Interrupt
    /// [`Pipe`]:         ../unix/enum.Signal.html#variant.Pipe
    /// [`Quit`]:         ../unix/enum.Signal.html#variant.Quit
    /// [`Terminate`]:    ../unix/enum.Signal.html#variant.Terminate
    /// [`UserDefined1`]: ../unix/enum.Signal.html#variant.UserDefined1
    /// [`UserDefined2`]: ../unix/enum.Signal.html#variant.UserDefined2
    #[inline]
    pub fn register_termination() -> Result<Self, RegisterCtrlCOnceError> {
        #[cfg(unix)]
        let inner = crate::unix::SignalSet::new()
            .termination_set()
            .register_once()?;

        Ok(Self(inner))
    }
}

#[cfg(unix)]
type RegisterCtrlCOnceErrorInner = unix::RegisterOnceError;

/// An error returned when registering a [`Signal`] or [`SignalSet`] fails.
///
/// [`Signal`]:    ../../unix/enum.Signal.html
/// [`SignalSet`]: ../../unix/struct.SignalSet.html
#[derive(Debug)]
pub struct RegisterCtrlCOnceError(RegisterCtrlCOnceErrorInner);

impl From<RegisterCtrlCOnceErrorInner> for RegisterCtrlCOnceError {
    #[inline]
    fn from(error: RegisterCtrlCOnceErrorInner) -> Self {
        Self(error)
    }
}
