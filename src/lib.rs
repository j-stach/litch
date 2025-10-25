
/// Contains a `ItchMessage` type that covers all protocol message variants.
pub mod msg;
pub use msg::ItchMessage;

/// Contains an `ItchError` type for recoverable errors.
pub mod error;

// Unit tests for the crate:
#[cfg(test)] mod test;

