use std::{fmt, iter::FromIterator, mem::MaybeUninit};

use super::{signal_mask::SignalMask, Signal};

/// A set of signals supported by this library.
///
/// Signals that cannot be handled are not listed as methods.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SignalSet(pub(crate) SignalMask);

impl From<Signal> for SignalSet {
    #[inline]
    fn from(signal: Signal) -> Self {
        Self(signal.into())
    }
}

impl fmt::Debug for SignalSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.into_iter()).finish()
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
            .fold(Self::new(), |builder, signal| builder.with(signal))
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

impl Default for SignalSet {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SignalSet {
    /// Creates a new, empty signal set builder.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(SignalMask::empty())
    }

    /// Converts `self` into a raw signal set.
    pub fn into_raw(self) -> libc::sigset_t {
        let mut set = unsafe {
            let mut set = MaybeUninit::<libc::sigset_t>::uninit();
            libc::sigemptyset(set.as_mut_ptr());
            set.assume_init()
        };
        for signal in self {
            unsafe { libc::sigaddset(&mut set, signal.into_raw()) };
        }
        set
    }

    /// Registers a signal handler that will only be fulfilled once.
    ///
    /// After the `SignalSetOnce` is fulfilled, all subsequent polls will return
    /// `Ready`.
    #[cfg(feature = "once")]
    pub fn register_once(
        self,
    ) -> Result<
        crate::once::unix::SignalSetOnce,
        crate::once::unix::RegisterOnceError,
    > {
        crate::once::unix::SignalSetOnce::register(self)
    }

    /// The set of signals that result in process termination.
    #[inline]
    #[must_use]
    pub const fn termination_set(self) -> Self {
        self.alarm()
            .hangup()
            .interrupt()
            .pipe()
            .quit()
            .terminate()
            .user_defined_1()
            .user_defined_2()
    }

    /// Returns `self` with `signal` added to it.
    #[inline]
    #[must_use]
    pub const fn with(self, signal: Signal) -> Self {
        Self(self.0.with(signal, true))
    }

    /// Returns `self` without `signal`.
    #[inline]
    #[must_use]
    pub const fn without(self, signal: Signal) -> Self {
        Self(self.0.with(signal, false))
    }

    /// Inserts `signal` into `self`.
    #[inline]
    pub fn insert(&mut self, signal: Signal) {
        self.0.set(signal, true);
    }

    /// Removes `signal` from `self`.
    #[inline]
    pub fn remove(&mut self, signal: Signal) {
        self.0.set(signal, false);
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
#[derive(Clone, Copy, Debug)]
pub struct SignalSetIter(SignalSet);

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
