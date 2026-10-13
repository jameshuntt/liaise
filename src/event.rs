//! A liaise code as it travels upstream (the fleet's control room, James 2026-10-02: "errors and logs go upstream as
//! liaise codes — prefixes, minimal identifiers"): WHAT was raised (its code and severity), WHERE (the daemon and the
//! node that raised it) and HOW OFTEN between two dumps (its count, the first and the last time). Never the message's
//! text, its tokens or its evidence — those stay with the daemon that raised it, in its own journal.
//!
//! [`LiaiseAccumulator`] counts them between two dumps: what one daemon on one node raised, per code.

use alloc::{string::String, vec::Vec};

use crate::{Liaise, LiaiseSeverity};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiaiseEvent {
    /// The code: its prefix and four-digit id (`RLY0003`).
    pub code: String,
    pub severity: LiaiseSeverity,
    /// Whether the code is urgent (`Liaise::is_urgent`): its daemon dumped it at once.
    pub urgent: bool,
    /// The daemon that raised it (its service id) and the node it runs on.
    pub daemon: String,
    pub node: String,
    /// How many times it was raised from `first_at_ms` to `last_at_ms` (milliseconds since the epoch).
    pub count: u64,
    pub first_at_ms: u64,
    pub last_at_ms: u64,
}

impl LiaiseEvent {
    /// `diagnostic` raised by `daemon` on `node` at `at_ms`, once.
    pub fn raised(diagnostic: &impl Liaise, daemon: &str, node: &str, at_ms: u64) -> Self {
        Self { code: diagnostic.code(), severity: diagnostic.severity(), urgent: diagnostic.is_urgent(), daemon: daemon.into(), node: node.into(), count: 1, first_at_ms: at_ms, last_at_ms: at_ms }
    }

    /// Raised again at `at_ms`: one more, and the last time moved on (never back).
    pub fn again(&mut self, at_ms: u64) {
        self.count = self.count.saturating_add(1);
        self.first_at_ms = self.first_at_ms.min(at_ms);
        self.last_at_ms = self.last_at_ms.max(at_ms);
    }

    /// Whether `other` is the same code from the same daemon on the same node: what is counted together.
    pub fn same(&self, other: &Self) -> bool {
        (&self.code, &self.daemon, &self.node) == (&other.code, &other.daemon, &other.node)
    }
}

/// What one daemon on one node raised since its last dump, counted per code ([`LiaiseEvent::same`]): what its box's
/// sidecar seals between two dump markers. Every raise is counted, urgent or not; an urgent one also says so, and its
/// daemon dumps at once rather than at the next interval — gathered by a pull, "sent at once" is "sealed at once", so the
/// next gather carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseAccumulator {
    daemon: String,
    node: String,
    counted: Vec<LiaiseEvent>,
}

impl LiaiseAccumulator {
    /// Nothing counted yet, for `daemon` (its service id) on `node`.
    pub fn new(daemon: &str, node: &str) -> Self {
        Self { daemon: daemon.into(), node: node.into(), counted: Vec::new() }
    }

    /// `diagnostic` raised at `at_ms`: counted with its code since the last take. True when it is urgent: dump now.
    pub fn raise(&mut self, diagnostic: &impl Liaise, at_ms: u64) -> bool {
        let event = LiaiseEvent::raised(diagnostic, &self.daemon, &self.node, at_ms);
        let urgent = event.urgent;
        match self.counted.iter_mut().find(|counted| counted.same(&event)) {
            Some(counted) => counted.again(at_ms),
            None => self.counted.push(event),
        }
        urgent
    }

    /// What is counted since the last take, in the order first raised.
    pub fn counted(&self) -> &[LiaiseEvent] {
        &self.counted
    }

    /// The dump: what was counted since the last take, in the order first raised; counting starts again.
    pub fn take(&mut self) -> Vec<LiaiseEvent> {
        core::mem::take(&mut self.counted)
    }
}
