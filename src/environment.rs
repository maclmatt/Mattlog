use std::collections::HashMap;
use crate::terms::Term;

#[derive(Clone)]
pub struct Environment {
    bindings: HashMap<String, Term>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
        }
    }
}
