//! Unix-specific functionality.

mod signal_kind;
mod signal_set;

#[doc(inline)]
pub use self::{signal_kind::*, signal_set::*};

/// A future for receiving a particular signal.
#[derive(Debug)]
pub struct Signal {
    _private: (),
}
