//! Rule 4: Use PLC flags as integrity checks around division.
//! Flag any `/` operations that are *not* inside a conditional checking
//! status word flags (e.g., SW.OV=0 AND SW.OS=0) or zero divisor.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        collect_div_violations(&f.statements, /*guarded*/ false, &mut violations);
    }

    RuleResult::violations(violations)
}

fn collect_div_violations(stmts: &[Statement], guarded: bool, out: &mut Vec<Violation>) {
    for st in stmts {
        match st {
            Statement::IfStmt { condition, then_branch, else_branch, .. } => {
                let cond_txt = expr_text(condition).to_ascii_uppercase();
                let has_guard = cond_txt.contains("SW.") && (cond_txt.contains("OV=0") || cond_txt.contains("OS=0") || cond_txt.contains("Z=0") || cond_txt.contains("/=0"));
                collect_div_violations(then_branch, guarded || has_guard, out);
                collect_div_violations(else_branch, guarded || has_guard, out);
            }
            Statement::Assign { value, line, .. } | Statement::Expr { expr: value, line } => {
                find_divs(value, *line, guarded, out);
            }
            _ => {}
        }
    }
}

fn find_divs(expr: &Expression, line: usize, guarded: bool, out: &mut Vec<Violation>) {
    match expr {
        Expression::BinaryOp { op: crate::ast::BinOp::Div, left, right, .. } => {
            if !guarded {
                out.push(Violation {
                    rule_no: 4,
                    rule_name: "Use PLC flags as integrity checks",
                    line,
                    reason: "Division operation without status-word / zero-divisor guard".into(),
                    suggestion: "Wrap division inside IF SW.OV=0 AND SW.OS=0 AND divisor<>0 THEN ...".into(),
                });
            }
            find_divs(left, line, guarded, out);
            find_divs(right, line, guarded, out);
        }
        Expression::BinaryOp { left, right, .. } => {
            find_divs(left, line, guarded, out);
            find_divs(right, line, guarded, out);
        }
        Expression::Index { base, index, .. } => {
            find_divs(base, line, guarded, out);
            find_divs(index, line, guarded, out);
        }
        _ => {}
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
