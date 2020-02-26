use crate::{
    signal::{AtomicSignalSet, Signal, SignalArray},
    unix::pipe::Writer,
};
use std::sync::atomic::{AtomicI32, Ordering};

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
    writer: AtomicI32,
}

impl Entry {
    #[allow(clippy::declare_interior_mutable_const)]
    const EMPTY: Self = Self {
        writer: AtomicI32::new(0),
    };

    /// Returns the file descriptor for the writing end of the pipe.
    #[inline]
    pub fn writer_fd(&self) -> &AtomicI32 {
        &self.writer
    }

    /// Returns the writing end of the pipe.
    #[inline]
    pub fn load_writer(&self, ordering: Ordering) -> Writer {
        // SAFETY: In the case that the file descriptor is 0, the `pipe::Writer`
        // still has a valid representation.
        Writer(self.writer_fd().load(ordering))
    }
}
