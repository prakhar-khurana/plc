//! Simplified parser for Siemens Structured Text (SCL).
//! Extracts enough structure for rule checks: functions/blocks, assigns,
//! IF/ELSE, calls, and basic expressions (division, literals, indexing).

use std::fs;
use std::path::Path;


use pest_derive::Parser;

use crate::ast::{
    BinOp, Expression, Function, FunctionKind, Program, Statement, Variable,
};

#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
ident = @{ (ASCII_ALPHANUMERIC | "_" | ".")+ }
number = @{ ASCII_DIGIT+ }
operator = _{ "=" | ":=" | "+" | "-" | "*" | "/" | "<" | "<=" | ">" | ">=" }

stmt_end = _{ ";" }
"#]
#[allow(dead_code)]
struct SclMiniParser;

pub fn parse_scl(path: &Path) -> Result<Program, String> {
    let src = fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;
    parse_scl_from_str(&src)
}

pub fn parse(path: &Path) -> Result<Program, String> {
    parse_scl(path)
}

pub fn parse_scl_from_str(src: &str) -> Result<Program, String> {
    let mut program = Program { functions: vec![] };

    let mut current_func: Option<Function> = None;
    let mut if_stack: Vec<Vec<Statement>> = Vec::new();

    let mut line_no = 0usize;
    let lines: Vec<&str> = src.lines().collect();

    while line_no < lines.len() {
        let raw = lines[line_no];
        line_no += 1;

        let line = strip_inline_comment(raw).trim().to_string();
        if line.is_empty() {
            continue;
        }

        // Block starts
        if starts_with_keyword(&line, "FUNCTION_BLOCK")
            || starts_with_keyword(&line, "FUNCTION")
            || starts_with_keyword(&line, "ORGANIZATION_BLOCK")
        {
             if let Some(mut f) = current_func.take() {
                // This also correctly handles any unclosed IF statements from the previous block.
                while let Some(mut body) = if_stack.pop() {
                    f.statements.append(&mut body);
                }
                program.functions.push(f);
            }
            let name = grab_second_token(&line).unwrap_or_else(|| "Unnamed".to_string());
            let kind = if starts_with_keyword(&line, "FUNCTION_BLOCK") {
                FunctionKind::FB
            } else if starts_with_keyword(&line, "FUNCTION") {
                FunctionKind::FC
            } else {
                if name.to_uppercase().contains("OB100") {
                    FunctionKind::OB100
                } else if name.to_uppercase().contains("OB1") {
                    FunctionKind::OB1
                } else if name.to_uppercase().contains("OB86") {
                    FunctionKind::OB86
                } else if name.to_uppercase().contains("OB82") {
                    FunctionKind::OB82
                } else if name.to_uppercase().contains("OB121") {
                    FunctionKind::OB121
                } else {
                    FunctionKind::OB
                }
            };
            current_func = Some(Function {
                name,
                kind,
                statements: vec![],
                line: line_no,
            });
            continue;
        }

        // Block ends
        if starts_with_keyword(&line, "END_FUNCTION_BLOCK")
            || starts_with_keyword(&line, "END_FUNCTION")
            || starts_with_keyword(&line, "END_ORGANIZATION_BLOCK")
        {
            if let Some(mut f) = current_func.take() {
                while let Some(mut body) = if_stack.pop() {
                    f.statements.append(&mut body);
                }
                program.functions.push(f);
            }
            continue;
        }

        // IF ... THEN
        if starts_with_keyword(&line, "IF ") || line.to_uppercase().starts_with("IF(") || line.to_uppercase().starts_with("IF (") {
            let upper = line.to_uppercase();
            let condition_text: String = if let Some(idx) = upper.find("THEN") {
                let if_pos = upper.find("IF").unwrap_or(0);
                line[if_pos + 2..idx].trim().to_string()
            } else {
                let mut cond = String::new();
                if let Some(pos) = upper.find("IF") {
                    cond.push_str(line[pos + 2..].trim());
                } else {
                    cond.push_str(line.trim());
                }
                while line_no < lines.len() {
                    let nxt_raw = lines[line_no];
                    line_no += 1;
                    let nxt = strip_inline_comment(nxt_raw);
                    cond.push(' ');
                    cond.push_str(nxt.trim());
                    if nxt.to_uppercase().contains("THEN") {
                        if let Some(ti) = nxt.to_uppercase().find("THEN") {
                            let head = nxt[..ti].trim();
                            let mut pieces: Vec<&str> = cond.split_whitespace().collect();
                            if !pieces.is_empty() {
                                pieces.pop();
                            }
                            cond = format!("{} {}", pieces.join(" "), head).trim().to_string();
                        }
                        break;
                    }
                }
                cond.trim().to_string()
            };

            if_stack.push(vec![Statement::IfStmt {
                condition: Expression::VariableRef(condition_text.clone()),
                then_branch: vec![],
                else_branch: vec![],
                line: line_no,
            }]);
            continue;
        }

        // ELSE
        if line.to_uppercase().starts_with("ELSE") {
            if let Some(body) = if_stack.last_mut() {
                body.push(Statement::ElseMarker { line: line_no });
            }
            continue;
        }

        // END_IF
        if line.to_uppercase().starts_with("END_IF") {
            if let Some(mut body) = if_stack.pop() {
                if let Some(Statement::IfStmt {
                    condition,
                    line,
                    ..
                }) = body.get(0).cloned()
                {
                    let mut then_branch = vec![];
                    let mut else_branch = vec![];
                    let mut in_else = false;
                    for st in body.into_iter().skip(1) {
                        match st {
                            Statement::ElseMarker { .. } => in_else = true,
                            other => {
                                if in_else {
                                    else_branch.push(other);
                                } else {
                                    then_branch.push(other);
                                }
                            }
                        }
                    }
                    let final_if = Statement::IfStmt {
                        condition,
                        then_branch,
                        else_branch,
                        line,
                    };
                    push_stmt(current_func.as_mut(), final_if);
                }
            }
            continue;
        }

        // Assignment: X := Y;
        if line.contains(":=") && line.trim_end().ends_with(';') {
            let (lhs, rhs) = split_once(&line, ":=").unwrap_or((line.as_str(), ""));
            let lhs = lhs.trim().trim_end_matches(';').to_string();
            let rhs = rhs.trim().trim_end_matches(';').to_string();
            let expr = parse_expr_heuristic(&rhs, line_no);
            push_stmt(
                current_func.as_mut(),
                Statement::Assign {
                    target: Variable { name: lhs },
                    value: expr,
                    line: line_no,
                },
            );
            continue;
        }

        // Function/FB call
        if looks_like_call(&line) {
            let name = line.split('(').next().unwrap_or(line.as_str()).trim().trim_end_matches(';').to_string();
            push_stmt(
                current_func.as_mut(),
                Statement::Call {
                    name,
                    args: vec![],
                    line: line_no,
                },
            );
            continue;
        }

        // Fallback: capture an indexing expression if present
        if let Some(idx_expr) = extract_index_expr(&line, line_no) {
            push_stmt(current_func.as_mut(), Statement::Expr {
                expr: idx_expr,
                line: line_no,
            });
        }
    }

    if let Some(mut f) = current_func.take() {
        while let Some(mut body) = if_stack.pop() {
            f.statements.append(&mut body);
        }
        program.functions.push(f);
    }

    Ok(program)
}

