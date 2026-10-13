//! A liaise code as it travels upstream: its code and severity, the daemon and node that raised it, how often and when
//! — never its message, tokens or evidence; the same code from the same place counted together; its wire form the
//! severity's own labels.

use liaise::{LiaiseCodes, LiaiseEvent, LiaiseSeverity};

#[derive(Debug, Clone, LiaiseCodes)]
#[liaise(prefix = "RLY")]
enum Code {
    #[liaise(code = 3, msg = "the anchor lock is held by {holder}", severity = "warning")]
    LockHeld { holder: &'static str },
    #[liaise(code = 9, msg = "the chain refused the anchor", severity = "fatal")]
    Refused,
}

#[test]
fn an_event_carries_the_code_where_and_how_often_and_never_the_message() {
    let mut held = LiaiseEvent::raised(&Code::LockHeld { holder: "node-a" }, "example:relayer", "node-a", 1_000);
    assert_eq!((held.code.as_str(), held.severity, held.count, held.first_at_ms, held.last_at_ms), ("RLY0003", LiaiseSeverity::Warning, 1, 1_000, 1_000));
    held.again(3_000);
    held.again(2_000);
    assert_eq!((held.count, held.first_at_ms, held.last_at_ms), (3, 1_000, 3_000), "the last time never moves back");
    let elsewhere = LiaiseEvent::raised(&Code::LockHeld { holder: "node-a" }, "example:relayer", "node-b", 1_500);
    let other = LiaiseEvent::raised(&Code::Refused, "example:relayer", "node-a", 1_500);
    assert!(held.same(&LiaiseEvent::raised(&Code::LockHeld { holder: "node-b" }, "example:relayer", "node-a", 9)), "the same code from the same place, whatever its tokens");
    assert!(!held.same(&elsewhere) && !held.same(&other));
    assert!(LiaiseSeverity::Fatal > LiaiseSeverity::Error && LiaiseSeverity::Error > LiaiseSeverity::Warning, "severities order by gravity");
}

#[cfg(feature = "serde")]
#[test]
fn its_wire_form_is_the_code_the_place_and_the_counts() {
    let event = LiaiseEvent::raised(&Code::Refused, "example:relayer", "node-a", 7);
    let json = serde_json::to_string(&event).unwrap();
    assert_eq!(json, r#"{"code":"RLY0009","severity":"fatal","urgent":true,"daemon":"example:relayer","node":"node-a","count":1,"first_at_ms":7,"last_at_ms":7}"#);
    assert!(!json.contains("refused the anchor"), "never the message");
    assert_eq!(serde_json::from_str::<LiaiseEvent>(&json).unwrap(), event);
}
