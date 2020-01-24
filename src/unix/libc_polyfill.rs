#![allow(non_camel_case_types, private_in_public)]

// This module exists to enable docs.rs to show Unix when compiling for Windows.

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Opaque;

pub type sigset_t = Opaque;
pub type c_int = i32;

pub fn sigemptyset(_: *mut sigset_t) -> c_int {
    -1
}
pub fn sigaddset(_: *mut sigset_t, _: c_int) {}

pub const SIGALRM: c_int = -1;
pub const SIGCHLD: c_int = -1;
pub const SIGHUP: c_int = -1;
pub const SIGINFO: c_int = -1;
pub const SIGINT: c_int = -1;
pub const SIGIO: c_int = -1;
pub const SIGPIPE: c_int = -1;
pub const SIGQUIT: c_int = -1;
pub const SIGTERM: c_int = -1;
pub const SIGUSR1: c_int = -1;
pub const SIGUSR2: c_int = -1;
pub const SIGWINCH: c_int = -1;
