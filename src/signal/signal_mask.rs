use super::Signal;
use std::{
    mem,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct SignalMask(u32);

impl From<Signal> for SignalMask {
    #[inline]
    fn from(signal: Signal) -> Self {
        Self::from_signal(signal)
    }
}

impl SignalMask {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn full() -> Self {
        Self(!(!0u32 << Signal::NUM))
    }

    #[inline]
    pub const fn from_signal(signal: Signal) -> Self {
        Self(1 << signal as u32)
    }

    #[inline]
    pub const fn with(mut self, signal: Signal, value: bool) -> Self {
        let signal = signal as u32;
        self.0 = (self.0 & !(1 << signal)) | ((value as u32) << signal);
        self
    }

    #[inline]
    pub const fn contains(self, signal: Signal) -> bool {
        self.contains_any(Self::from_signal(signal))
    }

    #[inline]
    pub const fn contains_any(self, mask: SignalMask) -> bool {
        self.0 & mask.0 != 0
    }

    /// Returns the number of signals in `self`.
    #[inline]
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the least significant bit of `self`.
    #[inline]
    fn lsb(self) -> Option<Signal> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(self.lsb_unchecked()) }
        }
    }

    /// Returns the most significant bit of `self`.
    #[inline]
    fn msb(self) -> Option<Signal> {
        if self.is_empty() {
            None
        } else {
            unsafe { Some(self.msb_unchecked()) }
        }
    }

    /// Returns the least significant bit of `self` without checking whether
    /// `self` is empty.
    #[inline]
    unsafe fn lsb_unchecked(self) -> Signal {
        Signal::from_u8_unchecked(self.0.trailing_zeros() as u8)
    }

    /// Returns the least significant bit of `self` without checking whether
    /// `self` is empty.
    #[inline]
    unsafe fn msb_unchecked(self) -> Signal {
        let bits = mem::size_of::<Self>() * 8 - 1;
        let signal = bits - self.0.leading_zeros() as usize;
        Signal::from_u8_unchecked(signal as u8)
    }

    /// Removes the least significant bit from `self`.
    #[inline]
    fn remove_lsb(&mut self) {
        self.0 &= self.0.wrapping_sub(1);
    }

    /// Removes the least significant bit from `self` and returns it.
    #[inline]
    pub fn pop_lsb(&mut self) -> Option<Signal> {
        // Explicitly removing the lsb is slightly more efficient than toggling.
        let lsb = self.lsb()?;
        self.remove_lsb();
        Some(lsb)
    }

    /// Removes the most significant bit from `self` and returns it.
    #[inline]
    pub fn pop_msb(&mut self) -> Option<Signal> {
        let msb = self.msb()?;
        self.0 ^= Self::from_signal(msb).0;
        Some(msb)
    }
}

pub(crate) struct AtomicSignalMask(AtomicU32);

impl AtomicSignalMask {
    #[inline]
    pub const fn empty() -> Self {
        Self(AtomicU32::new(0))
    }

    #[inline]
    pub fn enable(&self, signal: Signal, ordering: Ordering) -> SignalMask {
        self.enable_all(SignalMask::from_signal(signal), ordering)
    }

    #[inline]
    pub fn enable_all(
        &self,
        mask: SignalMask,
        ordering: Ordering,
    ) -> SignalMask {
        SignalMask(self.0.fetch_or(mask.0, ordering))
    }

    #[inline]
    pub fn contains(&self, signal: Signal, ordering: Ordering) -> bool {
        SignalMask(self.0.load(ordering)).contains(signal)
    }

    #[inline]
    pub fn contains_any(&self, mask: SignalMask, ordering: Ordering) -> bool {
        SignalMask(self.0.load(ordering)).contains_any(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full() {
        let mut full = SignalMask::full();
        assert_eq!(full.len(), Signal::NUM);

        while let Some(signal) = full.pop_lsb() {
            // This assumes that the value is still the same, even for an
            // invalid representation.
            let signal = signal as u32;

            assert!(
                Signal::from_u32(signal).is_some(),
                "Found incorrect signal {} in mask",
                signal
            );
        }
    }
}
