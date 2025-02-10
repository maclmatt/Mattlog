use crate::database::Database;
use crate::terms::{Clause, Term, Expression};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Expression, db: &Database) -> Option<Substitution> {
    let mut subs = Substitution::new();

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
                        if unify(term, head, &mut subs) {
                            let applied_body = body.apply(&subs);
                            if let Some(new_subs) = solve(&applied_body, db) {
                                subs.allow_merge(&new_subs);
                                return Some(subs);
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

