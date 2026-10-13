#[cfg(feature = "syn-error")]
pub mod syn_impls {
    use crate::diagnostic::{Combine, Liaise};
    use alloc::format;
    use proc_macro2::Span;
    use quote::ToTokens;
    use syn::Error;

    impl Combine for Error {
        fn combine(&mut self, other: Self) { self.combine(other); }
    }

    pub fn err_at<L: Liaise>(span: Span, code: L) -> Error {
        Error::new(span, code.render())
    }

    pub fn err_at_ctx<L: Liaise>(span: Span, code: L, ctx: impl core::fmt::Display) -> Error {
        Error::new(span, format!("{}: {}", code.render(), ctx))
    }

    pub fn err_spanned<L: Liaise>(tokens: impl ToTokens, code: L) -> Error {
        Error::new_spanned(tokens, code.render())
    }
}
