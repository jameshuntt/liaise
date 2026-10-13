#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

extern crate self as liaise;

/// Paths the derive expands to. Not part of the public API.
#[doc(hidden)]
pub mod __private {
    pub use alloc::{format, vec};
    pub use alloc::string::{String, ToString};
    pub use alloc::vec::Vec;
    pub use core::error::Error;
    pub use core::option::Option;
}

pub mod diagnostic;
pub mod event;
pub mod loc;
pub mod report;

#[cfg(feature = "syn-error")]
pub mod adapters;
#[macro_use]
pub mod macros;

pub use diagnostic::{
    validate_uniqueness, Combine, DiagBuffer, Diagnostic, ErrorRegistry, Liaise,
    LiaiseRegistry, LiaiseRegistryError, LiaiseRegistryResult,
};
pub use event::{LiaiseAccumulator, LiaiseEvent};
pub use loc::{DiagnosticLoc, LineLoc};
pub use report::{
    LiaiseDescriptor, LiaiseEvidence, LiaiseReport, LiaiseSeverity, LiaiseStability,
    LiaiseToken,
};
// `LiaiseDigest` is kept as a stable alias; the implementation now lives in the
// generic `stable_digest` primitive (content-addressing doesn't belong in a
// diagnostics crate).
pub use ::stable_digest::StableDigest as LiaiseDigest;
pub use liaise_derive::{LiaiseCodes, RegisterErrors};
