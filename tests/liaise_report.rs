//! Report/value types: severity, stability, tokens, stable digest, location.

use liaise::{
    DiagnosticLoc, LineLoc, LiaiseDigest, LiaiseSeverity, LiaiseStability, LiaiseToken,
};

// ---------- LiaiseSeverity ----------

#[test]
fn severity_as_str_all_variants() {
    assert_eq!(LiaiseSeverity::Trace.as_str(), "trace");
    assert_eq!(LiaiseSeverity::Debug.as_str(), "debug");
    assert_eq!(LiaiseSeverity::Info.as_str(), "info");
    assert_eq!(LiaiseSeverity::Warning.as_str(), "warning");
    assert_eq!(LiaiseSeverity::Error.as_str(), "error");
    assert_eq!(LiaiseSeverity::Fatal.as_str(), "fatal");
}

#[test]
fn severity_from_label_known_and_aliases() {
    assert_eq!(LiaiseSeverity::from_label("trace"), LiaiseSeverity::Trace);
    assert_eq!(LiaiseSeverity::from_label("warning"), LiaiseSeverity::Warning);
    assert_eq!(LiaiseSeverity::from_label("warn"), LiaiseSeverity::Warning);
    assert_eq!(LiaiseSeverity::from_label("fatal"), LiaiseSeverity::Fatal);
}

#[test]
fn severity_from_label_unknown_defaults_to_error() {
    assert_eq!(LiaiseSeverity::from_label("nonsense"), LiaiseSeverity::Error);
    assert_eq!(LiaiseSeverity::from_label(""), LiaiseSeverity::Error);
}

#[test]
fn severity_str_roundtrip_for_canonical_labels() {
    for s in [
        LiaiseSeverity::Trace,
        LiaiseSeverity::Debug,
        LiaiseSeverity::Info,
        LiaiseSeverity::Warning,
        LiaiseSeverity::Error,
        LiaiseSeverity::Fatal,
    ] {
        assert_eq!(LiaiseSeverity::from_label(s.as_str()), s);
    }
}

// ---------- LiaiseStability ----------

#[test]
fn stability_as_str_all_variants() {
    assert_eq!(LiaiseStability::Experimental.as_str(), "experimental");
    assert_eq!(LiaiseStability::Stable.as_str(), "stable");
    assert_eq!(LiaiseStability::Deprecated.as_str(), "deprecated");
    assert_eq!(LiaiseStability::Removed.as_str(), "removed");
}

#[test]
fn stability_from_label_unknown_defaults_experimental() {
    assert_eq!(LiaiseStability::from_label("weird"), LiaiseStability::Experimental);
}

#[test]
fn stability_str_roundtrip() {
    for s in [
        LiaiseStability::Experimental,
        LiaiseStability::Stable,
        LiaiseStability::Deprecated,
        LiaiseStability::Removed,
    ] {
        assert_eq!(LiaiseStability::from_label(s.as_str()), s);
    }
}

// ---------- LiaiseToken ----------

#[test]
fn token_new_and_fields() {
    let t = LiaiseToken::new("field", "name", "widget");
    assert_eq!(t.kind, "field");
    assert_eq!(t.key, "name");
    assert_eq!(t.value, "widget");
}

#[test]
fn token_stable_material_is_deterministic_and_uses_all_fields() {
    let t = LiaiseToken::new("field", "name", "widget");
    let m1 = t.stable_material();
    let m2 = t.stable_material();
    assert_eq!(m1, m2, "stable material must be deterministic");
    assert!(m1.contains("field"));
    assert!(m1.contains("name"));
    assert!(m1.contains("widget"));
}

#[test]
fn token_stable_material_differs_when_a_field_changes() {
    let a = LiaiseToken::new("field", "name", "a").stable_material();
    let b = LiaiseToken::new("field", "name", "b").stable_material();
    assert_ne!(a, b);
}

// ---------- LiaiseDigest ----------

#[test]
fn digest_matches_known_sha256_of_empty() {
    // SHA-256("") = e3b0c442...
    let d = LiaiseDigest::sha256("");
    assert_eq!(
        d.as_str(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn digest_matches_known_sha256_of_abc() {
    // SHA-256("abc") = ba7816bf...
    let d = LiaiseDigest::sha256("abc");
    assert_eq!(
        d.as_str(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn digest_is_deterministic_and_64_hex_chars() {
    let a = LiaiseDigest::sha256("hello world");
    let b = LiaiseDigest::sha256("hello world");
    assert_eq!(a, b);
    assert_eq!(a.as_str().len(), 64);
    assert!(a.as_str().chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn digest_display_equals_as_str() {
    let d = LiaiseDigest::sha256("x");
    assert_eq!(format!("{d}"), d.as_str());
}

// ---------- LineLoc ----------

#[test]
fn line_loc_source_display() {
    let loc = LineLoc { file: "src/foo.rs", line: 12 };
    assert_eq!(loc.source_display(), "src/foo.rs:12");
}
