#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Constant(String),
    Variable(String),
    Compound(String, Vec<Term>),
    Integer(i32),
    List(Box<Term>, Box<Term>), // Represents lists (head | tail)
    EmptyList,
}

impl Term {
    pub fn compound(name: &str, args: Vec<Term>) -> Self {
        Term::Compound(name.to_string(), args)
    }

    // You may also add methods to create other kinds of terms
    pub fn constant(value: &str) -> Self {
        Term::Constant(value.to_string())
    }

    pub fn variable(name: &str) -> Self {
        Term::Variable(name.to_string())
    }

    pub fn integer(value: i32) -> Self {
        Term::Integer(value)
    }

    pub fn list(head: Term, tail: Term) -> Self {
        Term::List(Box::new(head), Box::new(tail))
    }
}

#[derive(Debug, Clone)]
pub enum Clause {
    Fact(Term),
    Rule(Term, Term),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_creation() {
        // Testing constants
        let constant = Term::constant("a");
        assert_eq!(constant, Term::Constant("a".to_string()));

        // Testing variables
        let variable = Term::variable("X");
        assert_eq!(variable, Term::Variable("X".to_string()));

        // Testing compound terms
        let compound = Term::compound("nth", vec![
            Term::variable("X"), 
            Term::integer(0), 
            Term::variable("X")
        ]);
        assert_eq!(compound, Term::Compound("nth".to_string(), vec![
            Term::Variable("X".to_string()), 
            Term::Integer(0), 
            Term::Variable("X".to_string())
        ]));

        // Testing a list (Head | Tail)
        let list = Term::list(Term::integer(1), Term::EmptyList);
        assert_eq!(list, Term::List(Box::new(Term::Integer(1)), Box::new(Term::EmptyList)));
    }
}


