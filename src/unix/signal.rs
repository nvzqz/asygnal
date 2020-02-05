use super::SignalSet;
use std::mem;

// Required to enable polyfills on non-Unix platforms when documenting.
#[cfg(not(unix))]
use super::libc_polyfill as libc;

use libc::c_int;

macro_rules! signals {
    ($(
        $(#[doc = $doc:literal])+
        $(#[cfg($cfg:meta)])?
        $variant:ident, $method:ident, $libc:ident;
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
                $variant,
            )+
        }

        /// Handling of raw signal values from `libc`.
        impl Signal {
            /// Attempts to create an instance if `signal` is known.
            pub fn from_raw(signal: c_int) -> Option<Self> {
                match signal {
                    $(
                        $(#[cfg($cfg)])?
                        libc::$libc => Some(Self::$variant),
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
                        Self::$variant => libc::$libc,
                    )+
                }
            }
        }

        impl SignalSet {
            $(
                $(#[doc = $doc])+
                $(
                    #[cfg(any(docsrs, $cfg))]
                    #[cfg_attr(docsrs, doc(cfg($cfg)))]
                )?
                #[must_use]
                #[inline]
                pub const fn $method(self) -> Self {
                    self.with(Signal::$variant)
                }
            )+
        }
    };
}

signals! {
    /// The `SIGALRM` signal; sent when a real-time timer expires.
    ///
    /// **Default behavior:** process termination.
    Alarm, alarm, SIGALRM;

    /// The `SIGCHLD` signal; sent when the status of a child process changes.
    ///
    /// **Default behavior:** ignored.
    Child, child, SIGCHLD;

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** process termination.
    Hangup, hangup, SIGHUP;

    /// The `SIGINFO` signal; sent to request a status update from the process.
    ///
    /// **Keyboard shortcut:** `CTRL` + `T`.
    ///
    /// **Default behavior:** ignored.
    #[cfg(any(
        // According to `libc`:
        // "bsd"
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd",
        // "solarish"
        target_os = "solaris",
        target_os = "illumos",
    ))]
    Info, info, SIGINFO;

    /// The `SIGINT` signal; sent to interrupt a program.
    ///
    /// **Keyboard shortcut:** `CTRL` + `C`.
    ///
    /// **Default behavior:** process termination.
    Interrupt, interrupt, SIGINT;

    /// The `SIGIO` signal; sent when I/O operations are possible on some file
    /// descriptor.
    ///
    /// **Default behavior:** ignored.
    Io, io, SIGIO;

    /// The `SIGPIPE` signal; sent when the process attempts to write to a pipe
    /// which has no reader.
    ///
    /// **Default behavior:** process termination.
    Pipe, pipe, SIGPIPE;

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** process termination.
    Quit, quit, SIGQUIT;

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** process termination.
    Terminate, terminate, SIGTERM;

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    UserDefined1, user_defined_1, SIGUSR1;

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    UserDefined2, user_defined_2, SIGUSR2;

    /// The `SIGWINCH` signal; sent when the terminal window is resized.
    ///
    /// **Default behavior:** ignored.
    WindowChange, window_change, SIGWINCH;
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

macro_rules! from_int {
    ($(
        $(#[$meta:meta])+
        $method:ident, $int:ty;
    )+) => {
        /// Conversions from the integer value. This is **not** the same as the
        /// raw `libc` value.
        impl Signal {
            $(
                $(#[$meta])+
                #[inline]
                pub fn $method(signal: $int) -> Option<Self> {
                    // Fine since `MAX_VALUE` is less than `i8::MAX_VALUE`.
                    if signal <= Self::MAX_VALUE as $int {
                        Some(unsafe { mem::transmute(signal as u8) })
                    } else {
                        None
                    }
                }
            )+
        }
    };
}

from_int! {
    /// Creates an instance from an unsigned 8-bit integer.
    from_u8, u8;
    /// Creates an instance from a signed 8-bit integer.
    from_i8, i8;
    /// Creates an instance from an unsigned 16-bit integer.
    from_u16, u16;
    /// Creates an instance from a signed 16-bit integer.
    from_i16, i16;
    /// Creates an instance from an unsigned 32-bit integer.
    from_u32, u32;
    /// Creates an instance from a signed 32-bit integer.
    from_i32, i32;
    /// Creates an instance from an unsigned 64-bit integer.
    from_u64, u64;
    /// Creates an instance from a signed 64-bit integer.
    from_i64, i64;
    /// Creates an instance from an unsigned 128-bit integer.
    from_u128, u128;
    /// Creates an instance from a signed 128-bit integer.
    from_i128, i128;
    /// Creates an instance from an unsigned pointer-sized integer.
    from_usize, usize;
    /// Creates an instance from a signed pointer-sized integer.
    from_isize, isize;
}
