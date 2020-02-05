use crate::unix::{
    pipe::Writer, signal_mask::AtomicSignalMask, Signal, SignalTable,
};
use std::sync::atomic::{AtomicI32, Ordering};

pub(crate) struct Table {
    pub registered: AtomicSignalMask,
    pub caught: AtomicSignalMask,
    entries: SignalTable<Entry>,
}

impl Table {
    #[inline]
    pub fn global() -> &'static Self {
        static GLOBAL: Table = Table {
            registered: AtomicSignalMask::empty(),
            caught: AtomicSignalMask::empty(),
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