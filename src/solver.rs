use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Expression, db: &Database) -> Option<Substitution> {
    let mut subs = Substitution::new();

    if let Expression::Term(Term::Compound(name, args)) = query {
        if name == "is" && args.len() == 2 {
            let left = &args[0];  // Should be a variable
            let right = &args[1]; // Should be an evaluable expression
            
            println!("Evaluating: {:?} is {:?}", left, right);

            // Ensure left is a variable
            if let Term::Variable(var) = left {
                // Ensure right is evaluable
                if let Some(value) = evaluate_expression(right) {
                    println!("Computed: {} is {}", var, value);
                    let mut result = Substitution::new();
                    result.extend(var.clone(), Term::Integer(value)); // Bind X to 2
                    return Some(result);
                }
            }
            return None; // Fail if invalid `is/2`
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

fn evaluate_expression(expr: &Term) -> Option<i64> {
    match expr {
        Term::Integer(n) => Some(*n),
        Term::Compound(op, args) if args.len() == 2 => {
            let left = evaluate_expression(&args[0])?;
            let right = evaluate_expression(&args[1])?;
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
