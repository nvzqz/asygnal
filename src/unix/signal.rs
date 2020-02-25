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
        ///
        /// See ["POSIX Signals"][posix_signals] for more info on some of these.
        ///
        /// [posix_signals]: https://en.wikipedia.org/wiki/Signal_(IPC)#POSIX_signals
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
            /// Number of supported signals.
            pub(crate) const NUM: usize = {
                // Create a duplicate enum except with an explicit final value
                // that is not conditionally compiled. This ensures we can get
                // a max value regardless of target platform.
                #[allow(warnings)]
                enum Signal {
                    $(
                        $(#[cfg($cfg)])?
                        $variant,
                    )+
                    Max,
                }
                Signal::Max as usize
            };

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
                #[inline]
                #[must_use]
                pub const fn $method(self) -> Self {
                    self.with(Signal::$variant, true)
                }
            )+
        }
    };
}

signals! {
    // Signals that cannot be handled must never be included.
    //
    // These are:
    // - SIGKILL
    // - SIGSTOP
    //
    // This library uses fixed-size tables based on the number of signals below.
    // Including such signals would be a waste of space.

    /// The `SIGABRT` signal; sent when the process calls `abort()`.
    ///
    /// If you choose to register a handler for this signal, it is *highly*
    /// recommended to actually terminate the process.
    ///
    /// **Default behavior:** process termination.
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
    Abort, abort, SIGABRT;

    /// The `SIGALRM` signal; sent when a real-time timer expires.
    ///
    /// **Default behavior:** process termination.
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
        target_os = "hermit",
        target_os = "vxworks",
        target_env = "uclibc",
    ))]
    Alarm, alarm, SIGALRM;

    /// The `SIGCHLD` signal; sent when the status of a child process changes.
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
        // "linux-like"
        // Varies but it should be safe to assume all "linux" targets have this.
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
        target_os = "vxworks",
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
    Child, child, SIGCHLD;

    /// The `SIGFPE` ("floating point exception") signal; sent when the process
    /// executes an erroneous arithmetic operation, such as division by zero.
    ///
    /// **Default behavior:** process termination.
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
    Fpe, fpe, SIGFPE;

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** process termination.
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
        target_os = "hermit",
        target_os = "vxworks",
        target_env = "uclibc",
    ))]
    Hangup, hangup, SIGHUP;

    /// The `SIGILL` signal; sent when the process attempts to execute an
    /// **illegal**, malformed, unknown, or privileged instruction.
    ///
    /// This exists mainly for completeness. Handling this signal is *very
    /// difficult* and should be done with great care. You probably just want to
    /// just let the default handler deal with it.
    ///
    /// **Default behavior:** process termination.
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
    Illegal, illegal, SIGILL;

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
    Interrupt, interrupt, SIGINT;

    /// The `SIGIO` signal; sent when I/O operations are possible on some file
    /// descriptor.
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
        // "linux-like"
        // Varies but it should be safe to assume all "linux" targets have this.
        target_os = "linux",
        target_os = "android",
        target_os = "emscripten",
        // "solarish"
        target_os = "solaris",
        target_os = "illumos",
        // Uncategorized
        target_os = "fuchsia",
        target_os = "redox",
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
    Io, io, SIGIO;

    /// The `SIGPIPE` signal; sent when the process attempts to write to a pipe
    /// which has no reader.
    ///
    /// **Default behavior:** process termination.
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
        target_os = "hermit",
        target_os = "vxworks",
        target_env = "uclibc",
    ))]
    Pipe, pipe, SIGPIPE;

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** process termination.
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
        target_os = "hermit",
        target_os = "vxworks",
        target_env = "uclibc",
    ))]
    Quit, quit, SIGQUIT;

    /// The `SIGSEGV` signal; sent when the process has attempted to access a
    /// restricted area of memory.
    ///
    /// This exists mainly for completeness. Handling this signal is *very
    /// difficult* and should be done with great care. You probably just want to
    /// just let the default handler deal with it.
    ///
    /// **Default behavior:** process termination.
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
    SegViolation, seg_violation, SIGSEGV;

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** process termination.
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
    Terminate, terminate, SIGTERM;

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
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
        // Varies but it should be safe to assume all "linux" targets have this.
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
    UserDefined1, user_defined_1, SIGUSR1;

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
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
        // Varies but it should be safe to assume all "linux" targets have this.
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
    UserDefined2, user_defined_2, SIGUSR2;

    /// The `SIGWINCH` signal; sent when the terminal window is resized.
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
        // "linux-like"
        // Varies but it should be safe to assume all "linux" targets have this.
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
    WindowChange, window_change, SIGWINCH;
}

impl Signal {
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
                    if signal < Self::NUM as $int {
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
