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

