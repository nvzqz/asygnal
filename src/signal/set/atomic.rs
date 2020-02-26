use std::sync::atomic::{AtomicU32, Ordering};

use super::SignalSet;

/// Collection of signals supported by this library, backed by a cheap bit mask,
/// with atomic operations.
pub struct AtomicSignalSet(AtomicU32);

impl From<SignalSet> for AtomicSignalSet {
    #[inline]
    fn from(signals: SignalSet) -> Self {
        Self::from_signal_set(signals)
    }
}

impl AtomicSignalSet {
    /// Creates a new, empty signal set.
    #[inline]
    pub const fn new() -> Self {
        Self::from_signal_set(SignalSet::new())
    }

    /// Creates a new atomic signal set from `signals`.
    #[inline]
    pub const fn from_signal_set(signals: SignalSet) -> Self {
        Self(AtomicU32::new(signals.0))
    }

    /// Atomically loads the inner `SignalSet` using `ordering`.
    #[inline]
    #[must_use]
    pub fn load(&self, ordering: Ordering) -> SignalSet {
        SignalSet(self.0.load(ordering))
    }

    /// Atomically stores the `signals` in `self` using `ordering`.
    #[inline]
    pub fn store<S: Into<SignalSet>>(&self, signals: S, ordering: Ordering) {
        self.0.store(signals.into().0, ordering);
    }

    /// Atomically inserts `signals` into `self` using `ordering`.
    #[inline]
    pub fn insert<S: Into<SignalSet>>(
        &self,
        signals: S,
        ordering: Ordering,
    ) -> SignalSet {
        SignalSet(self.0.fetch_or(signals.into().0, ordering))
    }

    /// Atomically removes `signals` from `self` using `ordering`.
    #[inline]
    pub fn remove<S: Into<SignalSet>>(
        &self,
        signals: S,
        ordering: Ordering,
    ) -> SignalSet {
        SignalSet(self.0.fetch_and(!signals.into().0, ordering))
    }
}
