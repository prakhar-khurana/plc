//! Rule 9: Validate indirections (array indexing with variable index).
//! Flag any MyArray[IndexVar] that is not guarded by range checks on IndexVar.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        walk(&f.statements, &mut vec![], &mut violations);
    }

    RuleResult::violations(violations)
}

fn walk(stmts: &[Statement], guards: &mut Vec<String>, out: &mut Vec<Violation>) {
    for st in stmts {
        match st {
            Statement::IfStmt { condition, then_branch, else_branch, .. } => {
                guards.push(expr_text(condition));
                walk(then_branch, guards, out);
                walk(else_branch, guards, out);
                guards.pop();
            }
            Statement::Assign { value, line, .. } | Statement::Expr { expr: value, line } => {
                find_index_violations(value, *line, guards, out);
            }
            _ => {}
        }
    }
}

fn find_index_violations(e: &Expression, line: usize, guards: &Vec<String>, out: &mut Vec<Violation>) {
    match e {
        Expression::Index { base, index, .. } => {
            let idx_txt = expr_text(index);
            let idx_up = idx_txt.to_ascii_uppercase();
            let guarded = guards.iter().any(|g| {
                let gup = g.to_ascii_uppercase();
                gup.contains(&idx_up) && (gup.contains("<") || gup.contains(">") || gup.contains("BOUND") || gup.contains("LEN"))
            });
            if !guarded && matches!(**index, Expression::VariableRef(_)) {
                out.push(Violation {
                    rule_no: 9,
                    rule_name: "Validate indirections",
                    line,
                    reason: format!("Array indexed by variable '{}' without bounds check", idx_txt),
                    suggestion: "Validate index against array bounds before access.".into(),
                });
            }
            find_index_violations(base, line, guards, out);
            find_index_violations(index, line, guards, out);
        }
        Expression::BinaryOp { left, right, .. } => {
            find_index_violations(left, line, guards, out);
            find_index_violations(right, line, guards, out);
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
