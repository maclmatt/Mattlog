use std::collections::HashMap;
use crate::term::Term;

pub struct Environment {
    bindings: HashMap<String, Term>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, var: String, term: Term) {
        self.bindings.insert(var, term);
    }

    pub fn lookup(&self, var: &String) -> Option<&Term> {
        self.bindings.get(var)
    }
}
