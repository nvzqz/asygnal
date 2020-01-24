// Required to enable polyfills on non-Unix platforms when documenting.
#[cfg(not(unix))]
use super::libc_polyfill as libc;

/// A specific kind of signal.
///
/// Signals that cannot be handled are not listed as constants.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignalKind(libc::c_int);

// Needed in case `libc::SIGINFO` is not defined on the current platform and
// thus prevents the docs from compiling.
#[cfg(not(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
)))]
const SIGINFO: libc::c_int = libc::SIGINFO;

#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
))]
#[allow(unused)]
const SIGINFO: libc::c_int = -1;

impl SignalKind {
    // Constants are used so then they can be `match`ed on.

    // REMINDER: When updating the documentation of the following constants,
    // their corresponding `SignalSetBuilds` methods must be updated as well.

    /// The `SIGALRM` signal; sent when a real-time timer expires.
    ///
    /// **Default behavior:** process termination.
    pub const ALARM: Self = Self(libc::SIGALRM);

    /// The `SIGCHLD` signal; sent when the status of a child process changes.
    ///
    /// **Default behavior:** ignored.
    pub const CHILD: Self = Self(libc::SIGCHLD);

    /// The `SIGHUP` signal; sent when the terminal is disconnected.
    ///
    /// **Default behavior:** process termination.
    pub const HANGUP: Self = Self(libc::SIGHUP);

    /// The `SIGINFO` signal; sent to request a status update from the process.
    ///
    /// **Not supported on:** `android`, `emscripten`, `linux`.
    ///
    /// **Keyboard shortcut:** `CTRL` + `T`.
    ///
    /// **Default behavior:** ignored.
    #[cfg(any(
        not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "linux",
        )),
        docsrs,
    ))]
    // This doesn't seem to change docs to list the supported target OSes.
    #[cfg_attr(
        docsrs,
        doc(not(any(
            target_os = "android",
            target_os = "emscripten",
            target_os = "linux",
        )))
    )]
    pub const INFO: Self = Self(SIGINFO);

    /// The `SIGINT` signal; sent to interrupt a program.
    ///
    /// **Keyboard shortcut:** `CTRL` + `C`.
    ///
    /// **Default behavior:** process termination.
    pub const INTERRUPT: Self = Self(libc::SIGINT);

    /// The `SIGIO` signal; sent when I/O operations are possible on some file
    /// descriptor.
    ///
    /// **Default behavior:** ignored.
    pub const IO: Self = Self(libc::SIGIO);

    /// The `SIGPIPE` signal; sent when the process attempts to write to a pipe
    /// which has no reader.
    ///
    /// **Default behavior:** process termination.
    pub const PIPE: Self = Self(libc::SIGPIPE);

    /// The `SIGQUIT` signal; sent to issue a shutdown of the process, after
    /// which the OS will dump the process core.
    ///
    /// **Keyboard shortcut:** `CTRL` + `\`.
    ///
    /// **Default behavior:** process termination.
    pub const QUIT: Self = Self(libc::SIGQUIT);

    /// The `SIGTERM` signal; sent to issue a shutdown of the process.
    ///
    /// **Default behavior:** process termination.
    pub const TERMINATE: Self = Self(libc::SIGTERM);

    /// The `SIGUSR1` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    pub const USER_DEFINED_1: Self = Self(libc::SIGUSR1);

    /// The `SIGUSR2` signal; a user defined signal.
    ///
    /// **Default behavior:** process termination.
    pub const USER_DEFINED_2: Self = Self(libc::SIGUSR2);

    /// The `SIGWINCH` signal; sent when the terminal window is resized.
    ///
    /// **Default behavior:** ignored.
    pub const WINDOW_CHANGE: Self = Self(libc::SIGWINCH);

    /// Creates an instance from the raw signal value.
    ///
    /// # Safety
    ///
    /// This library assumes that all signals used are valid. Supplying an
    /// unsupported signal number invalidates this assumption.
    #[inline]
    pub const unsafe fn from_raw(signal: libc::c_int) -> Self {
        Self(signal)
    }

    /// Returns the raw value of this signal.
    #[inline]
    pub const fn into_raw(self) -> libc::c_int {
        self.0
    }
}
