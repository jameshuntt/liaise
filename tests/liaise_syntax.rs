//! Every attribute and macro syntax liaise has ever accepted still works.
use liaise::{register_liaise_errors, ErrorRegistry, Liaise, LiaiseCodes, RegisterErrors};

// --- 0.1.x forms ---------------------------------------------------------

#[derive(Debug, Clone, Copy, LiaiseCodes)]
#[error_prefix = "OLD"]
enum OldNameValue {
    #[liaise(code = 1, message = "name-value prefix form")]
    A,
}

#[derive(Debug, Clone, Copy, LiaiseCodes)]
#[error_prefix("LST")]
enum OldList {
    #[liaise(code = 1, msg = "list prefix form")]
    A,
}

#[derive(Debug, Clone, Copy, RegisterErrors)]
#[error_prefix("REG")]
enum Registered {
    A = 1,
    B = 2,
}

const SOLVER_PREFIX: &str = "SLV";
const SOLVER_BUSY: u16 = 40;

register_liaise_errors! {
    prefix: SOLVER_PREFIX,
    name: SolverCode,
    {
        Busy = SOLVER_BUSY => "solver is busy",
        Idle = 41 => "solver is idle",
    }
}

register_liaise_errors! {
    prefix: "VIS",
    vis: pub,
    name: VisCode,
    {
        One = 1 => "one",
    }
}

// --- 0.2.x / 0.3.x forms -------------------------------------------------

#[derive(Debug, Clone, Copy, LiaiseCodes)]
#[liaise(prefix = "NEW", domain = "tests.syntax", version = "1")]
enum New {
    #[liaise(code = 7, error = "error= spelling", severity = "warning", stability = "stable", slug = "custom-slug", category = "io", help = "try again")]
    A,
    #[liaise(code = 8, msg = "urgent one", urgent = true)]
    B,
}

#[test]
fn every_prefix_form_is_honoured() {
    assert_eq!(<OldNameValue as ErrorRegistry>::PREFIX, "OLD");
    assert_eq!(<OldList as ErrorRegistry>::PREFIX, "LST");
    assert_eq!(<Registered as ErrorRegistry>::PREFIX, "REG");
    assert_eq!(<SolverCode as ErrorRegistry>::PREFIX, "SLV");
    assert_eq!(<VisCode as ErrorRegistry>::PREFIX, "VIS");
    assert_eq!(<New as ErrorRegistry>::PREFIX, "NEW");
}

#[test]
fn codes_may_be_literals_or_consts() {
    assert_eq!(SolverCode::Busy.code_id(), 40);
    assert_eq!(SolverCode::Idle.code_id(), 41);
    assert_eq!(<SolverCode as ErrorRegistry>::ALL_CODES, &[40, 41]);
    assert_eq!(OldNameValue::A.message(), "name-value prefix form");
    assert_eq!(New::A.slug(), "custom-slug");
    assert_eq!(New::B.urgent_mark(), Some(true));
}

#[cfg(feature = "syn-error")]
#[test]
fn syn_buffer_folds_errors_like_before() {
    use liaise::adapters::syn_impls::{SynBuffer, SynBufferExt};
    let mut buf = SynBuffer::new();
    buf.push_at(proc_macro2::Span::call_site(), New::A);
    buf.push_at_ctx(proc_macro2::Span::call_site(), New::B, "while parsing");
    let err = buf.finish().expect_err("two errors were pushed");
    let text = err.to_string();
    assert!(text.contains("NEW0007"), "{text}");
}
