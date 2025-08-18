//! Rule 7: Validate paired inputs/outputs.

use crate::ast::{Expression, Program, Statement};
use super::{Policy, RuleResult, Violation};

pub fn check(program: &Program, policy: &Policy) -> RuleResult {
    let mut violations = vec![];

    let Some(pairs) = &policy.pairs else {
        return RuleResult::ok(7, "Validate paired inputs/outputs");
    };
    if pairs.is_empty() {
        return RuleResult::ok(7, "Validate paired inputs/outputs");
    }

    for pair in pairs {
        let a_up = pair[0].to_ascii_uppercase();
        let b_up = pair[1].to_ascii_uppercase();

        for func in &program.functions {
            let mut a_lines = vec![];
            let mut b_lines = vec![];

            collect_assigns(&func.statements, &a_up, &mut a_lines);
            collect_assigns(&func.statements, &b_up, &mut b_lines);

            if !a_lines.is_empty() && !b_lines.is_empty() {
                let first = a_lines[0].min(b_lines[0]);
                violations.push(Violation {
                    rule_no: 7,
                    rule_name: "Validate paired inputs/outputs",
                    line: first,
                    reason: format!(
                        "Signals '{}' and '{}' may be active concurrently",
                        &pair[0], &pair[1]
                    ),
                    suggestion: "Guard with mutually exclusive conditions or interlocks.".into(),
                });
            }
        }
    }

    RuleResult::violations(violations)
}

fn collect_assigns(stmts: &[Statement], target_up: &str, lines: &mut Vec<usize>) {
    for st in stmts {
        match st {
            Statement::Assign { target, value, line } => {
                if target.name.to_ascii_uppercase() == target_up {
                    match value {
                        Expression::BoolLiteral(true, _) => lines.push(*line),
                        Expression::NumberLiteral(n, _) if *n != 0 => lines.push(*line),
                        _ => {}
                    }
                }
            }
            Statement::IfStmt { then_branch, else_branch, .. } => {
                collect_assigns(then_branch, target_up, lines);
                collect_assigns(else_branch, target_up, lines);
            }
            _ => {}
        }
    }
}
