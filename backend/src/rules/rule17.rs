//! Rule 17: Log PLC uptime.
//! Check for SFC6 (RD_SINFO) and ensure the runtime counter is reported.

use crate::ast::{Expression, Program, Statement};
use super::{RuleResult, Violation};

pub fn check(program: &Program) -> RuleResult {
    let mut violations = vec![];

    for f in &program.functions {
        let mut sfc6_line = None;
        for st in &f.statements {
            if let Statement::Call { name, line, .. } = st {
                if name.to_ascii_uppercase().contains("SFC6") || name.to_ascii_uppercase().contains("RD_SINFO") {
                    sfc6_line = Some(*line);
                }
            }
        }
        if let Some(ln) = sfc6_line {
            // Look for some assignment to a reporting tag (HMI/UPTIME)
            let mut reported = false;
            for st in &f.statements {
                if let Statement::Assign { target, value, .. } = st {
                    let up = target.name.to_ascii_uppercase();
                    let vtxt = expr_text(value).to_ascii_uppercase();
                    if (up.contains("HMI") || up.contains("UPTIME")) && (vtxt.contains("SFC6") || vtxt.contains("RD_SINFO") || vtxt.contains("RUNTIME")) {
                        reported = true;
                        break;
                    }
                }
            }
            if !reported {
                violations.push(Violation {
                    rule_no: 17,
                    rule_name: "Log PLC uptime",
                    line: ln,
                    reason: "SFC6/RD_SINFO used but uptime not reported".into(),
                    suggestion: "Move uptime counter to an HMI tag or logging DB.".into(),
                });
            }
        }
    }

    RuleResult::violations(violations)
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
