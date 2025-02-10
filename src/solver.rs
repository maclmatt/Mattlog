use crate::database::Database;
use crate::terms::{Clause, Term};
use crate::unification::unify;
use std::collections::HashMap;

pub fn solve(query: &Term, db: &Database) -> Option<HashMap<String, Term>> {
    for clause in &db.clauses {
        let mut subs = HashMap::new();
        match clause {
            Clause::Fact(fact) => {
                if unify(query, fact, &mut subs) {
                    return Some(subs);
                }
            }
            Clause::Rule(head, body) => {
                if unify(query, head, &mut subs) && solve(body, db).is_some() {
                    return Some(subs);
                }
            }
        }
    }
    None
}
