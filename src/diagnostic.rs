use alloc::vec::Vec;
use alloc::string::String;
use crate::loc::DiagnosticLoc;

/// The "Static Blueprint": Focuses purely on the identity of the codes.
pub trait ErrorRegistry: Copy + 'static {
    const PREFIX: &'static str; // The macro fills this in

    const ALL_CODES: &'static [u16];

    /// The safety gate. If a user implements this trait, 
    /// they are forced into a uniqueness check.
    const VALIDATE: () = {
        if !crate::validate_uniqueness(Self::ALL_CODES) {
            panic!("LIAISE ERROR: Duplicate ID detected in ErrorRegistry!");
        }
    };
}

/// The "Active Reporter": Focuses on how the error is presented.
pub trait Liaise: ErrorRegistry {
    // fn prefix() -> &'static str;
    // Now this can just default to the Registry's prefix!
    fn prefix() -> &'static str {
        Self::PREFIX
    }
    fn code_id(self) -> u16;
    fn message(self) -> &'static str;

    fn render(self) -> alloc::string::String {
        alloc::format!("[{}{:04}] {}", Self::prefix(), self.code_id(), self.message())
    }
}

/// ENTERPRISE FEATURE: Compile-time check for ID collisions.
/// This uses a simple O(N^2) check which is fine for const-eval 
/// as it only runs once during compilation.
pub const fn validate_uniqueness(ids: &[u16]) -> bool {
    let mut i = 0;
    while i < ids.len() {
        let mut j = i + 1;
        while j < ids.len() {
            if ids[i] == ids[j] {
                return false; // Collision detected!
            }
            j += 1;
        }
        i += 1;
    }
    true
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
    pub fn new() -> Self {
        Self { reports: Vec::new() }
    }

    pub fn push(&mut self, loc: L, code: R) {
        self.reports.push(Diagnostic { loc, code, ctx: None });
    }

    pub fn push_ctx(&mut self, loc: L, code: R, ctx: impl core::fmt::Display) {
        self.reports.push(Diagnostic { loc, code, ctx: Some(alloc::format!("{}", ctx)) });
    }

    pub fn finish<E>(self, converter: impl Fn(String, L) -> E) -> Result<(), E> 
    where E: Combine {
        let mut final_err: Option<E> = None;
        for d in self.reports {
            let msg = match d.ctx {
                Some(c) => alloc::format!("{}: {}", d.code.render(), c),
                None => d.code.render(),
            };
            let e = converter(msg, d.loc);
            match final_err.as_mut() {
                Some(prev) => prev.combine(e),
                None => final_err = Some(e),
            }
        }
        final_err.map(Err).unwrap_or(Ok(()))
    }
}

pub trait Combine {
    fn combine(&mut self, other: Self);
}
pub struct DiagnosticBuffer<E: Combine> {
    err: Option<E>,
}

impl<E: Combine> DiagnosticBuffer<E> {
    pub fn new() -> Self {
        Self { err: None }
    }

    pub fn push(&mut self, new_err: E) {
        match &mut self.err {
            Some(existing) => existing.combine(new_err),
            None => self.err = Some(new_err),
        }
    }

    pub fn finish(self) -> Result<(), E> {
        match self.err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}