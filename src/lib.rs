#![no_std]
extern crate alloc;

pub mod adapters;
pub mod diagnostic;
pub mod loc;
#[macro_use]pub mod macros;

pub use diagnostic::{
    Combine,
    DiagBuffer,
    Diagnostic,
    ErrorRegistry,
    Liaise,
    validate_uniqueness
};
pub use loc::DiagnosticLoc;

pub use liaise_derive::RegisterErrors;
