//! syn-error adapter: turning `Liaise` codes into `syn::Error`, plus the
//! `Combine` impl that lets a whole expansion collapse to one compile error.
//!
//! Only compiles/runs under `--features syn-error`.
#![cfg(feature = "syn-error")]

use liaise::adapters::syn_impls::{err_at, err_at_ctx, err_spanned};
use liaise::{Combine, LiaiseCodes};
use proc_macro2::Span;
use quote::quote;

#[derive(Debug, Copy, Clone, LiaiseCodes)]
#[liaise(prefix = "SYN")]
enum Code {
    #[liaise(code = 1, msg = "bad thing happened")]
    Bad,
    #[liaise(code = 2, msg = "second bad thing")]
    Bad2,
}

#[test]
fn err_at_uses_the_rendered_message() {
    let e = err_at(Span::call_site(), Code::Bad);
    assert_eq!(e.to_string(), "[SYN0001] bad thing happened");
}

#[test]
fn err_at_ctx_appends_context() {
    let e = err_at_ctx(Span::call_site(), Code::Bad, "on field `x`");
    assert_eq!(e.to_string(), "[SYN0001] bad thing happened: on field `x`");
}

#[test]
fn err_spanned_carries_the_message() {
    let tokens = quote! { struct Foo; };
    let e = err_spanned(tokens, Code::Bad);
    assert_eq!(e.to_string(), "[SYN0001] bad thing happened");
}

#[test]
fn combine_merges_two_syn_errors_into_one() {
    let mut a = err_at(Span::call_site(), Code::Bad);
    let b = err_at(Span::call_site(), Code::Bad2);
    a.combine(b);
    // A combined syn::Error iterates over each contained error.
    let msgs: Vec<String> = a.into_iter().map(|e| e.to_string()).collect();
    assert_eq!(msgs.len(), 2, "combined error should hold two diagnostics");
    assert!(msgs.iter().any(|m| m.contains("SYN0001")));
    assert!(msgs.iter().any(|m| m.contains("SYN0002")));
}

#[test]
fn combined_error_to_compile_error_is_nonempty() {
    let mut a = err_at(Span::call_site(), Code::Bad);
    a.combine(err_at(Span::call_site(), Code::Bad2));
    let ts = a.to_compile_error();
    assert!(!ts.is_empty());
    let s = ts.to_string();
    assert!(s.contains("compile_error"));
}
