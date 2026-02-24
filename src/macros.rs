#[macro_export]macro_rules! register_liaise_errors {
    (
        prefix: $prefix:expr,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive($crate::RegisterErrors, Copy, Clone)]
        #[error_prefix = $prefix]
        pub enum $name {
            $($variant = $code),*
        }

        impl $crate::Liaise for $name {
            fn code_id(self) -> u16 { self as u16 }
            fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }
        }
    };
    (
        prefix: $prefix:expr,
        vis: $vis:vis,
        name: $name:ident,
        {
            $($variant:ident = $code:expr => $msg:expr),* $(,)?
        }
    ) => {
        #[derive($crate::RegisterErrors, Copy, Clone)]
        #[error_prefix = $prefix]
        $vis enum $name {
            $($variant = $code),*
        }

        impl liaise::Liaise for $name {
            fn code_id(self) -> u16 { self as u16 }
            fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $msg),*
                }
            }
        }
    };
}


extern crate std as std;
use crate::diagnostic::Liaise;
use liaise_derive::LiaiseCodes;
#[cfg(feature = "std")]
#[derive(LiaiseCodes, Debug)]
#[liaise(prefix = "ABUT")]
pub enum AbutCode {
    #[liaise(code = 1, msg = "I/O failure", source)]
    Io(std::io::Error),
    #[liaise(code = 2, msg = "Buffer too small (need {needed} bytes)")]
    BufferTooSmall { needed: usize },
}


#[cfg(not(feature = "std"))]
#[derive(Copy, Clone, LiaiseCodes)]
#[error_prefix("ABUT")]
pub enum AbutCode {
    #[liaise(code = 1, msg = "I/O failure")]
    Io,

    #[liaise(code = 2, msg = "Buffer too small (need {needed} bytes)")]
    BufferTooSmall { needed: usize },
}

#[test]
fn test_abut_code_rendering() {
    // Test Variant 1 (Unit/Tuple)
    #[cfg(feature = "std")]
    let err_io = AbutCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no"));
    #[cfg(not(feature = "std"))]
    let err_io = AbutCode::Io;

    assert_eq!(err_io.code_id(), 1);
    assert_eq!(AbutCode::prefix(), "ABUT");
    assert!(err_io.render().contains("[ABUT0001]"));
    assert!(err_io.render().contains("I/O failure"));

    // Test Variant 2 (Struct with fields)
    let err_buf = AbutCode::BufferTooSmall { needed: 64 };
    assert_eq!(err_buf.code_id(), 2);
    // Verifies that the {needed} interpolation worked via alloc::format!
    assert!(err_buf.render().contains("need 64 bytes"));
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_constraints() {
    let err = AbutCode::Io;
    
    // Test Copy: if we move it into a function, we should still be able to use it
    fn move_it(e: AbutCode) -> u16 { e.code_id() }
    
    let _ = move_it(err);
    let id = err.code_id(); // This only compiles if AbutCode is Copy
    assert_eq!(id, 1);
}

#[cfg(feature = "std")]
#[test]
fn test_std_source_propagation() {
    use alloc::string::{String, ToString}; // Add this to the top of macros.rs
    use std::error::Error;
    let raw_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let err = AbutCode::Io(raw_err);
    
    // Check that we can treat it as a trait object
    let trait_obj: &dyn Error = &err;
    
    // Check that source() actually returns the inner IO error
    assert!(trait_obj.source().is_some());
    assert_eq!(trait_obj.source().unwrap().to_string(), "file missing");
}
use alloc::string::String; // Add this to the top of macros.rs
use crate::Combine;
impl Combine for String {
    fn combine(&mut self, other: Self) {
        self.push('\n'); // Add a separator
        self.push_str(&other);
    }
}
#[test]
fn test_diagnostic_buffer_flow() {
use alloc::string::ToString; // Add this to the top of macros.rs
    // Mock location for testing
    #[derive(Clone, Copy)]
    struct MockLoc;
    impl crate::loc::DiagnosticLoc for MockLoc {
        fn source_display(&self) -> String { "line:1".to_string() }
    }

    let mut buffer = crate::DiagBuffer::<MockLoc, AbutCode>::new();
    
    // Push a standard error
    buffer.push(MockLoc, AbutCode::BufferTooSmall { needed: 128 });
    
//     // Push with extra context
//     buffer.push_ctx(MockLoc, AbutCode::Io, "Network timeout during read");
    // Inside test_diagnostic_buffer_flow()
#[cfg(feature = "std")]
buffer.push_ctx(MockLoc, AbutCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "test")), "Network timeout");

#[cfg(not(feature = "std"))]
buffer.push_ctx(MockLoc, AbutCode::Io, "Network timeout");

    // This proves the traits work together to allow buffer processing
    let result = buffer.finish(|msg, _loc| {
        // Just a simple collector for the test
        msg 
    });

    assert!(result.is_err());
}