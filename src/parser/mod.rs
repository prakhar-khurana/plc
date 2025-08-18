//! Parsers for different Siemens PLC source formats. The `parse_file`
//! function dispatches to the appropriate frontend based on file
//! extension. Both SCL (Structured Text) and PLCOpen XML are supported.

use std::path::Path;

use crate::ast::Program;

pub mod scl;
pub mod plcopen;

/// Parse a PLC source file into a [`Program`]. The file extension
/// determines which frontend to use:
/// - `.scl`, `.st`, `.sclsrc`  -> SCL parser
/// - `.xml` (PLCOpen)          -> PLCOpen parser
pub fn parse_file(path: &Path) -> Result<Program, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "scl" | "st" | "sclsrc" => scl::parse_scl(path),
        "xml" => plcopen::parse_plcopen(path),
        other => Err(format!(
            "Unsupported file extension: '{}'. Expected .scl/.st or .xml",
            other
        )),
    }
}
