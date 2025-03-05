use crate::parser::tree::{ TermKind, ExprKind, Clause as TreeClause, variable, atom, compound, conjunct, fact, rule, Term as TreeTerm, Expr as TreeExpr };
use crate::unification::Substitution;
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Constant(String),
    Variable(String),
    Compound(String, Vec<Term>),
    Integer(i64),
    List(Box<Term>, Box<Term>), // Represents lists (head | tail)
    EmptyList,
    Conjunct(Box<Term>, Box<Term>),
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

    pub fn integer(value: i64) -> Self {
        Term::Integer(value)
    }

    pub fn list(head: Term, tail: Term) -> Self {
        Term::List(Box::new(head), Box::new(tail))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Term::List(_, _) | Term::EmptyList)
    }

    pub fn from_tree_term(tree_term: TreeTerm) -> Self {
        match *tree_term {  // Use the getter method
            TermKind::Var(name) => Term::Variable(name.clone()),
            TermKind::Atom(value) => Term::Constant(value.clone()),
            TermKind::Integer(value) => Term::Integer(value),
            TermKind::String(value) => Term::Constant(value.clone()), // Convert strings to constants
            TermKind::Compound(name, args) => Term::Compound(
                name.clone(),
                args.iter().map(|arg| Term::from_tree_term(arg.clone())).collect(),
            ),
            TermKind::List(head, tail) => Term::List(
                Box::new(Term::from_tree_term(head)), 
                Box::new(Term::from_tree_term(tail))
            ),
            TermKind::EmptyList => Term::EmptyList,
        }
    }

    pub fn from_tree_expr(tree_expr: TreeExpr) -> Self {
        match *tree_expr {
            ExprKind::Term(term) => Term::from_tree_term(term), // Base case: single term
            ExprKind::Conjunct(lhs, rhs) => {
                // Convert conjunctive expressions into nested Compound terms
                Term::Compound("and".to_string(), vec![
                    Term::from_tree_expr(lhs),
                    Term::from_tree_expr(rhs)
                ])
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum Clause {
    Fact(Term),
    Rule(Term, Expression),
}

impl Clause {
    pub fn from_tree_clause(tree_clause: TreeClause) -> Self {
        match tree_clause {
            TreeClause::Fact(term) => Clause::Fact(Term::from_tree_term(term)),
            TreeClause::Rule(head, body) => Clause::Rule(
                Term::from_tree_term(head),
                Expression::from_tree_expr(body),
            ),
        }
    }

    fn convert_body(body: Vec<TreeTerm>) -> Term {
        let mut terms = body.into_iter().map(Term::from_tree_term);
        let first = terms.next().expect("Rule body cannot be empty");

        terms.fold(first, |acc, next| {
            Term::Compound("and".to_string(), vec![acc, next]) // Use Compound to represent conjunction
        })
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Term(Term),
    Conjunct(Box<Expression>, Box<Expression>),  // Handles multiple conditions
}

impl Expression {
    pub fn from_tree_expr(expr: Box<ExprKind>) -> Self {
        match *expr {
            ExprKind::Term(term) => Expression::Term(Term::from_tree_term(term)),
            ExprKind::Conjunct(left, right) => Expression::Conjunct(
                Box::new(Expression::from_tree_expr(left)),
                Box::new(Expression::from_tree_expr(right)),
            ),
        }
    }
    pub fn apply(&self, subs: &Substitution) -> Self {
        match self {
            Expression::Term(term) => Expression::Term(subs.apply(term)),
            Expression::Conjunct(left, right) => Expression::Conjunct(
                Box::new(left.apply(subs)),
                Box::new(right.apply(subs)),
            ),
        }
    }
    pub fn from_term(term: Term) -> Self {
        Expression::Term(term)  // Wraps a single term into an expression
    }

    pub fn unwrap_term(&self) -> Option<&Term> {
        if let Expression::Term(term) = self {
            Some(term)
        } else {
            None
        }
    }
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


