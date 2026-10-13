//! The urgent rule — Error or worse unless the code's own mark says otherwise, declared once with the code — and what a
//! daemon counts between two dumps: every raise, per code, urgent or not, the urgent ones saying "dump now".

use liaise::{Liaise, LiaiseAccumulator, LiaiseCodes, LiaiseSeverity};

#[derive(Debug, Clone, LiaiseCodes)]
#[liaise(prefix = "RLY")]
enum Code {
    #[liaise(code = 3, msg = "the anchor lock is held by {holder}", severity = "warning")]
    LockHeld { holder: &'static str },
    #[liaise(code = 4, msg = "the chain is slow to answer", severity = "warning", urgent = true)]
    ChainSlow,
    #[liaise(code = 9, msg = "the chain refused the anchor", severity = "fatal")]
    Refused,
    #[liaise(code = 11, msg = "one vote could not be read", urgent = false)]
    VoteUnread,
}

/// An enum that marks no code: the derive declares no mark, the trait's default stands.
#[derive(Debug, Clone, LiaiseCodes)]
#[liaise(prefix = "VOT")]
enum Unmarked {
    #[liaise(code = 1, msg = "a ballot arrived late", severity = "info")]
    Late,
    #[liaise(code = 2, msg = "the tally failed")]
    TallyFailed,
}

#[test]
fn urgent_is_error_or_worse_unless_the_code_is_marked() {
    assert!(!Code::LockHeld { holder: "node-a" }.is_urgent() && Code::Refused.is_urgent(), "unmarked: by its severity");
    assert!(Code::ChainSlow.is_urgent(), "a warning marked urgent is raised");
    assert_eq!(Code::VoteUnread.severity(), LiaiseSeverity::Error);
    assert!(!Code::VoteUnread.is_urgent(), "an error marked not urgent is lowered");
    assert_eq!((Code::Refused.urgent_mark(), Code::ChainSlow.urgent_mark(), Code::VoteUnread.urgent_mark()), (None, Some(true), Some(false)));
    assert_eq!((Unmarked::Late.urgent_mark(), Unmarked::Late.is_urgent(), Unmarked::TallyFailed.is_urgent()), (None, false, true));
}

#[test]
fn every_raise_is_counted_per_code_and_the_urgent_ones_say_dump_now() {
    let mut counted = LiaiseAccumulator::new("example:relayer", "node-a");
    assert!(!counted.raise(&Code::LockHeld { holder: "node-a" }, 1_000));
    assert!(counted.raise(&Code::Refused, 1_500), "urgent: dump now");
    assert!(!counted.raise(&Code::LockHeld { holder: "node-b" }, 2_000), "the same code, whatever its tokens");
    assert!(counted.raise(&Code::Refused, 2_500), "counted, and urgent again");
    let codes: Vec<_> = counted.counted().iter().map(|e| (e.code.as_str(), e.count, e.first_at_ms, e.last_at_ms, e.urgent)).collect();
    assert_eq!(codes, [("RLY0003", 2, 1_000, 2_000, false), ("RLY0009", 2, 1_500, 2_500, true)], "in the order first raised");
    assert!(counted.counted().iter().all(|e| (e.daemon.as_str(), e.node.as_str()) == ("example:relayer", "node-a")));
    let dumped = counted.take();
    assert_eq!(dumped.len(), 2);
    assert!(counted.counted().is_empty() && counted.take().is_empty(), "counting starts again at each dump");
    counted.raise(&Code::LockHeld { holder: "node-a" }, 3_000);
    assert_eq!((counted.counted()[0].count, counted.counted()[0].first_at_ms), (1, 3_000));
}
