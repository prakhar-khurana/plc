//! Rule 19: Monitor PLC memory usage.
//! Heuristic: look for SFC24 (TEST_DB) or similar and ensure values are reported.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        let mut mem_call_line = None;
        for st in &f.statements {
            if let Statement::Call { name, line, .. } = st {
                let up = name.to_ascii_uppercase();
                if up.contains("SFC24") || up.contains("TEST_DB") {
                    mem_call_line = Some(*line);
                }
            }
        }
        if let Some(ln) = mem_call_line {
            let mut reported = false;
            for st in &f.statements {
                if let Statement::Assign { target, value, .. } = st {
                    let tgt = target.name.to_ascii_uppercase();
                    let vtxt = expr_text(value).to_ascii_uppercase();
                    if (tgt.contains("HMI") || tgt.contains("MEM") || tgt.contains("DB"))
                        && (vtxt.contains("SFC24") || vtxt.contains("TEST_DB"))
                    {
                        reported = true;
                        break;
                    }
                }
            }
            if !reported {
                violations.push(Violation {
                    rule_no: 19,
                    rule_name: "Monitor PLC memory usage",
                    line: ln,
                    reason: "SFC24/TEST_DB used but memory usage not reported".into(),
                    suggestion: "Assign memory usage data to an HMI/DB tag for monitoring.".into(),
                });
            }
        }
    }

    RuleResult::violations(violations)
}

fn expr_text(e: &Expression) -> String{
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
