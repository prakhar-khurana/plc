//! Rule 5: Use checksum integrity checks.
//! Heuristic: look for a call named "GetChecksum" and then an IF using
//! "checksum" compared to a stored value and setting an alarm when mismatched.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        let mut has_call = false;
        let mut call_line = f.line;

        collect_checksum_calls(&f.statements, &mut has_call, &mut call_line);
        if has_call {
            if !has_checksum_compare_alarm(&f.statements) {
                violations.push(Violation {
                    rule_no: 5,
                    rule_name: "Use checksum integrity checks",
                    line: call_line,
                    reason: "GetChecksum called but its result is not compared and alarmed".into(),
                    suggestion: "Compare checksum to stored value and raise alarm when mismatched.".into(),
                });
            }
        }
    }

    RuleResult::violations(violations)
}

fn collect_checksum_calls(stmts: &[Statement], has: &mut bool, ln: &mut usize) {
    for st in stmts {
        match st {
            Statement::Call { name, line, .. } => {
                if name.to_ascii_uppercase().contains("GETCHECKSUM") {
                    *has = true; *ln = *line;
                }
            }
            Statement::IfStmt { then_branch, else_branch, .. } => {
                collect_checksum_calls(then_branch, has, ln);
                collect_checksum_calls(else_branch, has, ln);
            }
            _ => {}
        }
    }
}

fn has_checksum_compare_alarm(stmts: &[Statement]) -> bool {
    for st in stmts {
        match st {
            Statement::IfStmt { condition, then_branch, else_branch, .. } => {
                let c = expr_text(condition).to_ascii_uppercase();
                if c.contains("CHECKSUM") && (c.contains("<>") || c.contains("!=")) {
                    if then_branch.iter().any(|s| match s {
                        Statement::Assign { target, value, .. }
                            if target.name.to_ascii_uppercase().contains("ALARM") && is_true_expr(value) => true,
                        _ => false,
                    }) {
                        return true;
                    }
                }
                if has_checksum_compare_alarm(then_branch) || has_checksum_compare_alarm(else_branch) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn is_true_expr(e: &Expression) -> bool {
    match e {
        Expression::BoolLiteral(true, _) => true,
        Expression::NumberLiteral(n, _) => *n != 0,
        _ => false,
    }
}

fn expr_text(e: &Expression) -> String {
    match e {
        Expression::VariableRef(s) => s.clone(),
        Expression::NumberLiteral(n, _) => n.to_string(),
        Expression::BoolLiteral(b, _) => b.to_string(),
        Expression::BinaryOp { left, right, op, .. } => {
            let o = match op {
                crate::ast::BinOp::Div => "/",
                crate::ast::BinOp::Add => "+",
                crate::ast::BinOp::Sub => "-",
                crate::ast::BinOp::Mul => "*",
                crate::ast::BinOp::Eq => "=",
                crate::ast::BinOp::Lt => "<",
                crate::ast::BinOp::Le => "<=",
                crate::ast::BinOp::Gt => ">",
                crate::ast::BinOp::Ge => ">=",
            };
            format!("{} {} {}", expr_text(left), o, expr_text(right))
        }
        Expression::Index { base, index, .. } => format!("{}[{}]", expr_text(base), expr_text(index)),
    }
}
