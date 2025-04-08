use crate::parser::tree::{ TermKind, ExprKind, Clause as TreeClause, Term as TreeTerm };
use crate::unification::Substitution;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Constant(String),
    Variable(String),
    Compound(String, Vec<Term>),
    Integer(i64),
    List(Box<Term>, Box<Term>), // Represents lists (head | tail)
    EmptyList,
}

impl Term {

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

    pub fn from_vec(vec: &[Term]) -> Self {
        vec.iter().rev().fold(Term::EmptyList, |acc, x| {
            Term::List(Box::new(x.clone()), Box::new(acc))
        })
    }

    pub fn to_vec(&self) -> Option<Vec<Term>> {
        let mut terms = vec![];
        let mut current = self;

        while let Term::List(head, tail) = current {
            terms.push((**head).clone());
            current = tail;
        }
        if matches!(current, Term::EmptyList) {
            Some(terms)
        } else {
            None
        }
    }

    pub fn list_from_vec(mut elements: Vec<Term>) -> Term {
        let mut list = Term::EmptyList;
        while let Some(last) = elements.pop() {
            list = Term::List(Box::new(last), Box::new(list));
        }
        list
    }
    
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Variable(name) => write!(f, "{}", name),
            Term::Integer(n) => write!(f, "{}", n),
            Term::Constant(name) => write!(f, "{}", name),
            Term::Compound(name, args) => {
                let args_str: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
            Term::List(head, tail) => write!(f, "[{} | {}]", head, tail),
            Term::EmptyList => write!(f, "[]"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unification::Substitution;

    #[test]
    fn test_create_constant() {
        let term = Term::Constant("hello".to_string());
        assert_eq!(term, Term::Constant("hello".to_string()));
    }

    #[test]
    fn test_create_variable() {
        let term = Term::Variable("X".to_string());
        assert_eq!(term, Term::Variable("X".to_string()));
    }

    #[test]
    fn test_create_compound() {
        let term = Term::Compound("parent".to_string(), vec![
            Term::Constant("john".to_string()),
            Term::Constant("mary".to_string()),
        ]);
        assert_eq!(
            term,
            Term::Compound(
                "parent".to_string(),
                vec![
                    Term::Constant("john".to_string()),
                    Term::Constant("mary".to_string())
                ]
            )
        );
    }

    #[test]
    fn test_list_from_vec_and_back() {
        let terms = vec![
            Term::Integer(1),
            Term::Integer(2),
            Term::Integer(3),
        ];
        let list = Term::list_from_vec(terms.clone());
        let back_to_vec = list.to_vec().unwrap();
        assert_eq!(back_to_vec, terms);
    }

    #[test]
    fn test_display_compound() {
        let term = Term::Compound("likes".to_string(), vec![
            Term::Constant("john".to_string()),
            Term::Constant("pizza".to_string()),
        ]);
        assert_eq!(format!("{}", term), "likes(john, pizza)");
    }

    #[test]
    fn test_expression_from_term() {
        let term = Term::Variable("X".to_string());
        let expr = Expression::from_term(term.clone());
        assert_eq!(expr, Expression::Term(term));
    }

    #[test]
    fn test_expression_apply_substitution() {
        let term = Term::Variable("X".to_string());
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(5));
        let expr = Expression::Term(term);
        let applied = expr.apply(&subs);
        assert_eq!(applied, Expression::Term(Term::Integer(5)));
    }

    #[test]
    fn test_conjunct_apply_substitution() {
        let left = Expression::Term(Term::Variable("X".to_string()));
        let right = Expression::Term(Term::Variable("Y".to_string()));
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(1));
        subs.extend("Y".to_string(), Term::Integer(2));
        let conjunct = Expression::Conjunct(Box::new(left), Box::new(right));
        let applied = conjunct.apply(&subs);
        assert_eq!(
            applied,
            Expression::Conjunct(
                Box::new(Expression::Term(Term::Integer(1))),
                Box::new(Expression::Term(Term::Integer(2)))
            )
        );
    }

    #[test]
    fn test_clause_fact_creation() {
        let term = Term::Constant("hello".to_string());
        let clause = Clause::Fact(term.clone());
        assert_eq!(clause, Clause::Fact(term));
    }

    #[test]
    fn test_clause_rule_creation() {
        let head = Term::Compound("parent".to_string(), vec![
            Term::Variable("X".to_string()),
            Term::Variable("Y".to_string()),
        ]);
        let body = Expression::Conjunct(
            Box::new(Expression::Term(Term::Constant("father".to_string()))),
            Box::new(Expression::Term(Term::Constant("mother".to_string()))),
        );
        let clause = Clause::Rule(head.clone(), body.clone());
        assert_eq!(clause, Clause::Rule(head, body));
    }
}

