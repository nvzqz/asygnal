use std::{fmt, mem};

use super::Signal;

mod atomic;
pub use atomic::*;

/// Collection of signals supported by this library, backed by a cheap bit mask.
///
/// # Target Configuration
///
/// Signals that cannot be handled are not listed as methods. Each method has a
/// configuration appropriate for what platforms it's supported on. For example,
/// see [`SignalSet::abort`](#method.abort).
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SignalSet(u32);

impl From<Signal> for SignalSet {
    #[inline]
    fn from(signal: Signal) -> Self {
        Self::from_signal(signal)
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
            .fold(Self::new(), |builder, signal| builder.with(signal))
    }
}

impl FromIterator<SignalSet> for SignalSet {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = SignalSet>,
    {
        iter.into_iter()
            .fold(Self::new(), |builder, signal| builder.with_all(signal))
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

impl Extend<SignalSet> for SignalSet {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = SignalSet>,
    {
        iter.into_iter().for_each(|signals| self.insert(signals));
    }
}

impl SignalSet {
    /// Creates a new, empty signal set.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new set with all signals enabled.
    #[inline]
    #[must_use]
    pub const fn all() -> Self {
        Self(!(!0u32 << Signal::NUM))
    }

    /// Creates a new set with `signal` enabled.
    #[inline]
    pub const fn from_signal(signal: Signal) -> Self {
        Self::new().with(signal)
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
    /// - [`user_def_1`](#method.user_def_1)
    /// - [`user_def_2`](#method.user_def_2)
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

    cfg_docs! {
        /// Converts `self` into a raw signal set, returning [`None`] on error.
        #[cfg(any(
            unix,
            target_os = "fuchsia",
            target_os = "vxworks",
        ))]
        pub fn into_raw(self) -> Option<libc::sigset_t> {
            let mut set = unsafe {
                let mut set = mem::MaybeUninit::<libc::sigset_t>::uninit();

                if libc::sigemptyset(set.as_mut_ptr()) < 0 {
                    return None;
                }

                set.assume_init()
            };

            for signal in self {
                let err = unsafe { libc::sigaddset(&mut set, signal.into_raw()) };
                if err < 0 {
                    return None;
                }
            }

            Some(set)
        }
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

    /// Returns `self` with `signal` added to or removed from it.
    #[inline]
    #[must_use]
    pub const fn setting(mut self, signal: Signal, value: bool) -> Self {
        let signal = signal as u32;
        self.0 = (self.0 & !(1 << signal)) | ((value as u32) << signal);
        self
    }

    /// Inserts or removes `signal` from `self`.
    #[inline]
    pub fn set(&mut self, signal: Signal, value: bool) {
        *self = self.setting(signal, value);
    }

    /// Returns `self` with `signal` added to it.
    #[inline]
    #[must_use]
    pub const fn with(self, signal: Signal) -> Self {
        self.setting(signal, true)
    }

    /// Returns `self` with all of `signals` added to it.
    #[inline]
    #[must_use]
    pub const fn with_all(self, signals: SignalSet) -> Self {
        Self(self.0 | signals.0)
    }

    /// Inserts `signal` into `self`.
    #[inline]
    pub fn insert<S: Into<SignalSet>>(&mut self, signals: S) {
        self.0 |= signals.into().0;
    }

    /// Returns `self` with `signal` removed from it.
    #[inline]
    #[must_use]
    pub const fn without(self, signal: Signal) -> Self {
        self.setting(signal, false)
    }

    /// Returns `self` with all of `signals` removed from it.
    #[inline]
    #[must_use]
    pub const fn without_all(self, signals: SignalSet) -> Self {
        Self(self.0 & !signals.0)
    }

    /// Removes `signals` from `self`.
    #[inline]
    pub fn remove<S: Into<SignalSet>>(&mut self, signals: S) {
        self.0 &= !signals.into().0;
    }

    /// Returns the least significant signal bit of `self`.
    #[inline]
    pub const fn first(self) -> Option<Signal> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(self.first_unchecked()) }
        }
    }

    /// Returns the least significant signal bit of `self`, assuming `self` is
    /// not empty.
    #[inline]
    pub const unsafe fn first_unchecked(self) -> Signal {
        Signal::from_u8_unchecked(self.0.trailing_zeros() as u8)
    }

    /// Removes the least significant signal bit from `self`, returning it.
    #[inline]
    #[must_use = "use 'remove_first' instead"]
    pub fn pop_first(&mut self) -> Option<Signal> {
        // Explicitly removing the lsb is slightly more efficient than toggling.
        let lsb = self.first()?;
        self.remove_first();
        Some(lsb)
    }

    /// Returns `self` with the least significant bit removed from it.
    #[inline]
    #[must_use]
    pub const fn without_first(self) -> Self {
        Self(self.0 & self.0.wrapping_sub(1))
    }

    /// Removes the least significant signal bit from `self`, without returning
    /// it.
    #[inline]
    pub fn remove_first(&mut self) {
        *self = self.without_first();
    }

    /// Returns the most significant signal bit of `self`.
    #[inline]
    pub const fn last(self) -> Option<Signal> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(self.last_unchecked()) }
        }
    }

    /// Returns the most significant signal bit of `self`, assuming `self` is
    /// not empty.
    #[inline]
    pub const unsafe fn last_unchecked(self) -> Signal {
        let bits = mem::size_of::<Self>() * 8 - 1;
        let signal = bits - self.0.leading_zeros() as usize;
        Signal::from_u8_unchecked(signal as u8)
    }

    /// Removes the most significant signal signal from `self`, returning it.
    #[inline]
    #[must_use = "use 'remove_last' instead"]
    pub fn pop_last(&mut self) -> Option<Signal> {
        let msb = self.last()?;
        self.0 ^= Self::from_signal(msb).0;
        Some(msb)
    }

    /// Removes the most significant signal from `self`, without returning it.
    #[inline]
    pub fn remove_last(&mut self) {
        // This method exists for consistency with `remove_first`. This is the
        // currently known fastest way to remove the last bit.
        let _ = self.pop_last();
    }

    /// The number of signals in `self`.
    #[inline]
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns `true` if there are no signals in `self`.
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if `signal` is stored in `self`.
    #[inline]
    pub const fn contains(self, signal: Signal) -> bool {
        self.contains_any(Self::from_signal(signal))
    }

    /// Returns `true` if any [`Signal`] in `signals` is stored in `self`.
    ///
    /// [`Signal`]: struct.Signal.html
    #[inline]
    pub const fn contains_any(self, signals: SignalSet) -> bool {
        self.0 & signals.0 != 0
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
        self.0.pop_first()
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
        self.0.pop_last()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let all = SignalSet::all();
        assert_eq!(all.len(), Signal::NUM);

        fn assert(signal: u32) {
            assert!(
                Signal::from_u32(signal).is_some(),
                "Found incorrect signal {} in mask",
                signal
            );
        }

        // This assumes that the signal's enum value is still the same, even for
        // an invalid representation.
        all.into_iter().for_each(|s| assert(s as u32));
        all.into_iter().rev().for_each(|s| assert(s as u32));
    }
}
