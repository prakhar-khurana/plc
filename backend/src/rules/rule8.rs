//! Rule 8: Validate HMI input variables.
//! Heuristic: any usage of variables with HMI/DB prefix should be
//! range-checked in a preceding IF before being used in logic.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        check_stmts(&f.statements, &mut vec![], &mut violations);
    }

    RuleResult::violations(violations)
}

fn check_stmts(stmts: &[Statement], guards: &mut Vec<String>, out: &mut Vec<Violation>) {
    for st in stmts {
        match st {
            Statement::IfStmt { condition, then_branch, else_branch, .. } => {
                guards.push(expr_text(condition));
                check_stmts(then_branch, guards, out);
                check_stmts(else_branch, guards, out);
                guards.pop();
            }
            Statement::Assign { value, line, .. } | Statement::Expr { expr: value, line } => {
                // detect var refs likely coming from HMI-accessible DBs
                let mut vars = vec![];
                collect_vars(value, &mut vars);
                for v in vars {
                    let up = v.to_ascii_uppercase();
                    if up.contains("HMI") || up.contains("DB") {
                        if !guards.iter().any(|g| guard_covers(g, &up)) {
                            out.push(Violation {
                                rule_no: 8,
                                rule_name: "Validate HMI input variables",
                                line: *line,
                                reason: format!("HMI/DB variable '{}' used without prior sanitization", v),
                                suggestion: "Precede usage with range-checking IF or plausibility check.".into(),
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }
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

fn guard_covers(guard: &str, var_up: &str) -> bool {
    let g = guard.to_ascii_uppercase();
    g.contains(var_up) && (g.contains("<") || g.contains(">") || g.contains("MIN") || g.contains("MAX") || g.contains("RANGE"))
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