fn push_stmt(current_func: Option<&mut Function>, stmt: Statement) {
    if let Some(f) = current_func {
        f.statements.push(stmt);
    }
}

fn starts_with_keyword(s: &str, kw: &str) -> bool {
    s.trim_start().to_uppercase().starts_with(&kw.to_uppercase())
}

fn grab_second_token(s: &str) -> Option<String> {
    let toks: Vec<&str> = s.split_whitespace().collect();
    toks.get(1).map(|t| t.trim_matches(':').to_string())
}

fn split_once<'a>(s: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    let mut it = s.splitn(2, pat);
    Some((it.next()?, it.next()?))
}

fn strip_inline_comment(line: &str) -> String {
    let mut s = line.to_string();
    if let Some(i) = s.find("//") {
        s.truncate(i);
    }
    if let (Some(a), Some(b)) = (s.find("(*"), s.find("*)")) {
        if a < b {
            let mut t = String::new();
            t.push_str(&s[..a]);
            t.push_str(&s[b + 2..]);
            s = t;
        }
    }
    s
}

fn looks_like_call(line: &str) -> bool {
    let t = line.trim();
    t.contains('(') && t.trim_end().ends_with(");")
}

/// Heuristic expression parser: identify division to support Rule 4,
/// variable refs, numeric literals, and simple binary ops.
fn parse_expr_heuristic(text: &str, line: usize) -> Expression {
    let trimmed_text = text.trim();
    let upper_text = trimmed_text.to_ascii_uppercase();

    // **THE FIX**: Add checks for boolean literals first.
    if upper_text == "TRUE" {
        return Expression::BoolLiteral(true, line);
    }
    if upper_text == "FALSE" {
        return Expression::BoolLiteral(false, line);
    }

    if let Some(pos) = find_top_level_char(trimmed_text, '/') {
        let (l, r) = trimmed_text.split_at(pos);
        let r = &r[1..];
        return Expression::BinaryOp {
            op: BinOp::Div,
            left: Box::new(parse_expr_heuristic(l.trim(), line)),
            right: Box::new(parse_expr_heuristic(r.trim(), line)),
            line,
        };
    }
    if trimmed_text.chars().all(|c| c.is_ascii_digit()) {
        return Expression::NumberLiteral(trimmed_text.parse().unwrap_or(0), line);
    }
    
    Expression::VariableRef(trimmed_text.to_string())
}


fn find_top_level_char(s: &str, ch: char) -> Option<usize> {
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => paren_depth += 1,
            ')' => { if paren_depth > 0 { paren_depth -= 1; } }
            '[' => bracket_depth += 1,
            ']' => { if bracket_depth > 0 { bracket_depth -= 1; } }
            _ => {}
        }
        if c == ch && paren_depth == 0 && bracket_depth == 0 {
            return Some(i);
        }
    }
    None
}

fn extract_index_expr(line: &str, line_no: usize) -> Option<Expression> {
    let txt = line.trim();
    if let Some(lb) = txt.find('[') {
        if let Some(rb) = txt[lb..].find(']') {
            let base = txt[..lb].trim();
            let idx = txt[lb + 1..lb + rb].trim();
            return Some(Expression::Index {
                base: Box::new(Expression::VariableRef(base.to_string())),
                index: Box::new(Expression::VariableRef(idx.to_string())),
                line: line_no,
            });
        }
    }
    None
}
