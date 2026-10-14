//! Derived code binds only the fields it uses: no `unused variable` warnings.
#![deny(unused_variables)]

use liaise::{Liaise, LiaiseCodes};

#[allow(dead_code)] // the fields are read only through the derive
#[derive(Debug, Clone, LiaiseCodes)]
#[liaise(prefix = "NWR", domain = "tests.no_warnings")]
enum Quiet {
    #[liaise(code = 1, msg = "a named variant whose message uses none of its fields")]
    Unused { path: String, line: u32 },
    #[liaise(code = 2, msg = "only `{path}` is in the message", token(line = line), evidence(path))]
    Partial { path: String, line: u32, column: u32 },
    #[liaise(code = 3, msg = "escaped {{column}} is not a capture; {line:>4} is")]
    Escaped { line: u32, column: u32 },
}

#[test]
fn named_variants_that_ignore_their_fields_compile_without_warnings() {
    let unused = Quiet::Unused { path: "a.rs".to_string(), line: 3 };
    assert_eq!(unused.message(), "a named variant whose message uses none of its fields");
    assert!(unused.tokens().is_empty());
    assert!(unused.evidence().is_empty());

    let partial = Quiet::Partial { path: "b.rs".to_string(), line: 7, column: 9 };
    assert_eq!(partial.message(), "only `b.rs` is in the message");
    let report = partial.liaise_report();
    assert_eq!(report.token_value("line"), Some("7"));
    assert_eq!(report.evidence_value("path"), Some("b.rs"));

    let escaped = Quiet::Escaped { line: 12, column: 1 };
    assert_eq!(escaped.message(), "escaped {column} is not a capture;   12 is");
}
