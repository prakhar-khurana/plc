//! Rule 2: Track operating modes.
//! Heuristic: if we see a call suggesting reading CPU/system mode (e.g. RD_SYS*, SFC*),
//! ensure there exists an IF that checks mode != 8 and sets an ALARM variable.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        // Detect a "mode read" via calls
        let mut mode_call_lines = vec![];
        collect_mode_calls(&f.statements, &mut mode_call_lines);

        if !mode_call_lines.is_empty() {
            // Look for an IF whose condition mentions a mode-like symbol and value 8,
            // and that sets an alarm variable in the THEN.
            let mut ok = false;
            if has_alarm_on_non_run(&f.statements) {
                ok = true;
            }
            if !ok {
                let line = mode_call_lines[0];
                violations.push(Violation {
                    rule_no: 2,
                    rule_name: "Track operating modes",
                    line,
                    reason: "CPU operating mode read without alarm on non-RUN (value != 8)".into(),
                    suggestion: "Add IF (Mode <> 8) THEN Alarm := TRUE; END_IF;".into(),
                });
            }
        }
    }

    RuleResult::violations(violations)
}

fn collect_mode_calls(stmts: &[Statement], out: &mut Vec<usize>) {
    for st in stmts {
        match st {
            Statement::Call { name, line, .. } => {
                let up = name.to_ascii_uppercase();
                if up.contains("RD_SYS") || up.contains("RD_SINFO") || up.contains("SFC") {
                    out.push(*line);
                }
            }
            Statement::IfStmt { then_branch, else_branch, .. } => {
                collect_mode_calls(then_branch, out);
                collect_mode_calls(else_branch, out);
            }
            _ => {}
        }
    }
}

fn has_alarm_on_non_run(stmts: &[Statement]) -> bool {
    for st in stmts {
        if let Statement::IfStmt { condition, then_branch, else_branch, .. } = st {
            let cond_txt = expr_text(condition).to_ascii_uppercase();
            if (cond_txt.contains("MODE") || cond_txt.contains("RUN"))
                && (cond_txt.contains("<> 8") || cond_txt.contains("!= 8") || cond_txt.contains("NOT 8") || cond_txt.contains("8 = FALSE"))
            {
                if branch_sets_alarm_true(then_branch) {
                    return true;
                }
            }
            if has_alarm_on_non_run(then_branch) || has_alarm_on_non_run(else_branch) {
                return true;
            }
        }
    }
    false
}

fn branch_sets_alarm_true(stmts: &[Statement]) -> bool {
    for st in stmts {
        if let Statement::Assign { target, value, line: _ } = st {
            if target.name.to_ascii_uppercase().contains("ALARM") {
                if is_true_expr(value) {
                    return true;
                }
            }
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
