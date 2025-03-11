use crate::terms::Clause;

#[derive(Debug)]
pub struct Database {
    pub clauses: Vec<Clause>,
}

impl Database {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Database { clauses }
    }
}
