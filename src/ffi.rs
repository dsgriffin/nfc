//! Raw re-exports from [`nfc-sys`].
//!
//! Use this module when you need libnfc features that cannot be wrapped in a
//! general safe API, such as custom driver registration or emulator callback
//! state machines.
//!
//! Most applications should start with [`crate::Context`] and [`crate::Device`]
//! instead.

pub use nfc_sys::*;
