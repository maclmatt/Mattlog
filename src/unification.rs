use std::collections::HashMap;
use crate::terms::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Substitution(HashMap<String, Term>);

impl Substitution {
    pub fn new() -> Self {
        Substitution(HashMap::new())
    }

    pub fn resolve(&self, term: &Term) -> Term {
        match term {
            Term::Variable(var) => {
                if let Some(val) = self.get(var) {
                    self.resolve(val)
                } else {
                    term.clone()
                }
            }
            Term::Compound(name, args) => {
                Term::Compound(name.clone(), args.iter().map(|t| self.resolve(t)).collect())
            }
            _ => term.clone(),
        }
    }

    pub fn apply(&self, term: &Term) -> Term {
        match term {
            Term::Variable(name) => {
                if let Some(substituted_term) = self.0.get(name) {
                    self.apply(substituted_term) // Recursively apply substitution
                } else {
                    term.clone()
                }
            }
            Term::Constant(_) | Term::Integer(_) | Term::EmptyList => term.clone(),
    
            Term::Compound(name, args) => {
                Term::Compound(name.clone(), args.iter().map(|t| self.apply(t)).collect())
            }
            
            Term::List(head, tail) => {
                let new_head = Box::new(self.apply(head));
                let new_tail = Box::new(self.apply(tail));
                Term::List(new_head, new_tail)
            }
        }
    }  

    pub fn extend(&mut self, var: String, term: Term) {
        self.0.insert(var, term);
    }

    pub fn merge(&self, other: &Substitution) -> Option<Substitution> {
        if other.0.is_empty() {
            return Some(self.clone()); // If `other` is empty, return `self`
        }
        if self.0.is_empty() {
            return Some(other.clone()); // If `self` is empty, return `other`
        }
        let mut merged = self.clone();
        for (key, value) in &other.0 {
            if let Some(existing) = merged.0.get(key) {
                if existing != value {
                    return None; // Conflict detected
                }
            } else {
                merged.0.insert(key.clone(), value.clone()); // Correctly update the HashMap
            }
        }
        Some(merged)
    }

    pub fn get(&self, var: &str) -> Option<&Term> {
        self.0.get(var) // Access the internal map safely
    }
}

pub fn unify(term1: &Term, term2: &Term, subst: &mut Substitution) -> bool {
    if term1 == term2 { return true } // Stop immediately if the terms are already equal
    match (term1, term2) {
        (Term::Variable(x), t) | (t, Term::Variable(x)) => {
            if t != term1 && occurs_check(x, t) { return false }
            if let Some(existing) = subst.get(x) {
                if existing != t { return false } // Stops an overwrite if 'x' is already bound
            }
            subst.extend(x.clone(), t.clone()); // Variable unification
            return true;
        }
        (Term::Constant(a), Term::Constant(b)) => a == b, // Constant unification
        (Term::Integer(a), Term::Integer(b)) => a == b, // Integer unification
        (Term::Compound(name1, args1), 
        Term::Compound(name2, args2)) => {
            name1 == name2 && unify_lists(args1, args2, subst)
        }
        (Term::List(head1, tail1), 
        Term::List(head2, tail2)) => {
            unify(head1, head2, subst) && unify(tail1, tail2, subst)
        }
        (Term::EmptyList, Term::EmptyList) => true, // Empty lists are equal
        _ => false, // Mismatched structures
    }
}

fn unify_lists(list1: &[Term], list2: &[Term], subst: &mut Substitution) -> bool {
    if list1.len() != list2.len() { return false }
    list1.iter().zip(list2.iter()).all(|(t1, t2)| unify(t1, t2, subst))
}

