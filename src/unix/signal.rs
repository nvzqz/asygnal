use std::mem;

// Required to enable polyfills on non-Unix platforms when documenting.
#[cfg(not(unix))]
use super::libc_polyfill as libc;

use libc::c_int;

macro_rules! kinds {
    ($(
        $(#[doc = $doc:literal])+
        $(#[cfg($cfg:meta)])?
        $name:ident, $libc:ident;
    )+) => {
        /// Signals supported by this library.
        ///
        /// Note that the value, when casted to an integer, may vary depending
        /// on the target platform. This is deliberate. Call the
        /// [`into_raw`](#method.into_raw) method to get the raw signal value
        /// for the target platform.
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[non_exhaustive]
        pub enum Signal {
            $(
                $(#[doc = $doc])+
                $(
                    #[cfg(any(docsrs, $cfg))]
                    #[cfg_attr(docsrs, doc(cfg($cfg)))]
                )?
                $name,
            )+
        }

        /// Handling of raw signal values from `libc`.
        impl Signal {
            /// Attempts to create an instance if `signal` is known.
            pub fn from_raw(signal: c_int) -> Option<Self> {
                match signal {
                    $(
                        $(#[cfg($cfg)])?
                        libc::$libc => Some(Self::$name),
                    )+
                    _ => None,
                }
            }

            /// Returns the raw signal value.
            pub fn into_raw(self) -> c_int {
                #[cfg(docsrs)]
                { -1 }

                #[cfg(not(docsrs))]
                match self {
                    $(
                        $(#[cfg($cfg)])?
                        Self::$name => libc::$libc,
                    )+
                }
            }
        }
    };
}

kinds! {
    /// The `SIGALRM` signal; sent when a real-time timer expires.
    ///
    /// **Default behavior:** process termination.
    Alarm, SIGALRM;

    /// The `SIGCHLD` signal; sent when the status of a child process changes.
    ///
    /// **Default behavior:** ignored.
    Child, SIGCHLD;

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** process termination.
    Hangup, SIGHUP;

    /// The `SIGINFO` signal; sent to request a status update from the process.
    ///
    /// **Keyboard shortcut:** `CTRL` + `T`.
    ///
    /// **Default behavior:** ignored.
    #[cfg(not(any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "linux",
    )))]
    Info, SIGINFO;

    /// The `SIGINT` signal; sent to interrupt a program.
    ///
    /// **Keyboard shortcut:** `CTRL` + `C`.
    ///
    /// **Default behavior:** process termination.
    Interrupt, SIGINT;

    /// The `SIGIO` signal; sent when I/O operations are possible on some file
    /// descriptor.
    ///
    /// **Default behavior:** ignored.
    Io, SIGIO;

    /// The `SIGPIPE` signal; sent when the process attempts to write to a pipe
    /// which has no reader.
    ///
    /// **Default behavior:** process termination.
    Pipe, SIGPIPE;

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** process termination.
    Quit, SIGQUIT;

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** process termination.
    Terminate, SIGTERM;

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    UserDefined1, SIGUSR1;

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    UserDefined2, SIGUSR2;

    /// The `SIGWINCH` signal; sent when the terminal window is resized.
    ///
    /// **Default behavior:** ignored.
    WindowChange, SIGWINCH;
}

impl Signal {
    /// The maximum supported signal.
    pub(crate) const MAX_VALUE: Self = Self::WindowChange;

    /// Number of supported signals.
    pub(crate) const NUM: usize = 1 + Self::MAX_VALUE as usize;

    #[inline]
    pub(crate) unsafe fn from_u8_unchecked(signal: u8) -> Self {
        mem::transmute(signal)
    }

    /// Registers a signal handler that will only be fulfilled once.
    ///
    /// After the `SignalOnce` is fulfilled, all subsequent polls will return
    /// `Ready`.
    #[cfg(feature = "once")]
    pub fn register_once(
        self,
    ) -> Result<
        crate::once::unix::SignalOnce,
        crate::once::unix::RegisterOnceError,
    > {
        crate::once::unix::SignalOnce::register(self)
    }
}
