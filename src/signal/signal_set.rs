use std::{fmt, iter::FromIterator};

use super::{signal_mask::SignalMask, Signal};

/// A set of signals supported by this library.
///
/// Signals that cannot be handled are not listed as methods.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SignalSet(pub(crate) SignalMask);

impl From<Signal> for SignalSet {
    #[inline]
    fn from(signal: Signal) -> Self {
        Self(signal.into())
    }
}

impl From<SignalSetIter> for SignalSet {
    #[inline]
    fn from(iter: SignalSetIter) -> Self {
        iter.into_signal_set()
    }
}

impl fmt::Debug for SignalSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

impl IntoIterator for SignalSet {
    type Item = Signal;
    type IntoIter = SignalSetIter;

    #[inline]
    fn into_iter(self) -> SignalSetIter {
        SignalSetIter(self)
    }
}

impl FromIterator<Signal> for SignalSet {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Signal>,
    {
        iter.into_iter()
            .fold(Self::new(), |builder, signal| builder.with(signal, true))
    }
}

impl Extend<Signal> for SignalSet {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Signal>,
    {
        iter.into_iter().for_each(|signal| self.insert(signal));
    }
}

impl SignalSet {
    /// Creates a new, empty signal set builder.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(SignalMask::empty())
    }

    /// Creates a new set with all signals enabled.
    #[inline]
    #[must_use]
    pub const fn all() -> Self {
        Self(SignalMask::full())
    }

    /// Creates a new set of signals that result in process termination.
    ///
    /// This only includes signals that can be trivially handled with grace:
    /// - [`alarm`](#method.alarm)
    /// - [`hangup`](#method.hangup)
    /// - [`interrupt`](#method.interrupt)
    /// - [`pipe`](#method.pipe)
    /// - [`profile`](#method.profile)
    /// - [`quit`](#method.quit)
    /// - [`terminate`](#method.terminate)
    /// - [`user_defined_1`](#method.user_defined_1)
    /// - [`user_defined_2`](#method.user_defined_2)
    /// - [`vt_alarm`](#method.vt_alarm)
    ///
    /// If a listed signal is not available for the current target, the returned
    /// set will simply not include it.
    #[inline]
    #[must_use]
    pub const fn termination() -> Self {
        #[allow(unused_mut)]
        let mut set = Self::new();

        // This would make for an amazing use case of `#[cfg(accessible(...))]`.
        // See https://github.com/rust-lang/rust/issues/64797 for info on this.
        #[cfg(any(
            // According to `libc`:
            // "bsd"
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd",
            // "linux-like"
            target_os = "linux",
            target_os = "android",
            target_os = "emscripten",
            // "solarish"
            target_os = "solaris",
            target_os = "illumos",
            // Uncategorized
            windows,
            target_os = "fuchsia",
            target_os = "redox",
            target_os = "haiku",
            target_os = "hermit",
            target_os = "vxworks",
            target_env = "uclibc",
        ))]
        {
            #[cfg(not(windows))]
            {
                set = set.alarm().hangup().pipe().quit();
            }

            #[cfg(any(
                not(target_env = "uclibc"),
                all(
                    target_env = "uclibc",
                    any(
                        target_arch = "arm",
                        target_arch = "mips",
                        target_arch = "mips64",
                    ),
                ),
            ))]
            {
                set = set.user_def_1().user_def_2();
            }

            set = set.interrupt().terminate();
        }

        #[cfg(any(
            // According to `libc`:
            // "bsd"
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd",
            // "linux-like"
            target_os = "linux",
            target_os = "android",
            target_os = "emscripten",
            // "solarish"
            target_os = "solaris",
            target_os = "illumos",
            // Uncategorized
            target_os = "fuchsia",
            target_os = "redox",
            target_os = "haiku",
            all(
                // Oddly enough, "x86_64" does not support this signal.
                target_env = "uclibc",
                any(
                    target_arch = "arm",
                    target_arch = "mips",
                    target_arch = "mips64",
                ),
            ),
        ))]
        {
            set = set.profile().vt_alarm();
        }

        set
    }

    /// Registers a signal handler that will only be fulfilled once.
    ///
    /// After the `SignalSetOnce` is fulfilled, all subsequent polls will return
    /// `Ready`.
    #[cfg(any(docsrs, feature = "once"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "once")))]
    pub fn register_once(
        self,
    ) -> Result<
        crate::once::signal::SignalSetOnce,
        crate::once::signal::RegisterOnceError,
    > {
        crate::once::signal::SignalSetOnce::register(self)
    }

    /// Returns `self` with `signal` added or removed from it.
    #[inline]
    #[must_use]
    pub const fn with(self, signal: Signal, value: bool) -> Self {
        Self(self.0.with(signal, value))
    }

    /// Inserts or removes `signal` from `self`.
    #[inline]
    pub fn set(&mut self, signal: Signal, value: bool) {
        *self = self.with(signal, value);
    }

    /// Inserts `signal` into `self`.
    #[inline]
    pub fn insert(&mut self, signal: Signal) {
        self.set(signal, true);
    }

    /// Removes `signal` from `self`.
    #[inline]
    pub fn remove(&mut self, signal: Signal) {
        self.set(signal, false);
    }

    /// Removes the first signal from `self`, returning it.
    #[inline]
    pub fn remove_first(&mut self) -> Option<Signal> {
        self.0.pop_lsb()
    }

    /// Removes the last signal from `self`, returning it.
    #[inline]
    pub fn remove_last(&mut self) -> Option<Signal> {
        self.0.pop_msb()
    }

    /// The number of signals in `self`.
    #[inline]
    pub const fn len(self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no signals in `self`.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }
}

/// An iterator over a [`SignalSet`].
///
/// [`SignalSet`]: struct.SignalSet.html
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalSetIter(SignalSet);

impl From<SignalSet> for SignalSetIter {
    #[inline]
    fn from(signals: SignalSet) -> Self {
        Self(signals)
    }
}

impl Iterator for SignalSetIter {
    type Item = Signal;

    #[inline]
    fn next(&mut self) -> Option<Signal> {
        self.0.remove_first()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.len()
    }

    #[inline]
    fn last(mut self) -> Option<Signal> {
        self.next_back()
    }
}

impl DoubleEndedIterator for SignalSetIter {
    #[inline]
    fn next_back(&mut self) -> Option<Signal> {
        self.0.remove_last()
    }
}

impl ExactSizeIterator for SignalSetIter {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl SignalSetIter {
    /// Creates a new iterator for `signals`.
    #[inline]
    pub const fn new(signals: SignalSet) -> Self {
        Self(signals)
    }

    /// Returns the remaining set of signals.
    #[inline]
    pub const fn into_signal_set(self) -> SignalSet {
        self.0
    }
}
