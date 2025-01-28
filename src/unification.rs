use crate::term::Term;
use crate::environment::Environment;

impl Environment {
    pub fn unify(&mut self, term1: &Term, term2: &Term) -> Result<(), String> {
        match (term1, term2) {
            (Term::Constant(a), Term::Constant(b)) => {
                if a == b {
                    Ok(())
                } else {
                    Err(format!("Failed to unify {} and {}", a, b))
                }
            }
            (Term::Variable(v), t) | (t, Term::Variable(v)) => {
                self.bind(v.clone(), t.clone());
                Ok(())
            }
            (Term::Compound { name: name1, args: args1 },
             Term::Compound { name: name2, args: args2 }) => {
                if name1 == name2 && args1.len() == args2.len() {
                    for (arg1, arg2) in args1.iter().zip(args2) {
                        self.unify(arg1, arg2)?;
                    }
                    Ok(())
                } else {
                    Err(format!(
                        "Failed to unify compound terms {} and {}",
                        name1, name2
                    ))
                }
            }
            _ => Err(format!("Failed to unify {:?} and {:?}", term1, term2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unify_constants() {
        let mut env = Environment::new();
        assert!(env.unify(&Term::Constant("a".into()), &Term::Constant("a".into())).is_ok());
        assert!(env.unify(&Term::Constant("a".into()), &Term::Constant("b".into())).is_err());
    }

    #[test]
    fn test_unify_variables() {
        let mut env = Environment::new();
        assert!(env.unify(&Term::Variable("X".into()), &Term::Constant("a".into())).is_ok());
        assert_eq!(
            env.lookup(&"X".into()),
            Some(&Term::Constant("a".into()))
        );
    }

    #[test]
    fn test_unify_compound_terms() {
        let mut env = Environment::new();
        let t1 = Term::Compound {
            name: "parent".into(),
            args: vec![Term::Variable("X".into()), Term::Constant("mary".into())],
        };
        let t2 = Term::Compound {
            name: "parent".into(),
            args: vec![Term::Constant("john".into()), Term::Constant("mary".into())],
        };
        assert!(env.unify(&t1, &t2).is_ok());
    }
}
