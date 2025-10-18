
/// Contains a `ItchMessage` type that covers all protocol message variants.
pub mod msg;

/// Contains strong types used in ITCH protocol messages.
pub mod types;

/// Contains an `ItchError` type for recoverable errors.
pub mod error;

/// Helper functions used throughout the crate.
pub mod helper;

// Unit tests for the crate:
#[cfg(test)] mod test;

