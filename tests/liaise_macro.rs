//! The `register_liaise_errors!` declarative macro shorthand.
//!
//! Regression coverage: the macro previously passed `slug = stringify!($variant)`,
//! which the derive's `expect_str` rejects (it wants a string *literal*), so the
//! generated enum silently had no `Liaise` impl. The macro now omits `slug` and
//! lets it default to the kebab of the variant name — consistent with a direct
//! `#[derive(LiaiseCodes)]`.

use liaise::{register_liaise_errors, ErrorRegistry, Liaise};

register_liaise_errors! {
    prefix: "MAC",
    name: MacroCode,
    {
        MissingName = 1 => "missing name",
        TooMany = 17 => "too many things",
    }
}

#[test]
fn macro_generates_working_liaise_enum() {
    assert_eq!(MacroCode::MissingName.code(), "MAC0001");
    assert_eq!(MacroCode::MissingName.message(), "missing name");
    assert_eq!(MacroCode::TooMany.code(), "MAC0017");
    assert_eq!(MacroCode::TooMany.message(), "too many things");
    assert_eq!(MacroCode::MissingName.render(), "[MAC0001] missing name");
}

#[test]
fn macro_slug_defaults_to_kebab() {
    assert_eq!(MacroCode::MissingName.slug(), "missing-name");
    assert_eq!(MacroCode::TooMany.slug(), "too-many");
}

#[test]
fn macro_codes_are_unique_and_registered() {
    assert!(MacroCode::ALL_CODES.contains(&1));
    assert!(MacroCode::ALL_CODES.contains(&17));
    assert!(liaise::validate_uniqueness(MacroCode::ALL_CODES));
}
