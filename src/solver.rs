use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Expression, db: &Database) -> Option<Substitution> {
    match query {
        Expression::Term(term) => solve_term(term, db),
        Expression::Conjunct(lhs, rhs) => {
            if let Some(lhs_subs) = solve(lhs, db) {
                let applied_rhs = rhs.apply(&lhs_subs);
                if let Some(rhs_subs) = solve(&applied_rhs, db) {
                    if let Some(merged_subs) = lhs_subs.merge(&rhs_subs) {
                        return Some(merged_subs);
                    }
                }
            }
            None
        }
    }
}

fn solve_term(term: &Term, db: &Database) -> Option<Substitution> {
    let mut subs = Substitution::new();

    if let Term::Compound(name, args) = term {
        // 1. Handle arithmetic "is"
        if name == "is" && args.len() == 2 {
            let left = &args[0];  // Should be a variable
            let right = &args[1]; // Should be an evaluable expression

            println!("Evaluating: {:?} is {:?}", left, right);

            if let Term::Variable(var) = left {
                if let Some(value) = evaluate_arithmetic(right) {
                    println!("Computed: {} is {}", var, value);
                    let mut result = Substitution::new();
                    result.extend(var.clone(), Term::Integer(value));
                    return Some(result);
                }
            }
            return None;
        }

        // 2. Handle relational operators (<, >, =<, etc.)
        if RELATIONAL_OPERATORS.contains(&name.as_str()) && args.len() == 2 {
            if let Some(result) = evaluate_relation(name, &args[0], &args[1]) {
                if result {
                    return Some(subs); // Success with no new substitution
                } else {
                    return None; // Relation check failed
                }
            }
            return None; // Invalid relation expression
        }

        // 3. Try to match against facts/rules in the database
        for clause in &db.clauses {
            match clause {
                Clause::Fact(fact) => {
                    // Unify directly with fact
                    if unify(term, fact, &mut subs) {
                        return Some(subs);
                    }
                }
                Clause::Rule(head, body) => {
                    let mut local_subs = subs.clone();

                    // First unify the query with the rule head
                    if unify(term, head, &mut local_subs) {
                        println!("Matched rule head, applying body: {:?}", body);

                        let applied_body = body.apply(&local_subs);

                        // Recursively solve the body (may contain multiple terms if conjunct)
                        if let Some(body_subs) = solve(&applied_body, db) {
                            if body_subs.is_empty() {
                                // No new subs needed, return local substitutions
                                return Some(local_subs);
                            }

                            // Merge local_subs and body_subs (more correct than manual extend)
                            if local_subs.allow_merge(&body_subs) {
                                let merged = local_subs.merged_with(&body_subs);
                                return Some(merged);
                            } else {
                                println!("Conflict detected when merging substitutions");
                            }
                        }
                    }
                }
            }
        }
    }

    None
}


// Arithmetic evaluation (for 'is')
fn evaluate_arithmetic(expr: &Term) -> Option<i64> {
    match expr {
        Term::Integer(n) => Some(*n),
        Term::Compound(op, args) if args.len() == 2 => {
            let left = evaluate_arithmetic(&args[0])?;
            let right = evaluate_arithmetic(&args[1])?;
            match op.as_str() {
                "+" => Some(left + right),
                "-" => Some(left - right),
                "*" => Some(left * right),
                "/" => if right != 0 { Some(left / right) } else { None },
                _ => None,
            }
        }
        _ => None,
    }
}

// Relation evaluation (for <, >, =<, etc.)
fn evaluate_relation(op: &str, left: &Term, right: &Term) -> Option<bool> {
    let left_value = evaluate_arithmetic(left)?;
    let right_value = evaluate_arithmetic(right)?;
    println!("Left and Right {} {}", left_value, right_value);

    match op {
        "<" => Some(left_value < right_value),
        ">" => Some(left_value > right_value),
        "=<" => Some(left_value <= right_value),
        ">=" => Some(left_value >= right_value),
        "=" => Some(left_value == right_value),
        "\\=" => Some(left_value != right_value),
        _ => None,
    }
}

// Operators we want to handle
const RELATIONAL_OPERATORS: [&str; 6] = ["<", ">", "=<", ">=", "=", "\\="];
