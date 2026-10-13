use liaise::{Liaise, LiaiseCodes, LiaiseRegistry, LiaiseRegistryError};

#[derive(Debug, Clone, LiaiseCodes)]
#[liaise(prefix = "TST", domain = "tests.diagnostic", version = "0.1.0")]
enum TestDiagnostic {
    #[liaise(
        code = 1,
        slug = "missing-pattern",
        msg = "missing pattern `{pattern_id}`",
        severity = "error",
        category = "registry",
        help = "Add the pattern to the registry before extraction.",
        token(domain = "tests.diagnostic"),
        token(pattern = pattern_id),
        token(validation = "registry.pattern.exists"),
        evidence(pattern_id)
    )]
    MissingPattern { pattern_id: String },
}

#[test]
fn derive_builds_report_tokens_evidence_and_digest() {
    let diagnostic = TestDiagnostic::MissingPattern { pattern_id: "arg.scope.path".to_string() };
    let report = diagnostic.liaise_report();
    assert_eq!(report.code, "TST0001");
    assert_eq!(report.domain, "tests.diagnostic");
    assert_eq!(report.slug, "missing-pattern");
    assert_eq!(report.evidence_value("pattern_id"), Some("arg.scope.path"));
    assert_eq!(report.token_value("pattern"), Some("arg.scope.path"));
    assert_eq!(report.token_value("validation"), Some("registry.pattern.exists"));
    assert_eq!(report.digest.as_str().len(), 64);
    assert!(report.render().contains("[TST0001]"));
}

#[test]
fn registry_rejects_duplicate_codes() {
    let diagnostic = TestDiagnostic::MissingPattern { pattern_id: "arg.scope.path".to_string() };
    let descriptor = diagnostic.descriptor();
    let err = LiaiseRegistry::new("tests.registry", vec![descriptor.clone(), descriptor]).unwrap_err();
    assert_eq!(err, LiaiseRegistryError::DuplicateCode { code: "TST0001".to_string() });
}
