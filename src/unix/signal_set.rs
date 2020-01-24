use libc::sigset_t;
use std::{iter::FromIterator, mem::MaybeUninit};

use super::SignalKind;

/// A stream for receiving a set of signals.
#[derive(Debug)]
pub struct SignalSet {
    _private: (),
}

impl SignalSet {
    /// Returns a builder for constructing an instance.
    #[inline]
    pub fn builder() -> SignalSetBuilder {
        SignalSetBuilder::new()
    }
}

/// Constructs a [`SignalSet`] using the builder pattern.
///
/// Signals that cannot be handled are not listed as methods.
///
/// [`SignalSet`]: struct.SignalSet.html
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalSetBuilder {
    signal_set: sigset_t,
}

impl FromIterator<SignalKind> for SignalSetBuilder {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = SignalKind>,
    {
        iter.into_iter()
            .fold(Self::new(), |builder, signal| builder.with(signal))
    }
}

impl Extend<SignalKind> for SignalSetBuilder {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = SignalKind>,
    {
        iter.into_iter().for_each(|signal| self.insert(signal));
    }
}

impl SignalSetBuilder {
    /// Creates a new, empty signal set builder.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            let mut set = MaybeUninit::<sigset_t>::uninit();
            libc::sigemptyset(set.as_mut_ptr());
            Self::from_raw(set.assume_init())
        }
    }

    /// Creates a new builder from the raw `signal_set`.
    ///
    /// # Safety
    ///
    /// This library assumes that all signals used are valid. Supplying an
    /// unsupported signal set invalidates this assumption.
    #[inline]
    #[must_use]
    pub const unsafe fn from_raw(signal_set: sigset_t) -> Self {
        Self { signal_set }
    }

    /// Returns the raw value of this signal set builder.
    #[inline]
    #[must_use]
    pub const fn into_raw(self) -> sigset_t {
        self.signal_set
    }

    /// The set of signals that result in process termination.
    #[inline]
    #[must_use]
    pub fn termination_set(self) -> Self {
        self.alarm()
            .hangup()
            .interrupt()
            .pipe()
            .quit()
            .terminate()
            .user_defined_1()
            .user_defined_2()
    }

    // REMINDER: When updating the documentation of the following methods, their
    // corresponding `SignalKind` constants must be updated as well.

    /// The `SIGALRM` signal; sent when a real-time timer expires.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn alarm(self) -> Self {
        self.with(SignalKind::ALARM)
    }

    /// The `SIGCHLD` signal; sent when the status of a child process changes.
    ///
    /// **Default behavior:** ignored.
    #[inline]
    #[must_use]
    pub fn child(self) -> Self {
        self.with(SignalKind::CHILD)
    }

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn hangup(self) -> Self {
        self.with(SignalKind::HANGUP)
    }

    /// The `SIGINFO` signal; sent to request a status update from the process.
    ///
    /// **Supported on:** `dragonfly`, `freebsd`, `macos`, `netbsd`, `openbsd`.
    ///
    /// **Keyboard shortcut:** `CTRL` + `T`.
    ///
    /// **Default behavior:** ignored.
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    // This doesn't seem to change docs to list the supported target OSes.
    #[cfg_attr(
        feature = "_docs",
        doc(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        ))
    )]
    #[inline]
    #[must_use]
    pub fn info(self) -> Self {
        self.with(SignalKind::INFO)
    }

    /// The `SIGINT` signal; sent to interrupt a program.
    ///
    /// **Keyboard shortcut:** `CTRL` + `C`.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn interrupt(self) -> Self {
        self.with(SignalKind::INTERRUPT)
    }

    /// The `SIGIO` signal; sent when I/O operations are possible on some file
    /// descriptor.
    ///
    /// **Default behavior:** ignored.
    #[inline]
    #[must_use]
    pub fn io(self) -> Self {
        self.with(SignalKind::IO)
    }

    /// The `SIGPIPE` signal; sent when the process attempts to write to a pipe
    /// which has no reader.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn pipe(self) -> Self {
        self.with(SignalKind::PIPE)
    }

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn quit(self) -> Self {
        self.with(SignalKind::QUIT)
    }

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn terminate(self) -> Self {
        self.with(SignalKind::TERMINATE)
    }

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn user_defined_1(self) -> Self {
        self.with(SignalKind::USER_DEFINED_1)
    }

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    #[inline]
    #[must_use]
    pub fn user_defined_2(self) -> Self {
        self.with(SignalKind::USER_DEFINED_2)
    }

    /// The `SIGWINCH` signal; sent when the terminal window is resized.
    ///
    /// **Default behavior:** ignored.
    #[inline]
    #[must_use]
    pub fn window_change(self) -> Self {
        self.with(SignalKind::WINDOW_CHANGE)
    }

    /// Returns `self` with `signal` added to it.
    #[inline]
    #[must_use]
    pub fn with(mut self, signal: SignalKind) -> Self {
        self.insert(signal);
        self
    }

    /// Adds `signal` to `self`.
    #[inline]
    pub fn insert(&mut self, signal: SignalKind) {
        unsafe {
            libc::sigaddset(&mut self.signal_set, signal.into_raw());
        }
    }
}