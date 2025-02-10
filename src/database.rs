use crate::terms::{Clause, Term};

#[derive(Debug)]
pub struct Database {
    pub clauses: Vec<Clause>,
}

impl Database {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Database { clauses }
    }
    
    pub fn find_matching_clauses(&self, query: &Term) -> Vec<&Clause> {
        self.clauses.iter().filter(|clause| {
            match clause {
                Clause::Fact(fact) => fact == query, 
                Clause::Rule(head, _) => head == query,
            }
        }).collect()
    }
}