fn occurs_check(var: &str, term: &Term) -> bool {
    match term {
        Term::Variable(v) => v == var,
        Term::Compound(_, args) => args.iter().any(|t| occurs_check(var, t)),
        Term::List(head, tail) => occurs_check(var, head) || occurs_check(var, tail),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::Term;

    #[test]
    fn test_extend_and_get() {
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(5));
        assert_eq!(subs.get("X"), Some(&Term::Integer(5)));
    }

    #[test]
    fn test_resolve_simple_variable() {
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(3));
        let resolved = subs.resolve(&Term::Variable("X".to_string()));
        assert_eq!(resolved, Term::Integer(3));
    }

    #[test]
    fn test_resolve_nested_variables() {
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Variable("Y".to_string()));
        subs.extend("Y".to_string(), Term::Integer(7));
        let resolved = subs.resolve(&Term::Variable("X".to_string()));
        assert_eq!(resolved, Term::Integer(7));
    }

    #[test]
    fn test_apply_to_compound() {
        let mut subs = Substitution::new();
        subs.extend("X".to_string(), Term::Integer(2));
        let term = Term::Compound("f".to_string(), vec![Term::Variable("X".to_string())]);
        let applied = subs.apply(&term);
        assert_eq!(applied, Term::Compound("f".to_string(), vec![Term::Integer(2)]));
    }

    #[test]
    fn test_merge_successful() {
        let mut subs1 = Substitution::new();
        subs1.extend("X".to_string(), Term::Integer(5));
        let mut subs2 = Substitution::new();
        subs2.extend("Y".to_string(), Term::Integer(10));

        let merged = subs1.merge(&subs2).unwrap();
        assert_eq!(merged.get("X"), Some(&Term::Integer(5)));
        assert_eq!(merged.get("Y"), Some(&Term::Integer(10)));
    }

    #[test]
    fn test_merge_conflict() {
        let mut subs1 = Substitution::new();
        subs1.extend("X".to_string(), Term::Integer(5));
        let mut subs2 = Substitution::new();
        subs2.extend("X".to_string(), Term::Integer(6));

        assert!(subs1.merge(&subs2).is_none());
    }

    #[test]
    fn test_unify_constants() {
        let mut subs = Substitution::new();
        let t1 = Term::Constant("a".to_string());
        let t2 = Term::Constant("a".to_string());
        assert!(unify(&t1, &t2, &mut subs));
    }

    #[test]
    fn test_unify_variable_with_constant() {
        let mut subs = Substitution::new();
        let t1 = Term::Variable("X".to_string());
        let t2 = Term::Constant("a".to_string());
        assert!(unify(&t1, &t2, &mut subs));
        assert_eq!(subs.get("X"), Some(&Term::Constant("a".to_string())));
    }

    #[test]
    fn test_unify_compound_terms() {
        let mut subs = Substitution::new();
        let t1 = Term::Compound("f".to_string(), vec![Term::Variable("X".to_string()), Term::Integer(2)]);
        let t2 = Term::Compound("f".to_string(), vec![Term::Integer(1), Term::Variable("Y".to_string())]);
        assert!(unify(&t1, &t2, &mut subs));
        assert_eq!(subs.get("X"), Some(&Term::Integer(1)));
        assert_eq!(subs.get("Y"), Some(&Term::Integer(2)));
    }

    #[test]
    fn test_unify_lists() {
        let mut subs = Substitution::new();
        let t1 = Term::List(
            Box::new(Term::Variable("X".to_string())),
            Box::new(Term::EmptyList),
        );
        let t2 = Term::List(
            Box::new(Term::Integer(5)),
            Box::new(Term::EmptyList),
        );
        assert!(unify(&t1, &t2, &mut subs));
        assert_eq!(subs.get("X"), Some(&Term::Integer(5)));
    }

    #[test]
    fn test_unify_fails_on_different_functor_names() {
        let mut subs = Substitution::new();
        let t1 = Term::Compound("f".to_string(), vec![Term::Integer(1)]);
        let t2 = Term::Compound("g".to_string(), vec![Term::Integer(1)]);
        assert!(!unify(&t1, &t2, &mut subs));
    }

    #[test]
    fn test_occurs_check_blocks_recursive_binding() {
        let mut subs = Substitution::new();
        let t1 = Term::Variable("X".to_string());
        let t2 = Term::Compound("f".to_string(), vec![Term::Variable("X".to_string())]);
        assert!(!unify(&t1, &t2, &mut subs)); // Should fail due to occurs check
    }
}
