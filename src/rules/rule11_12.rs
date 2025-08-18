//! Rules 11 & 12: Plausibility checks.
//! Requirement: Any use of an HMI-accessible DB variable must be immediately
//! preceded by a comment block starting with "(* @PlausibilityCheck:".
//!
//! Note: The SCL parser strips comments except we could preserve explicit
//! plausibility comments if they existed as standalone lines. If not present,
//! we conservatively flag usage of HMI/DB variables.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        let mut prev_was_plaus = false;
        for st in &f.statements {
            match st {
                Statement::Comment { text, .. } => {
                    prev_was_plaus = text.trim().starts_with("(* @PlausibilityCheck:");
                }
                Statement::Assign { value, line, .. } | Statement::Expr { expr: value, line } => {
                    let mut vars = vec![];
                    collect_vars(value, &mut vars);
                    for v in vars {
                        let up = v.to_ascii_uppercase();
                        if up.contains("HMI") || up.contains("DB") {
                            if !prev_was_plaus {
                                violations.push(Violation {
                                    rule_no: 11,
                                    rule_name: "Plausibility Checks",
                                    line: *line,
                                    reason: format!("Use of '{}' not preceded by @PlausibilityCheck comment", v),
                                    suggestion: "Add '(* @PlausibilityCheck: ... *)' immediately before usage.".into(),
                                });
                            }
                        }
                    }
                    prev_was_plaus = false;
                }
                _ => {
                    prev_was_plaus = false;
                }
            }
        }
    }

    RuleResult::violations(violations)
}

fn collect_vars(e: &Expression, out: &mut Vec<String>) {
    match e {
        Expression::VariableRef(s) => out.push(s.clone()),
        Expression::BinaryOp { left, right, .. } => {
            collect_vars(left, out);
            collect_vars(right, out);
        }
        Expression::Index { base, index, .. } => {
            collect_vars(base, out);
            collect_vars(index, out);
        }
        _ => {}
    }
}
