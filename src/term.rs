#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Constant(String),
    Variable(String),
    Compound { 
        name: String, 
        args: Vec<Term> 
    },
}

// Helper methods for creating terms
impl Term {
    pub fn constant(value: &str) -> Self {
        Term::Constant(value.to_string())
    }

    pub fn variable(name: &str) -> Self {
        Term::Variable(name.to_string())
    }

    pub fn compound(name: &str, args: Vec<Term>) -> Self {
        Term::Compound {
            name: name.to_string(),
            args,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_creation() {
        let constant = Term::constant("a");
        assert_eq!(constant, Term::Constant("a".to_string()));

        let variable = Term::variable("X");
        assert_eq!(variable, Term::Variable("X".to_string()));
    }
}
