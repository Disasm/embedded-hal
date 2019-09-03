//! Digital I/O
//!
//! 
//!

// Infallible traits
pub mod v1;

// Fallible traits
#[deprecated(since = "0.2.3", note = "Deprecated due to the name collision with v1 traits. \
                                      Driver developers should use the traits in digital::v3.")]
pub mod v2;

// New fallible traits
pub mod v3;

// v2 -> v1 compatibility wrappers
// These require explicit casts from v2 -> v1
pub mod v1_compat;

// v1 -> v2 compatibility shims
// These are implicit over v1 implementations
pub mod v2_compat;

// v2 -> v3 compatibility shims
// These are implicit over v1 and v2 implementations
pub mod v3_compat;

// Re-export old traits so this isn't a breaking change
pub use self::v1::*;

