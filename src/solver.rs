use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Expression, db: &Database) -> Option<Substitution> {
    let mut subs = Substitution::new();

    let term = if let Expression::Term(term) = query {
        preprocess(term.clone())
    } else {
        return None;  // Unsupported query type (e.g., not a Term expression)
    };
    

    if let Expression::Term(Term::Compound(name, args)) = query {
        if name == "is" && args.len() == 2 {
            let left = &args[0];  // Should be a variable
            let right = &args[1]; // Should be an evaluable expression
            
            println!("Evaluating: {:?} is {:?}", left, right);

            // Ensure left is a variable
            if let Term::Variable(var) = left {
                // Ensure right is evaluable
                if let Some(value) = evaluate_arithmetic(right) {
                    println!("Computed: {} is {}", var, value);
                    let mut result = Substitution::new();
                    result.extend(var.clone(), Term::Integer(value)); // Bind X to 2
                    return Some(result);
                }
            }
            return None; // Fail if invalid `is/2`
        }

        if ["<", ">", "=<", ">=", "=", "\\="].contains(&name.as_str()) && args.len() == 2 {
            if let Some(result) = evaluate_relation(name, &args[0], &args[1]) {
                if result {
                    return Some(subs); // Success, return empty substitution (nothing to bind)
                } else {
                    return None; // Failure (relation is false)
                }
            }
            return None; // Invalid relation expression
        }

    }

    match query {
        Expression::Term(term) => {
            for clause in &db.clauses {
                match clause {
                    Clause::Fact(fact) => {
                        if unify(term, fact, &mut subs) {
                            return Some(subs);
                        }
                    }
                    Clause::Rule(head, body) => {
                        let mut local_subs = subs.clone(); // Clone subs for safety
                        if unify(term, head, &mut local_subs) {
                            let applied_body = body.apply(&local_subs);
                    
                            if let Some(new_subs) = solve(&applied_body, db) {
                                
                                if new_subs.is_empty() { //NEW FIX: Stop here if no new info
                                    return Some(local_subs);
                                }
                                // Ensure no conflicts before merging
                                if local_subs.allow_merge(&new_subs) {
                                    for (var, term) in new_subs.iter() {
                                        local_subs.extend(var.clone(), term.clone()); // Clone since `extend` takes ownership
                                    }
                                    return Some(local_subs); // Return merged substitutions
                                } else {
                                    println!("Conflict detected when merging substitutions");
                                }
                            }
                        }
                    }
                }
            }
        }

        Expression::Conjunct(left, right) => {
            if let Some(mut left_subs) = solve(left, db) { 
                let applied_right = right.apply(&left_subs);
                if let Some(right_subs) = solve(&applied_right, db) {
                    left_subs.allow_merge(&right_subs);
                    return Some(left_subs);
                }
            }
        }
        
    }

    None
}

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
                _ => None, // Unsupported operator
            }
        }
        _ => None, // Not an arithmetic expression
    }
}

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
        _ => None, // Unsupported operator
    }
}

const RELATIONAL_OPERATORS: [&str; 6] = ["<", ">", "=<", ">=", "=", "\\="];
const ARITHMETIC_OPERATORS: [&str; 4] = ["+", "-", "*", "/"];

pub fn preprocess(term: Term) -> Term {
    if let Term::Compound(op, args) = &term {
        if RELATIONAL_OPERATORS.contains(&op.as_str()) && args.len() == 2 {
            let left = preprocess(args[0].clone());
            let right = preprocess(args[1].clone());
            return Term::Compound(op.clone(), vec![left, right]);
        }
    }
    preprocess_arithmetic(term)
}

pub fn preprocess_arithmetic(term: Term) -> Term {
    if let Term::Compound(op, args) = &term {
        if ARITHMETIC_OPERATORS.contains(&op.as_str()) && args.len() == 2 {
            let left = preprocess_arithmetic(args[0].clone());
            let right = preprocess_arithmetic(args[1].clone());
            return Term::Compound(op.clone(), vec![left, right]);
        }
    }
    term
}
