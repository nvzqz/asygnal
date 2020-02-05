//! Unix-specific functionality.

use std::{
    io, mem, ptr,
    sync::atomic::Ordering,
    task::{Context, Poll},
};
use tokio::io::PollEvented;

use crate::unix::{pipe, Signal, SignalSet};

mod signal;
mod signal_set;
mod table;

pub use {signal::SignalOnce, signal_set::SignalSetOnce};

/// The event driver for when the pipe can be read.
#[derive(Debug)]
struct Driver(PollEvented<pipe::Reader>);

impl Driver {
    pub fn new(reader: pipe::Reader) -> io::Result<Self> {
        Ok(Self(PollEvented::new(reader)?))
    }

    pub fn poll(&self, cx: &mut Context) -> Poll<()> {
        match self.0.poll_read_ready(cx, mio::Ready::readable()) {
            Poll::Ready(Ok(_)) => Poll::Ready(()),
            Poll::Ready(Err(error)) => panic!("Error on self-pipe: {}", error),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// An error returned when registering a [`Signal`] or [`SignalSet`] fails.
///
/// [`Signal`]:    ../../unix/enum.Signal.html
/// [`SignalSet`]: ../../unix/struct.SignalSet.html
#[derive(Debug)]
pub enum RegisterOnceError {
    /// Signals were already registered.
    Registered(SignalSet),
    /// An I/O error.
    Io(io::Error),
}

impl From<io::Error> for RegisterOnceError {
    #[inline]
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

fn register_signal(signal: Signal) -> io::Result<RegisteredSignal> {
    extern "C" fn signal_handler(signal: libc::c_int) {
        if let Some(signal) = Signal::from_raw(signal) {
            let table = table::Table::global();

            // Set the flag before waking up the reading end.
            table.caught.enable(signal, Ordering::SeqCst);
            table.entry(signal).load_writer(Ordering::SeqCst).wake();
        }
    }

    let raw_signal = signal.into_raw();

    // A custom `sigaction` union type is used because:
    //
    // 1. The `sa_handler` field is used regardless of platform, since `libc`
    //    specifies some having only `sa_sigaction` or `sa_handler`. This is a
    //    restriction based on Rust not having had unions at the time.
    //
    // 2. The union allows for ensuring the correct offset for the `sa_flags`
    //    field and overall size/alignment of the type.
    let new_action = {
        #[allow(non_camel_case_types)]
        union sigaction {
            sa_handler: Option<extern "C" fn(signal: libc::c_int)>,
            libc: libc::sigaction,
        }

        unsafe {
            let mut action: sigaction = mem::zeroed();
            action.sa_handler = Some(signal_handler);
            action.libc.sa_flags = libc::SA_RESTART | libc::SA_NOCLDSTOP;
            action.libc
        }
    };

    let mut old_action: libc::sigaction = unsafe { mem::zeroed() };

    match unsafe { libc::sigaction(raw_signal, &new_action, &mut old_action) } {
        0 => Ok(RegisteredSignal {
            raw_signal,
            old_action,
        }),
        _ => Err(io::Error::last_os_error()),
    }
}

struct RegisteredSignal {
    pub raw_signal: libc::c_int,
    pub old_action: libc::sigaction,
}

impl RegisteredSignal {
    pub fn reset(&self) {
        unsafe {
            libc::sigaction(self.raw_signal, &self.old_action, ptr::null_mut());
        }
    }
}
