use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Expression, db: &Database) -> Option<Substitution> {
    let mut subs = Substitution::new();

    if let Expression::Term(Term::Compound(name, args)) = query {
        if name == "equal" && args.len() == 2 {
            let left = &args[0];
            let right = &args[1];
    
            println!("Checking equality in solver: {:?} == {:?}", left, right);
    
            if left != right {
                println!("Mismatch: {:?} and {:?} are not equal", left, right);
                return None; // Fail immediately
            }
    
            println!("Direct match: {:?} == {:?}", left, right);
            return Some(Substitution::new()); // Success with empty substitution
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
                                // Ensure no conflicts before merging
                                if local_subs.allow_merge(&new_subs) {
                                    return Some(local_subs);
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

