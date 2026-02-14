#[cfg(feature = "syn-error")]
mod syn_impls {
    use super::*;
    use crate::diagnostic::{Combine, DiagnosticBuffer, Liaise};
    use crate::loc::DiagnosticLoc;
    use alloc::format;
    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::Error;

    impl DiagnosticLoc for proc_macro2::Span {
        fn source_display(&self) -> alloc::string::String {
            alloc::format!("{:?}", self)
        }
    }

    // 1. Teach the core how to merge syn errors
    impl Combine for Error {
        fn combine(&mut self, other: Self) {
            self.combine(other);
        }
    }

    // 2. Provide the "Concrete" version for proc-macros
    pub type SynBuffer = DiagnosticBuffer<syn::Error>;

    pub trait SynBufferExt {
        fn push_at<L: Liaise>(&mut self, span: Span, code: L);
        fn push_at_ctx<L: Liaise>(&mut self, span: Span, code: L, ctx: impl core::fmt::Display);
        fn push_spanned<L: Liaise>(&mut self, tokens: impl ToTokens, code: L);
    }

    impl SynBufferExt for SynBuffer {
        #[inline]
        fn push_at<L: Liaise>(&mut self, span: Span, code: L) {
            self.push(err_at(span, code));
        }

        #[inline]
        fn push_at_ctx<L: Liaise>(&mut self, span: Span, code: L, ctx: impl core::fmt::Display) {
            self.push(err_at_ctx(span, code, ctx));
        }

        #[inline]
        fn push_spanned<L: Liaise>(&mut self, tokens: impl ToTokens, code: L) {
            self.push(err_spanned(tokens, code));
        }
    }

    /// A generic version of your err_at helper
    pub fn err_at<L: Liaise>(span: Span, code: L) -> Error {
        Error::new(span, code.render())
    }

    pub fn err_at_ctx<L: Liaise>(span: Span, code: L, ctx: impl core::fmt::Display) -> Error {
        Error::new(span, format!("{}: {}", code.render(), ctx))
    }

    /// A generic version of your spanned helper
    pub fn err_spanned<L: Liaise>(tokens: impl ToTokens, code: L) -> Error {
        Error::new_spanned(tokens, code.render())
    }
}
