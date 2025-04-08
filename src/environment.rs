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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::Term;

    #[test]
    fn test_new_environment_is_empty() {
        let env = Environment::new();
        assert_eq!(env.bindings.len(), 0, "Environment should be empty upon creation");
    }

    #[test]
    fn test_environment_can_store_binding() {
        let mut env = Environment::new();
        env.bindings.insert("X".to_string(), Term::Integer(42));
        assert_eq!(env.bindings.get("X"), Some(&Term::Integer(42)));
    }

    #[test]
    fn test_environment_overwrites_existing_binding() {
        let mut env = Environment::new();
        env.bindings.insert("X".to_string(), Term::Integer(1));
        env.bindings.insert("X".to_string(), Term::Integer(2));
        assert_eq!(env.bindings.get("X"), Some(&Term::Integer(2)));
    }
}
