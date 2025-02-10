use std::collections::HashMap;
use crate::terms::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Substitution(HashMap<String, Term>);

impl Substitution {
    pub fn new() -> Self {
        Substitution(HashMap::new())
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
    
            Term::Conjunct(left, right) => {
                let new_left = Box::new(self.apply(left));
                let new_right = Box::new(self.apply(right));
                Term::Conjunct(new_left, new_right)
            }
        }
    }
    
    

    pub fn extend(&mut self, var: String, term: Term) {
        self.0.insert(var, term);
    }

    pub fn allow_merge(&mut self, other: &Substitution) {
        self.0.extend(other.0.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

}

pub fn unify(term1: &Term, term2: &Term, subst: &mut Substitution) -> bool {
    match (term1, term2) {
        (Term::Variable(x), t) | (t, Term::Variable(x)) => {
            // Variable unification with occurs check
            if t != term1 && occurs_check(x, t) {
                false
            } else {
                subst.extend(x.clone(), t.clone());
                true
            }
        }
        (Term::Constant(a), Term::Constant(b)) => a == b, // Constant unification
        (Term::Integer(a), Term::Integer(b)) => a == b, // Integer unification
        (Term::Compound(name1, args1), Term::Compound(name2, args2)) => {
            name1 == name2 && unify_lists(args1, args2, subst)
        }
        (Term::List(head1, tail1), Term::List(head2, tail2)) => {
            unify(head1, head2, subst) && unify(tail1, tail2, subst)
        }
        (Term::EmptyList, Term::EmptyList) => true, // Empty lists are equal
        _ => false, // Mismatched structures
    }
}

fn unify_lists(list1: &[Term], list2: &[Term], subst: &mut Substitution) -> bool {
    if list1.len() != list2.len() {
        return false;
    }
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
