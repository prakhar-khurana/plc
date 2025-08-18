//! Library crate for plc_secure_checker.
//!
//! This file simply re‑exports the modules used by the binary. The
//! library layout mirrors the organisation of the checker: the AST,
//! parsers and rule implementations all live under this crate root.

pub mod ast;
pub mod parser;
pub mod rules;
