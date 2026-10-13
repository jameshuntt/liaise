use alloc::{string::{String, ToString}, vec::Vec};
use crate::loc::DiagnosticLoc;
use crate::report::{LiaiseDescriptor, LiaiseEvidence, LiaiseReport, LiaiseSeverity, LiaiseStability, LiaiseToken};

pub trait ErrorRegistry: 'static {
    const PREFIX: &'static str;
    const ALL_CODES: &'static [u16];

    const VALIDATE: () = {
        if !crate::validate_uniqueness(Self::ALL_CODES) {
            panic!("LIAISE ERROR: duplicate diagnostic code detected");
        }
    };
}

pub trait Liaise: ErrorRegistry + core::fmt::Display {
    fn prefix(&self) -> &'static str { Self::PREFIX }
    fn code_id(&self) -> u16;
    fn domain(&self) -> &'static str { "diagnostic" }
    fn slug(&self) -> &'static str;
    fn category(&self) -> Option<&'static str> { None }
    fn severity(&self) -> LiaiseSeverity { LiaiseSeverity::Error }
    fn stability(&self) -> LiaiseStability { LiaiseStability::Experimental }
    fn message(&self) -> String;
    fn help(&self) -> Option<&'static str> { None }
    /// The code's urgent mark, declared once with the code (`#[liaise(urgent = true)]` or `false`): None when unmarked.
    fn urgent_mark(&self) -> Option<bool> { None }
    /// Whether the code is urgent — dumped at once rather than at the next interval: its mark, and unmarked, Error or
    /// worse. The one rule; a code's mark raises or lowers it.
    fn is_urgent(&self) -> bool { self.urgent_mark().unwrap_or(self.severity() >= LiaiseSeverity::Error) }
    fn tokens(&self) -> Vec<LiaiseToken> { Vec::new() }
    fn evidence(&self) -> Vec<LiaiseEvidence> { Vec::new() }

    fn code(&self) -> String { alloc::format!("{}{:04}", self.prefix(), self.code_id()) }
    fn render(&self) -> String { alloc::format!("[{}] {}", self.code(), self.message()) }

    fn descriptor(&self) -> LiaiseDescriptor {
        LiaiseDescriptor {
            code: self.code(),
            prefix: self.prefix().to_string(),
            code_id: self.code_id(),
            domain: self.domain().to_string(),
            slug: self.slug().to_string(),
            category: self.category().map(ToString::to_string),
            severity: self.severity(),
            stability: self.stability(),
            message_template: self.message(),
            help: self.help().map(ToString::to_string),
        }
    }

    fn liaise_report(&self) -> LiaiseReport {
        LiaiseReport::new(
            self.code(),
            self.prefix(),
            self.code_id(),
            self.domain(),
            self.slug(),
            self.category().map(ToString::to_string),
            self.severity(),
            self.stability(),
            self.message(),
            self.help().map(ToString::to_string),
            self.tokens(),
            self.evidence(),
        )
    }
}

pub const fn validate_uniqueness(ids: &[u16]) -> bool {
    let mut i = 0;
    while i < ids.len() {
        let mut j = i + 1;
        while j < ids.len() {
            if ids[i] == ids[j] { return false; }
            j += 1;
        }
        i += 1;
    }
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiaiseRegistryError {
    EmptyRegistryId,
    DuplicateCode { code: String },
}

pub type LiaiseRegistryResult<T> = Result<T, LiaiseRegistryError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiaiseRegistry {
    pub registry_id: String,
    pub descriptors: Vec<LiaiseDescriptor>,
}

impl LiaiseRegistry {
    pub fn new(registry_id: impl Into<String>, descriptors: Vec<LiaiseDescriptor>) -> LiaiseRegistryResult<Self> {
        let registry_id = registry_id.into();
        if registry_id.trim().is_empty() { return Err(LiaiseRegistryError::EmptyRegistryId); }
        for (index, descriptor) in descriptors.iter().enumerate() {
            if descriptors.iter().skip(index + 1).any(|other| other.code == descriptor.code) {
                return Err(LiaiseRegistryError::DuplicateCode { code: descriptor.code.clone() });
            }
        }
        Ok(Self { registry_id, descriptors })
    }

    pub fn descriptor_by_code(&self, code: &str) -> Option<&LiaiseDescriptor> {
        self.descriptors.iter().find(|descriptor| descriptor.code == code)
    }
}

pub struct Diagnostic<L: DiagnosticLoc, R: Liaise> {
    pub loc: L,
    pub code: R,
    pub ctx: Option<String>,
}

pub struct DiagBuffer<L: DiagnosticLoc, R: Liaise> {
    reports: Vec<Diagnostic<L, R>>,
}

impl<L: DiagnosticLoc, R: Liaise> DiagBuffer<L, R> {
    pub fn new() -> Self { Self { reports: Vec::new() } }
    pub fn push(&mut self, loc: L, code: R) { self.reports.push(Diagnostic { loc, code, ctx: None }); }
    pub fn push_ctx(&mut self, loc: L, code: R, ctx: impl core::fmt::Display) {
        self.reports.push(Diagnostic { loc, code, ctx: Some(alloc::format!("{}", ctx)) });
    }
    pub fn finish<E>(self, converter: impl Fn(String, L) -> E) -> Result<(), E>
    where E: Combine {
        let mut final_err: Option<E> = None;
        for d in self.reports {
            let msg = match d.ctx { Some(c) => alloc::format!("{}: {}", d.code.render(), c), None => d.code.render() };
            let e = converter(msg, d.loc);
            match final_err.as_mut() { Some(prev) => prev.combine(e), None => final_err = Some(e) }
        }
        final_err.map(Err).unwrap_or(Ok(()))
    }
}

impl<L: DiagnosticLoc, R: Liaise> Default for DiagBuffer<L, R> {
    fn default() -> Self { Self::new() }
}

pub trait Combine { fn combine(&mut self, other: Self); }

/// A buffer that folds errors of one combinable type into one (the 0.1.x form;
/// `DiagBuffer` is the located, coded form). `SynBuffer` is this over `syn::Error`.
pub struct DiagnosticBuffer<E: Combine> {
    err: Option<E>,
}

impl<E: Combine> DiagnosticBuffer<E> {
    pub fn new() -> Self { Self { err: None } }
    pub fn push(&mut self, new_err: E) {
        match &mut self.err {
            Some(existing) => existing.combine(new_err),
            None => self.err = Some(new_err),
        }
    }
    pub fn finish(self) -> Result<(), E> {
        match self.err { Some(e) => Err(e), None => Ok(()) }
    }
}

impl<E: Combine> Default for DiagnosticBuffer<E> {
    fn default() -> Self { Self::new() }
}

impl Combine for String {
    fn combine(&mut self, other: Self) {
        self.push('\n');
        self.push_str(&other);
    }
}
