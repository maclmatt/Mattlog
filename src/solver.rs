use crate::database::Database;
use crate::terms::{Clause, Term};
use crate::unification::{Substitution, unify};
use std::collections::HashMap;

pub fn solve(query: &Term, db: &Database) -> Option<Substitution> {
    for clause in &db.clauses {
        let mut subs = Substitution::new(); // Use the Substitution struct

        match clause {
            Clause::Fact(fact) => {
                if unify(query, fact, &mut subs) {
                    return Some(subs);
                }
            }
            Clause::Rule(head, body) => {
                if unify(query, head, &mut subs) {
                    let applied_body = subs.apply(body); // Apply substitutions to the body
                    if let Some(new_subs) = solve(&applied_body, db) {
                        subs.allow_merge(&new_subs); // Use the new extend method (allow_merge)
                        return Some(subs);
                    }
                }
            }
        }
    }
    None
}
