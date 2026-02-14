# Liaise 
[![Crates.io](https://img.shields.io/crates/v/liaise.svg)](https://crates.io/crates/liaise)

**Liaise** is a `no_std` diagnostic framework that enforces unique error identities at compile-time. It provides a bridge (a "liaison") between your internal error enums and professional, grep-able diagnostic reporting.



## Why Liaise?
1. **Introspection & Validation:** Automatically extracts variant IDs and ensures no two variants share the same ID.
2. **Zero-Runtime Cost:** All validation happens during compilation via a "Sentinel" check. If your IDs aren't unique, the code won't build.
3. **Enterprise Standards:** Standardizes error reporting into the `[PREFIX0000]` format used by professional toolchains.
4. **Proc-macro Ready:** Includes built-in adapters for `syn` and `proc-macro2` for high-quality compiler error reporting.

---

## Quick Start

### 1. Define and Register
Use `#[derive(RegisterErrors)]` to turn a standard enum into a validated Registry.

```rust
use liaise::{Liaise, RegisterErrors};

#[derive(RegisterErrors)]
#[error_prefix = "FILE"] // Sets the reporting prefix
pub enum FileError {
    NotFound = 404,
    PermissionDenied = 403,
    // Duplicate = 404, // THIS WOULD TRIGGER A COMPILE ERROR
}

impl Liaise for FileError {
    fn code_id(self) -> u16 { self as u16 }
    
    fn message(self) -> &'static str {
        match self {
            Self::NotFound => "The requested file was not found.",
            Self::PermissionDenied => "Access to this file is restricted.",
        }
    }
}
```

### 2. Batch Reporting (with `syn`)
If you're writing a procedural macro, use the `DiagBuffer` to collect errors and report them all at once.

```rust
use liaise::adapters::syn_impls::{SynBuffer, SynBufferExt};

fn validate_tokens(input: MyAst) -> syn::Result<()> {
    let mut buffer = SynBuffer::new();

    if input.is_invalid() {
        // Automatically renders as: [FILE0404] The requested file was not found.
        buffer.push_at(input.span(), FileError::NotFound);
    }

    buffer.finish() 
}
```

---

## The Architecture
Liaise splits diagnostic concerns into three layers to maintain `no_std` purity and modularity:

| Layer | Responsibility | Mechanism |
| :--- | :--- | :--- |
| **`ErrorRegistry`** | **Identity/DNA** | Stores static metadata (IDs, Prefix) and validates uniqueness. |
| **`Liaise`** | **Behavior/Voice** | Defines rendering logic and human-readable messages. |
| **`Adapters`** | **Bridge** | Connects the system to external crates like `syn`. |



---

## Features
* `syn-error`: Enables the `SynBuffer` and helper extensions for `syn`.

---

### Pro-Tip: Stability is Key
Liaise is designed for systems where error codes are **permanent**. By assigning an explicit `u16` to every variant, you ensure that even if you rename the variant in your source code, the error ID remains stable for your users, documentation, and support teams.