//! Rule 16: Summarize PLC cycle times.
//! Verify reading OB1_PREV_CYCLE and moving it to an HMI tag.

use crate::ast::{FunctionKind, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    let ob1 = program.functions.iter().find(|f| f.kind == FunctionKind::OB1);
    if let Some(f) = ob1 {
        let mut saw_prev_cycle = false;
        let mut moved_to_hmi = false;
        let mut ln = f.line;

        for st in &f.statements {
            match st {
                Statement::Assign { target, value, line } => {
                    let val_txt = expr_text(value).to_ascii_uppercase();
                    let tgt_txt = target.name.to_ascii_uppercase();
                    if val_txt.contains("OB1_PREV_CYCLE") {
                        saw_prev_cycle = true;
                        ln = *line;
                    }
                    if tgt_txt.contains("HMI") && val_txt.contains("OB1_PREV_CYCLE") {
                        moved_to_hmi = true;
                    }
                }
                _ => {}
            }
        }

        if saw_prev_cycle && !moved_to_hmi {
            violations.push(Violation {
                rule_no: 16,
                rule_name: "Summarize PLC cycle times",
                line: ln,
                reason: "OB1_PREV_CYCLE read but not reported to HMI".into(),
                suggestion: "Move OB1_PREV_CYCLE to a designated HMI tag.".into(),
            });
        }
    }

    RuleResult::violations(violations)
}

fn expr_text(e: &crate::ast::Expression) -> String {
    match e {
        crate::ast::Expression::VariableRef(s) => s.clone(),
        crate::ast::Expression::NumberLiteral(n, _) => n.to_string(),
        crate::ast::Expression::BoolLiteral(b, _) => b.to_string(),
        crate::ast::Expression::BinaryOp { left, right, op, .. } => {
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
        crate::ast::Expression::Index { base, index, .. } => format!("{}[{}]", expr_text(base), expr_text(index)),
    }
}
