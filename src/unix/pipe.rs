use mio::{unix::EventedFd, Evented};
use std::{io, mem::MaybeUninit, os::unix::io::RawFd};

/// A pipe suitable for signal handling.
///
/// Note: because this pipe is expected to live throughout the duration of the
/// program, neither end implements `Drop` to close its file descriptors.
pub(crate) fn pipe() -> io::Result<(Reader, Writer)> {
    pipe_impl()
}

/// The sending end of the pipe.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct Writer(pub RawFd);

impl Writer {
    /// Wakes up the reading end of the pipe.
    ///
    /// It is imperative that this function is signal-safe.
    #[inline]
    pub fn wake(&self) {
        let buf: [u8; 1] = [1u8];
        unsafe {
            // Wake up any reader. Any errors are ignored since it's likely a
            // full pipe, in which case it will wake up anyway. Also, there's no
            // reasonable way to handle errors from within the signal handler.
            libc::write(self.0, buf.as_ptr() as *const _, buf.len() as _);
        }
    }
}

/// The receiving end of the pipe.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Reader(pub RawFd);

impl Evented for Reader {
    #[inline]
    fn register(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.0).register(poll, token, interest, opts)
    }

    #[inline]
    fn reregister(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.0).reregister(poll, token, interest, opts)
    }

    #[inline]
    fn deregister(&self, poll: &mio::Poll) -> std::io::Result<()> {
        EventedFd(&self.0).deregister(poll)
    }
}

#[cfg(any(
    // Targets known to have `libc::pipe2`:
    target_env = "uclibc",
    target_os = "redox",
    target_os = "fuchsia",
    // "linux-like"
    target_os = "linux",
    target_os = "android",
    target_os = "emscripten",
    // "netbsd-like"
    target_os = "openbsd",
    target_os = "netbsd",
    // "freebsd-like"
    target_os = "freebsd",
    target_os = "dragonfly",
    // "solarish"
    target_os = "solaris",
    target_os = "illumos",
))]
fn pipe_impl() -> io::Result<(Reader, Writer)> {
    let [reader, writer] = {
        let flags = libc::O_NONBLOCK | libc::O_CLOEXEC;

        let mut fds: MaybeUninit<[RawFd; 2]> = MaybeUninit::uninit();
        let error = unsafe { libc::pipe2(fds.as_mut_ptr() as _, flags) };
        if error != 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { fds.assume_init() }
    };

    Ok((Reader(reader), Writer(writer)))
}

// Copies the functionality of `libc::pipe2` for platforms that aren't known to
// have it by setting the flags on the file descriptors with `libc::fcntl`.
#[cfg(not(any(
    target_env = "uclibc",
    target_os = "redox",
    target_os = "fuchsia",
    // "linux-like"
    target_os = "linux",
    target_os = "android",
    target_os = "emscripten",
    // "netbsd-like"
    target_os = "openbsd",
    target_os = "netbsd",
    // "freebsd-like"
    target_os = "freebsd",
    target_os = "dragonfly",
    // "solarish"
    target_os = "solaris",
    target_os = "illumos",
)))]
fn pipe_impl() -> io::Result<(Reader, Writer)> {
    // Create the read/write ends of the pipe, returning any error.
    let [reader, writer] = {
        let mut fds: MaybeUninit<[RawFd; 2]> = MaybeUninit::uninit();
        let error = unsafe { libc::pipe(fds.as_mut_ptr() as *mut RawFd) };
        if error != 0 {
            return Err(io::Error::last_os_error());
        }

        unsafe { fds.assume_init() }
    };

    // Sets the `O_NONBLOCK` and `O_CLOEXEC` flags on the file descriptor.
    fn set_flags(fd: RawFd) -> io::Result<()> {
        let error = unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) };
        if error != 0 {
            return Err(io::Error::last_os_error());
        }

        let error = unsafe { libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) };
        if error != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    if let Err(error) = set_flags(reader).and_then(|_| set_flags(writer)) {
        // Close the pipe, ignoring any errors since the one we care about is
        // from setting the flags.
        unsafe {
            libc::close(reader);
            libc::close(writer);
        }
        return Err(error);
    }

    Ok((Reader(reader), Writer(writer)))
}
