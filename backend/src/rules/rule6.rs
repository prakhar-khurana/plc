//! Rule 6: Validate timers and counters.
//! Heuristic: assignments to TIMER/COUNTER presets (names containing PT, PRESET,
//! TIMER, COUNTER) that use an external variable (non-literal) must be
//! preceded by a range-checking IF on that value.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        let mut ctx: Vec<String> = vec![];
        check_stmts(&f.statements, &mut ctx, &mut violations);
    }

    RuleResult::violations(violations)
}

fn check_stmts(stmts: &[Statement], guards: &mut Vec<String>, out: &mut Vec<Violation>) {
    for st in stmts {
        match st {
            Statement::IfStmt { condition, then_branch, else_branch, .. } => {
                let guard_txt = expr_text(condition);
                guards.push(guard_txt);
                check_stmts(then_branch, guards, out);
                check_stmts(else_branch, guards, out);
                guards.pop();
            }
            Statement::Assign { target, value, line } => {
                let t_up = target.name.to_ascii_uppercase();
                if (t_up.contains("TIMER") || t_up.contains("COUNTER") || t_up.contains(".PT") || t_up.contains("PRESET"))
                    && !is_literal(value)
                {
                    let vtxt = expr_text(value);
                    if !guards.iter().any(|g| guard_covers(g, &vtxt)) {
                        out.push(Violation {
                            rule_no: 6,
                            rule_name: "Validate timers and counters",
                            line: *line,
                            reason: format!("Preset '{}' set from external value '{}' without range check", target.name, vtxt),
                            suggestion: "Precede with IF value within expected min/max range.".into(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

fn is_literal(e: &Expression) -> bool {
    matches!(e, Expression::NumberLiteral(_, _) | Expression::BoolLiteral(_, _))
}

fn guard_covers(guard: &str, val: &str) -> bool {
    let g = guard.to_ascii_uppercase();
    let v = val.to_ascii_uppercase();
    g.contains(&v) && (g.contains("<") || g.contains(">") || g.contains("MIN") || g.contains("MAX"))
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
