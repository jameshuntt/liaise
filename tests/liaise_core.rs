//! Core diagnostic surface: the `Liaise` trait, code formatting, registry,
//! `DiagBuffer` collection, and `Combine`.

use liaise::{
    Combine, DiagBuffer, Liaise, LiaiseRegistry, LiaiseRegistryError, LiaiseSeverity,
    LiaiseStability, LineLoc, validate_uniqueness, LiaiseCodes,
};

#[derive(Debug, Copy, Clone, LiaiseCodes)]
#[liaise(prefix = "TST")]
enum Code {
    #[liaise(code = 1, msg = "the first problem")]
    First,
    #[liaise(code = 42, msg = "answer problem", severity = "warning", stability = "stable")]
    Answer,
    #[liaise(code = 1234, msg = "big code", category = "layout", help = "try smaller numbers")]
    Big,
    #[liaise(code = 7, msg = "unknown field `{name}`")]
    WithField { name: &'static str },
    #[liaise(code = 8, msg = "tuple variant")]
    Tuple(#[allow(dead_code)] u32),
}

// ---------- code() / render() formatting ----------

#[test]
fn code_is_prefix_plus_zero_padded_id() {
    assert_eq!(Code::First.code(), "TST0001");
    assert_eq!(Code::Answer.code(), "TST0042");
}

#[test]
fn code_id_over_four_digits_is_not_truncated() {
    assert_eq!(Code::Big.code_id(), 1234);
    assert_eq!(Code::Big.code(), "TST1234");
}

#[test]
fn render_wraps_code_and_message() {
    assert_eq!(Code::First.render(), "[TST0001] the first problem");
}

#[test]
fn prefix_and_code_id_come_from_the_derive() {
    assert_eq!(Code::First.prefix(), "TST");
    assert_eq!(Code::First.code_id(), 1);
    assert_eq!(Code::Answer.code_id(), 42);
}

// ---------- message interpolation ----------

#[test]
fn message_interpolates_named_fields() {
    assert_eq!(Code::WithField { name: "widget" }.message(), "unknown field `widget`");
}

#[test]
fn message_plain_when_no_fields() {
    assert_eq!(Code::First.message(), "the first problem");
    assert_eq!(Code::Tuple(9).message(), "tuple variant");
}

// ---------- severity / stability / category / help defaults + overrides ----------

#[test]
fn defaults_apply_when_unspecified() {
    assert_eq!(Code::First.severity(), LiaiseSeverity::Error);
    assert_eq!(Code::First.stability(), LiaiseStability::Experimental);
    assert_eq!(Code::First.category(), None);
    assert_eq!(Code::First.help(), None);
    assert_eq!(Code::First.domain(), "diagnostic");
}

#[test]
fn attributes_override_defaults() {
    assert_eq!(Code::Answer.severity(), LiaiseSeverity::Warning);
    assert_eq!(Code::Answer.stability(), LiaiseStability::Stable);
    assert_eq!(Code::Big.category(), Some("layout"));
    assert_eq!(Code::Big.help(), Some("try smaller numbers"));
}

// ---------- slug defaults ----------

#[test]
fn slug_defaults_to_kebab_of_variant() {
    // `WithField` -> kebab -> "with-field" (derive default)
    assert_eq!(Code::WithField { name: "x" }.slug(), "with-field");
    assert_eq!(Code::First.slug(), "first");
}

// ---------- descriptor / report ----------

#[test]
fn descriptor_carries_every_field() {
    let d = Code::Big.descriptor();
    assert_eq!(d.code, "TST1234");
    assert_eq!(d.prefix, "TST");
    assert_eq!(d.code_id, 1234);
    assert_eq!(d.category.as_deref(), Some("layout"));
    assert_eq!(d.help.as_deref(), Some("try smaller numbers"));
    assert_eq!(d.severity, LiaiseSeverity::Error);
    assert_eq!(d.message_template, "big code");
}

#[test]
fn report_matches_descriptor_code() {
    let r = Code::Answer.liaise_report();
    assert_eq!(r.code, "TST0042");
}

// ---------- ALL_CODES / uniqueness ----------

#[test]
fn all_codes_contains_every_variant() {
    use liaise::ErrorRegistry;
    let all = Code::ALL_CODES;
    for c in [1u16, 42, 1234, 7, 8] {
        assert!(all.contains(&c), "missing {c} in {all:?}");
    }
}

#[test]
fn validate_uniqueness_logic() {
    assert!(validate_uniqueness(&[]));
    assert!(validate_uniqueness(&[1]));
    assert!(validate_uniqueness(&[1, 2, 3]));
    assert!(!validate_uniqueness(&[1, 2, 1]));
    assert!(!validate_uniqueness(&[5, 5]));
}

// ---------- LiaiseRegistry ----------

#[test]
fn registry_rejects_empty_id() {
    let err = LiaiseRegistry::new("   ", vec![]).unwrap_err();
    assert_eq!(err, LiaiseRegistryError::EmptyRegistryId);
}

#[test]
fn registry_dedups_and_looks_up_by_code() {
    let reg = LiaiseRegistry::new(
        "tst",
        vec![Code::First.descriptor(), Code::Answer.descriptor()],
    )
    .expect("valid registry");
    assert!(reg.descriptor_by_code("TST0001").is_some());
    assert!(reg.descriptor_by_code("TST0042").is_some());
    assert!(reg.descriptor_by_code("TST9999").is_none());
}

#[test]
fn registry_rejects_duplicate_code() {
    let dup = Code::First.descriptor();
    let err = LiaiseRegistry::new("tst", vec![dup.clone(), dup]).unwrap_err();
    match err {
        LiaiseRegistryError::DuplicateCode { code } => assert_eq!(code, "TST0001"),
        other => panic!("expected DuplicateCode, got {other:?}"),
    }
}

// ---------- DiagBuffer collection ----------

#[test]
fn empty_diagbuffer_finishes_ok() {
    let buf: DiagBuffer<LineLoc, Code> = DiagBuffer::new();
    let out: Result<(), String> = buf.finish(|msg, _loc| msg);
    assert!(out.is_ok());
}

#[test]
fn diagbuffer_combines_all_pushed_errors() {
    let mut buf: DiagBuffer<LineLoc, Code> = DiagBuffer::new();
    buf.push(LineLoc { file: "a.rs", line: 1 }, Code::First);
    buf.push(LineLoc { file: "a.rs", line: 2 }, Code::Answer);
    let out: Result<(), String> = buf.finish(|msg, _loc| msg);
    let joined = out.unwrap_err();
    assert!(joined.contains("[TST0001] the first problem"), "{joined}");
    assert!(joined.contains("[TST0042] answer problem"), "{joined}");
    // Combine for String joins with a newline.
    assert!(joined.contains('\n'), "expected newline join: {joined}");
}

#[test]
fn diagbuffer_push_ctx_appends_context() {
    let mut buf: DiagBuffer<LineLoc, Code> = DiagBuffer::new();
    buf.push_ctx(LineLoc { file: "a.rs", line: 3 }, Code::First, "near `foo`");
    let out: Result<(), String> = buf.finish(|msg, _loc| msg);
    let msg = out.unwrap_err();
    assert!(msg.contains("[TST0001] the first problem: near `foo`"), "{msg}");
}

// ---------- Combine for String ----------

#[test]
fn string_combine_newline_joins() {
    let mut a = String::from("one");
    a.combine(String::from("two"));
    assert_eq!(a, "one\ntwo");
}
