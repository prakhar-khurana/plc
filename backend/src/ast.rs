//! Unified AST for Siemens PLC sources (SCL and PLCOpen XML).
//! Keep this lean and language-agnostic so both frontends can target it
//! and the rules engine can reason over it.

use std::fmt;

/// A complete PLC program is a collection of functions (FCs), function
/// blocks (FBs) and organisational blocks (OBs).
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

/// A top-level routine (FC, FB or OB).
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub kind: FunctionKind,
    pub statements: Vec<Statement>,
    /// Best-effort source line where this routine was first seen.
    pub line: usize,
}

/// Kind of routine. We include both generic `OB` and specific OB variants
/// that certain rules care about (OB1, OB100, OB82, OB86, OB121).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionKind {
    FC,
    FB,
    OB,
    OB1,
    OB100,
    OB82,
    OB86,
    OB121,
}

/// Variable (symbolic) reference used in assignments.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
}

/// Statements form the imperative body of a routine.
#[derive(Debug, Clone)]
pub enum Statement {
    /// `X := <expr>;`
    Assign {
        target: Variable,
        value: Expression,
        line: usize,
    },
    /// `SomeBlockOrFunction(...);`
    Call {
        name: String,
        args: Vec<(String, Expression)>, // not heavily used by the checks
        line: usize,
    },
    /// `IF <cond> THEN ... [ELSE ...] END_IF`
    IfStmt {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
        line: usize,
    },
    /// Standalone expression (used to keep track of things like indexing).
    Expr {
        expr: Expression,
        line: usize,
    },
    /// Comment preserved when helpful for rules (e.g., plausibility annotations).
    Comment {
        text: String,
        line: usize,
    },
    /// Internal marker used while rebuilding IFs from a line-oriented scan.
    /// Safe to keep; rules ignore it.
    ElseMarker {
        line: usize,
    },
}

/// Arithmetic / logical binary operators we care about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Div,
    Add,
    Sub,
    Mul,
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Expressions are deliberately minimal; we only model what is useful for rules.
#[derive(Debug, Clone)]
pub enum Expression {
    /// `Some.Var.Name`
    VariableRef(String),
    /// numeric literal with best-effort source line
    NumberLiteral(i64, usize),
    /// boolean literal with best-effort source line
    BoolLiteral(bool, usize),
    /// `<left> <op> <right>`
    BinaryOp {
        op: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
        line: usize,
    },
    /// `Base[Index]`
    Index {
        base: Box<Expression>,
        index: Box<Expression>,
        line: usize,
    },
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
