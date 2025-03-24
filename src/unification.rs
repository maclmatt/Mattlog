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
