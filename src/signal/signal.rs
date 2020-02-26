use super::SignalSet;
use std::mem;

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

        const SIGNAL_NUM: usize = {
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
                #[inline]
                #[must_use]
                pub const fn $method(self) -> Self {
                    self.with(Signal::$variant)
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
    /// **Default behavior:** terminate (core dump).
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
    /// **Default behavior:** terminate.
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

    /// The `SIGBUS` signal; sent when the process causes a [bus error], e.g.
    /// due to incorrect memory access alignment or non-existent physical
    /// address.
    ///
    /// **Default behavior:** terminate (core dump).
    ///
    /// [bus error]: https://en.wikipedia.org/wiki/Bus_error
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
    Bus, bus, SIGBUS;

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

    /// The `SIGCONT` signal; sent when the process is **continued** after being
    /// previously paused by the `SIGSTOP` or `SIGTSTP` signal.
    ///
    /// **Default behavior:** continue executing, if stopped.
    ///
    /// **Note:** the `Cont` and `cont` identifiers are used over `Continue` and
    /// `continue` because `continue` is a keyword in Rust.
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
    Cont, cont, SIGCONT;

    /// The `SIGFPE` ("**float**ing point **exc**eption") signal; sent when the
    /// process executes an erroneous arithmetic operation, such as division by
    /// zero.
    ///
    /// **Default behavior:** terminate (core dump).
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
    FloatExc, float_exc, SIGFPE;

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** terminate.
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
    /// **Default behavior:** terminate (core dump).
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
    IllInstr, ill_instr, SIGILL;

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
    /// **Default behavior:** terminate.
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
    /// **Default behavior:** terminate.
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

    /// The `SIGPOLL` signal; sent when an event occurred on an explicitly
    /// watched file descriptor.
    ///
    /// **Default behavior:** ignored.
    #[cfg(any(
        // According to `libc`:
        // "linux-like"
        target_os = "linux",
        target_os = "android",
        target_os = "emscripten",
        // "solarish"
        target_os = "solaris",
        target_os = "illumos",
        // Uncategorized
        target_os = "fuchsia",
        target_os = "haiku",
        all(
            target_env = "uclibc",
            any(
                target_arch = "mips",
                target_arch = "mips64",
            ),
        ),
    ))]
    Poll, poll, SIGPOLL;

    /// The `SIGPROF` signal; sent when the limit for CPU time used by the
    /// process and by the system on behalf of the process elapses.
    ///
    /// **Default behavior:** terminate.
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
    Profile, profile, SIGPROF;

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** terminate (core dump).
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
    /// **Default behavior:** terminate (core dump).
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

    /// The `SIGTSTP` signal; sent when the terminal requests the process to
    /// stop.
    ///
    /// **Keyboard shortcut:** `CTRL` + `Z`.
    ///
    /// **Default behavior:** stop process.
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
    TermStop, term_stop, SIGTSTP;

    /// The `SIGSYS` signal; sent when a non-existent system call is invoked.
    ///
    /// **Default behavior:** terminate (core dump).
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
    System, system, SIGSYS;

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** terminate.
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

    /// The `SIGTRAP` signal; sent when an exception (or **trap**) occurs.
    ///
    /// **Default behavior:** terminate (core dump).
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
        target_os = "vxworks",
        target_env = "uclibc",
    ))]
    Trap, trap, SIGTRAP;

    /// The `SIGTTIN` signal; sent when the process attempts to read **in** from
    /// the [tty] when in the [background].
    ///
    /// **Default behavior:** stop process.
    ///
    /// [tty]:        https://en.wikipedia.org/wiki/Teletypewriter
    /// [background]: https://en.wikipedia.org/wiki/Background_process
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
    TtIn, tt_in, SIGTTIN;

    /// The `SIGTTOU` signal; sent when the process attempts to write **out** to
    /// the [tty] when in the [background].
    ///
    /// **Default behavior:** stop process.
    ///
    /// [tty]:        https://en.wikipedia.org/wiki/Teletypewriter
    /// [background]: https://en.wikipedia.org/wiki/Background_process
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
    TtOut, tt_out, SIGTTOU;

    /// The `SIGURG` signal; sent when a [socket] has **urgent** or
    /// [out-of-band data] available to read.
    ///
    /// **Default behavior:** ignored.
    ///
    /// [socket]:           https://en.wikipedia.org/wiki/Berkeley_sockets
    /// [out-of-band data]: https://en.wikipedia.org/wiki/Out-of-band_data
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
    Urgent, urgent, SIGURG;

    /// The `SIGVTALRM` signal; sent when the limit for CPU time used by the
    /// process elapses.
    ///
    /// **Default behavior:** terminate.
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
    VtAlarm, vt_alarm, SIGVTALRM;

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** terminate.
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
    UserDef1, user_def_1, SIGUSR1;

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** terminate.
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
    UserDef2, user_def_2, SIGUSR2;

    /// The `SIGXCPU` signal; sent when the process has used up the CPU for a
    /// duration that **exceeds** a certain predetermined user-settable value.
    ///
    /// The arrival of a `SIGXCPU` signal provides the receiving process a
    /// chance to quickly save any intermediate results and to exit gracefully,
    /// before it is terminated by the operating system using the `SIGKILL`
    /// signal.
    ///
    /// See also:
    ///
    /// - `setrlimit(2)`
    ///
    /// **Default behavior:** terminate (core dump).
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
    XCpu, x_cpu, SIGXCPU;

    /// The `SIGXFSZ` signal; sent when the process grows a file that
    /// **exceeds** the maximum allowed size.
    ///
    /// See also:
    ///
    /// - `setrlimit(2)`
    ///
    /// **Default behavior:** terminate (core dump).
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
    XFileSize, x_file_size, SIGXFSZ;

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
    /// The number of supported signals.
    ///
    /// This value is exempted from [semantic versioning](https://semver.org)
    /// because it may change in the future as more signals are added.
    pub const NUM: usize = SIGNAL_NUM;

    /// Returns the set of all supported signals.
    #[inline]
    pub const fn all() -> SignalSet {
        SignalSet::all()
    }

    #[inline]
    pub(crate) unsafe fn from_u8_unchecked(signal: u8) -> Self {
        mem::transmute(signal)
    }

    /// Registers a signal handler that will only be fulfilled once.
    ///
    /// After the `SignalOnce` is fulfilled, all subsequent polls will return
    /// `Ready`.
    #[cfg(any(docsrs, feature = "once"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "once")))]
    pub fn register_once(
        self,
    ) -> Result<
        crate::once::signal::SignalOnce,
        crate::once::signal::RegisterOnceError,
    > {
        crate::once::signal::SignalOnce::register(self)
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
                        Some(unsafe { Self::from_u8_unchecked(signal as u8) })
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
