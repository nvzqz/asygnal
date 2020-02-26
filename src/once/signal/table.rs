use crate::{
    signal::{AtomicSignalSet, Signal, SignalArray},
    unix::pipe::Writer,
};
use std::sync::atomic::{AtomicI32, Ordering};

#[repr(align(32))] // Potentially improve cache performance.
pub(crate) struct Table {
    pub registered: AtomicSignalSet,
    pub caught: AtomicSignalSet,
    entries: SignalArray<Entry>,
}

impl Table {
    #[inline]
    pub fn global() -> &'static Self {
        static GLOBAL: Table = Table {
            registered: AtomicSignalSet::new(),
            caught: AtomicSignalSet::new(),
            entries: [Entry::EMPTY; Signal::NUM],
        };
        &GLOBAL
    }

    #[inline]
    pub fn entry(&self, signal: Signal) -> &Entry {
        &self.entries[signal as usize]
    }
}

pub(crate) struct Entry {
    // TODO: Use `signalfd` on platforms that support it.
    /// The file descriptor for the writing end of the pipe.
    pub writer_fd: AtomicI32,
}

impl Entry {
    #[allow(clippy::declare_interior_mutable_const)]
    const EMPTY: Self = Self {
        writer_fd: AtomicI32::new(0),
    };

    /// Returns the writing end of the pipe.
    #[inline]
    pub fn load_writer(&self, ordering: Ordering) -> Writer {
        // SAFETY: In the case that the file descriptor is 0, the `pipe::Writer`
        // still has a valid representation.
        Writer(self.writer_fd.load(ordering))
    }
}
