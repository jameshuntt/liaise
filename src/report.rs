use alloc::{string::{String, ToString}, vec::Vec};
use crate::LiaiseDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all = "lowercase"))]
pub enum LiaiseSeverity {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LiaiseSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "trace" => Self::Trace,
            "debug" => Self::Debug,
            "info" => Self::Info,
            "warning" | "warn" => Self::Warning,
            "fatal" => Self::Fatal,
            _ => Self::Error,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiaiseStability {
    Experimental,
    Stable,
    Deprecated,
    Removed,
}

impl LiaiseStability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Experimental => "experimental",
            Self::Stable => "stable",
            Self::Deprecated => "deprecated",
            Self::Removed => "removed",
        }
    }

    pub fn from_label(label: &str) -> Self {
        match label {
            "stable" => Self::Stable,
            "deprecated" => Self::Deprecated,
            "removed" => Self::Removed,
            _ => Self::Experimental,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseToken {
    pub kind: String,
    pub key: String,
    pub value: String,
}

impl LiaiseToken {
    pub fn new(kind: impl Into<String>, key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { kind: kind.into(), key: key.into(), value: value.into() }
    }

    pub fn stable_material(&self) -> String {
        let mut material = String::new();
        push_stable_field(&mut material, "kind", &self.kind);
        push_stable_field(&mut material, "key", &self.key);
        push_stable_field(&mut material, "value", &self.value);
        material
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseEvidence {
    pub key: String,
    pub value: String,
}

impl LiaiseEvidence {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into() }
    }

    pub fn stable_material(&self) -> String {
        let mut material = String::new();
        push_stable_field(&mut material, "key", &self.key);
        push_stable_field(&mut material, "value", &self.value);
        material
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseDescriptor {
    pub code: String,
    pub prefix: String,
    pub code_id: u16,
    pub domain: String,
    pub slug: String,
    pub category: Option<String>,
    pub severity: LiaiseSeverity,
    pub stability: LiaiseStability,
    pub message_template: String,
    pub help: Option<String>,
}

impl LiaiseDescriptor {
    pub fn stable_material(&self) -> String {
        let mut material = String::new();
        material.push_str("liaise.descriptor.v1\n");
        push_stable_field(&mut material, "code", &self.code);
        push_stable_field(&mut material, "prefix", &self.prefix);
        push_stable_field(&mut material, "code_id", &self.code_id.to_string());
        push_stable_field(&mut material, "domain", &self.domain);
        push_stable_field(&mut material, "slug", &self.slug);
        push_stable_field(&mut material, "category", self.category.as_deref().unwrap_or(""));
        push_stable_field(&mut material, "severity", self.severity.as_str());
        push_stable_field(&mut material, "stability", self.stability.as_str());
        push_stable_field(&mut material, "message_template", &self.message_template);
        push_stable_field(&mut material, "help", self.help.as_deref().unwrap_or(""));
        material
    }

    pub fn digest(&self) -> LiaiseDigest { LiaiseDigest::sha256(self.stable_material()) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseReport {
    pub code: String,
    pub prefix: String,
    pub code_id: u16,
    pub domain: String,
    pub slug: String,
    pub category: Option<String>,
    pub severity: LiaiseSeverity,
    pub stability: LiaiseStability,
    pub message: String,
    pub help: Option<String>,
    pub tokens: Vec<LiaiseToken>,
    pub evidence: Vec<LiaiseEvidence>,
    pub stable_material: String,
    pub digest: LiaiseDigest,
}

impl LiaiseReport {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        code: impl Into<String>,
        prefix: impl Into<String>,
        code_id: u16,
        domain: impl Into<String>,
        slug: impl Into<String>,
        category: Option<String>,
        severity: LiaiseSeverity,
        stability: LiaiseStability,
        message: impl Into<String>,
        help: Option<String>,
        tokens: Vec<LiaiseToken>,
        evidence: Vec<LiaiseEvidence>,
    ) -> Self {
        let mut report = Self {
            code: code.into(),
            prefix: prefix.into(),
            code_id,
            domain: domain.into(),
            slug: slug.into(),
            category,
            severity,
            stability,
            message: message.into(),
            help,
            tokens,
            evidence,
            stable_material: String::new(),
            digest: LiaiseDigest::sha256(""),
        };
        let material = report.compute_stable_material();
        let digest = LiaiseDigest::sha256(&material);
        report.stable_material = material;
        report.digest = digest;
        report
    }

    pub fn render(&self) -> String { alloc::format!("[{}] {}", self.code, self.message) }

    fn compute_stable_material(&self) -> String {
        let mut material = String::new();
        material.push_str("liaise.report.v1\n");
        push_stable_field(&mut material, "code", &self.code);
        push_stable_field(&mut material, "prefix", &self.prefix);
        push_stable_field(&mut material, "code_id", &self.code_id.to_string());
        push_stable_field(&mut material, "domain", &self.domain);
        push_stable_field(&mut material, "slug", &self.slug);
        push_stable_field(&mut material, "category", self.category.as_deref().unwrap_or(""));
        push_stable_field(&mut material, "severity", self.severity.as_str());
        push_stable_field(&mut material, "stability", self.stability.as_str());
        push_stable_field(&mut material, "message", &self.message);
        push_stable_field(&mut material, "help", self.help.as_deref().unwrap_or(""));
        for (index, token) in self.tokens.iter().enumerate() {
            push_stable_field(&mut material, &alloc::format!("tokens[{index}]"), &token.stable_material());
        }
        for (index, evidence) in self.evidence.iter().enumerate() {
            push_stable_field(&mut material, &alloc::format!("evidence[{index}]"), &evidence.stable_material());
        }
        material
    }

    pub fn evidence_value(&self, key: &str) -> Option<&str> {
        self.evidence.iter().find(|item| item.key == key).map(|item| item.value.as_str())
    }

    pub fn token_value(&self, key: &str) -> Option<&str> {
        self.tokens.iter().find(|item| item.key == key).map(|item| item.value.as_str())
    }
}

impl core::fmt::Display for LiaiseReport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.render())
    }
}

pub(crate) fn push_stable_field(out: &mut String, key: &str, value: &str) {
    out.push_str(key);
    out.push_str(".len=");
    out.push_str(&value.as_bytes().len().to_string());
    out.push('\n');
    out.push_str(key);
    out.push('=');
    out.push_str(value);
    out.push('\n');
}
