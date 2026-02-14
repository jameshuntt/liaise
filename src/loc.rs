use alloc::string::String;

pub trait DiagnosticLoc: Clone {
    fn source_display(&self) -> String;
}

// A simple default for CLI/Basic apps
#[derive(Debug, Clone)]
pub struct LineLoc {
    pub file: &'static str,
    pub line: u32,
}

impl DiagnosticLoc for LineLoc {
    fn source_display(&self) -> String {
        alloc::format!("{}:{}", self.file, self.line)
    }
}